mod reader;
mod tables;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    countour_count: i16,
    xmin: i16,
    ymin: i16,
    xmax: i16,
    ymax: i16,
}

#[derive(Debug)]
pub struct TrueTypeFont {
    file: reader::FontReader,
    offset_sub_table: tables::OffsetSubTable,
    offset_tables: HashMap<String, tables::OffsetTable>,
    head: tables::Head,
}

impl TrueTypeFont {
    pub fn new(filename: String) -> Option<Self> {
        let mut file = match reader::FontReader::new(filename) {
            Ok(reader) => reader,
            Err(_) => panic!("File not found!!"),
        };

        let offset_sub_table = file.read_offset_subtable()?;
        let offset_tables = file.read_offset_tables(offset_sub_table.numtables)?;
        let head = file.read_head(*offset_tables.get("head")?)?;

        return Some(TrueTypeFont {
            file,
            offset_sub_table,
            offset_tables,
            head,
        });
    }

    fn glyph_count(&mut self) -> Option<()> {
        if !self.offset_tables.contains_key("head".into()) {
            return None;
        }

        let maxp_table = self.offset_tables.get("maxp".into())?;
        let old = match self.file.seek(maxp_table.offset as usize + 4) {
            Ok(val) => val,
            Err(_) => return None,
        };
        //self.length = self.file.get_uint16()?;
        let _ = self.file.seek(old);

        Some(())
    }

    fn get_glyph_offset(&mut self, index: u32) -> Option<u32> {
        if !self.offset_tables.contains_key("head".into()) {
            return None;
        }
        let table = self.offset_tables.get("loca".into());

        if self.head.index_to_loc_format == 1 {
            let old = match self.file.seek((table?.offset + index * 4) as usize) {
                Ok(val) => val,
                Err(_) => return None,
            };
            let offset = self.file.get_uint32()?;
            let _ = match self.file.seek(old) {
                Ok(val) => val,
                Err(_) => return None,
            };
            return Some(offset + self.offset_tables.get("glyf".into())?.offset);
        } else {
            let old = match self.file.seek((table?.offset + index * 2) as usize) {
                Ok(val) => val,
                Err(_) => return None,
            };
            let offset = self.file.get_uint16()? * 2;
            let _ = match self.file.seek(old) {
                Ok(val) => val,
                Err(_) => return None,
            };
            return Some(offset as u32 + self.offset_tables.get("glyf".into())?.offset);
        }
    }

    pub fn read_glyph(&mut self, index: u32) -> Option<Glyph> {
        let offset = self.get_glyph_offset(index)?;
        let table = self.offset_tables.get("glyf".into())?;
        if offset >= table.offset + table.length {
            return None;
        }

        let _ = self.file.seek(offset as usize);

        let glyph = Glyph {
            countour_count: self.file.get_int16()?,
            xmin: self.file.get_int16()?,
            ymin: self.file.get_int16()?,
            xmax: self.file.get_int16()?,
            ymax: self.file.get_int16()?,
        };

        if glyph.countour_count < -1 {
            return None;
        }

        return Some(glyph);
    }
}
