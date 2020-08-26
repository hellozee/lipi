use crate::truetype::processor;
use crate::truetype::tables;
use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FontReader {
    pos: usize,
    data: Vec<u8>,
}

impl FontReader {
    pub fn new(filename: String) -> Self {
        return FontReader {
            pos: 0,
            data: match std::fs::read(filename) {
                Ok(val) => val,
                Err(_) => panic!("Can't read the file!!"),
            },
        };
    }

    pub fn seek(&mut self, pos: usize) -> usize {
        if pos >= self.data.len() {
            panic!("Reached the end of file");
        }

        let oldpos = self.pos;
        self.pos = pos;
        return oldpos;
    }

    pub fn get_uint8(&mut self) -> Option<u8> {
        let byte = self.data.get(self.pos)?.clone();
        self.pos += 1;
        return Some(byte);
    }

    pub fn get_uint16(&mut self) -> Option<u16> {
        let byte1 = self.get_uint8()?;
        let byte2 = self.get_uint8()?;
        return Some((byte1 as u16) << 8 | byte2 as u16);
    }

    pub fn get_uint32(&mut self) -> Option<u32> {
        let byte1 = self.get_uint16()?;
        let byte2 = self.get_uint16()?;
        return Some((byte1 as u32) << 16 | byte2 as u32);
    }

    pub fn get_int16(&mut self) -> Option<i16> {
        Some(self.get_uint16()? as i16)
    }

    pub fn get_int32(&mut self) -> Option<i32> {
        Some(self.get_uint32()? as i32)
    }

    pub fn get_float32(&mut self) -> Option<f32> {
        Some(self.get_int32()? as f32 / (1 << 16) as f32)
    }

    pub fn get_string(&mut self, length: usize) -> Option<String> {
        let mut result = String::new();

        for _ in 0..length {
            result.push(self.get_uint8()? as char);
        }

        return Some(result);
    }

    pub fn get_date(&mut self) -> Option<NaiveDateTime> {
        let unix_hfs_epoch_diff = 208284480;
        let time =
            ((self.get_uint32()? as u64) << 32 | self.get_uint32()? as u64) - unix_hfs_epoch_diff;
        let date = NaiveDateTime::from_timestamp(time as i64, 0);
        return Some(date);
    }

    pub fn read_offset_subtable(&mut self) -> Option<tables::OffsetSubTable> {
        let scalar_type = self.get_uint32()?;
        let numtables = self.get_uint16()?;
        let search_range = self.get_uint16()?;
        let entry_selector = self.get_uint16()?;
        let range_shift = self.get_uint16()?;

        return Some(tables::OffsetSubTable {
            scalar_type,
            numtables,
            search_range,
            entry_selector,
            range_shift,
        });
    }

    pub fn read_offset_tables(
        &mut self,
        numtables: u16,
    ) -> Option<HashMap<String, tables::OffsetTable>> {
        let mut offset_tables = HashMap::new();
        for _ in 0..numtables {
            let tag = self.get_string(4)?;
            let table = tables::OffsetTable {
                checksum: self.get_uint32()?,
                offset: self.get_uint32()?,
                length: self.get_uint32()?,
            };

            offset_tables.insert(tag.clone(), table);

            if tag != "head"
                && processor::table_cs(self, table.offset, table.length) != table.checksum
            {
                println!("Checksums don't match for the {} table", tag);
                return None;
            }
        }

        Some(offset_tables)
    }

    pub fn read_head(&mut self, head_offset_table: tables::OffsetTable) -> Option<tables::Head> {
        let _ = self.seek(head_offset_table.offset as usize);

        let version = self.get_float32()?;
        let font_revision = self.get_float32()?;
        let checksum_adjustment = self.get_uint32()?;
        let magic_number = self.get_uint32()?;
        if magic_number != 0x5f0f3cf5 {
            println!("Wrong magic number is head table");
            return None;
        }
        let flags = self.get_uint16()?;
        let units_per_em = self.get_uint16()?;
        let created = self.get_date()?;
        let modified = self.get_date()?;
        let xmin = self.get_int16()?;
        let ymin = self.get_int16()?;
        let xmax = self.get_int16()?;
        let ymax = self.get_int16()?;
        let mac_style = self.get_uint16()?;
        let lowest_rec_ppem = self.get_uint16()?;
        let font_direction_hint = self.get_int16()?;
        let index_to_loc_format = self.get_int16()?;
        let glyph_data_format = self.get_int16()?;

        return Some(tables::Head {
            version,
            font_revision,
            checksum_adjustment,
            magic_number,
            flags,
            units_per_em,
            created,
            modified,
            xmin,
            ymin,
            xmax,
            ymax,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format,
        });
    }

    pub fn read_maxp(&mut self, maxp_offset_table: tables::OffsetTable) -> Option<tables::Maxp> {
        let _ = self.seek(maxp_offset_table.offset as usize);

        return Some(tables::Maxp {
            version: self.get_float32()?,
            glyph_count: self.get_uint16()?,
            max_points: self.get_uint16()?,
            max_contours: self.get_uint16()?,
            max_component_points: self.get_uint16()?,
            max_component_contours: self.get_uint16()?,
            max_zones: self.get_uint16()?,
            max_twilight_points: self.get_uint16()?,
            max_storage: self.get_uint16()?,
            max_function_defs: self.get_uint16()?,
            max_instruction_defs: self.get_uint16()?,
            max_stack_elements: self.get_uint16()?,
            max_size_of_instructions: self.get_uint16()?,
            max_component_elements: self.get_uint16()?,
            max_component_depth: self.get_uint16()?,
        });
    }

    pub fn read_cmap(&mut self, cmap_offset_table: tables::OffsetTable) -> Option<tables::Cmap> {
        let _ = self.seek(cmap_offset_table.offset as usize);

        let version = self.get_uint16()?;
        let subtable_count = self.get_uint16()?;

        let index = tables::CmapIndex {
            version,
            subtable_count,
        };

        let encodings = processor::cmap_encoding_tables(self, subtable_count)?;
        let format_table = processor::cmap_format_table(self)?;

        return Some(tables::Cmap {
            index,
            encodings,
            format_table,
        });
    }
}
