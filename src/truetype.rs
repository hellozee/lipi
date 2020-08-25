mod reader;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct OffsetTable {
    checksum: u32,
    offset: u32,
    length: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    countour_count: i16,
    xmin: i16,
    ymin: i16,
    xmax: i16,
    ymax: i16,
}

#[derive(Debug)]
pub struct TrueType {
    file: reader::BinaryReader,
    offset_tables: HashMap<String, OffsetTable>,
    length: u16,
    scalar_type: u32,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    version: f32,
    font_revision: f32,
    checksum_adjustment: u32,
    magic_number: u32,
    flags: u16,
    units_per_em: u16,
    xmin: i16,
    ymin: i16,
    xmax: i16,
    ymax: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16,
}

impl TrueType {
    pub fn new(filename: String) -> Option<Self> {
        let mut tt = TrueType {
            file: match reader::BinaryReader::new(filename) {
                Ok(reader) => reader,
                Err(_) => panic!("File not found!!"),
            },
            offset_tables: HashMap::new(),
            length: 0,
            scalar_type: 0,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            version: 0.0,
            font_revision: 0.0,
            checksum_adjustment: 0,
            magic_number: 0,
            flags: 0,
            units_per_em: 0,
            xmin: 0,
            ymin: 0,
            xmax: 0,
            ymax: 0,
            mac_style: 0,
            lowest_rec_ppem: 0,
            font_direction_hint: 0,
            index_to_loc_format: 0,
            glyph_data_format: 0,
        };
        tt.read_offset_tables()?;
        tt.read_head()?;
        tt.glyph_count()?;

        return Some(tt);
    }
    fn read_offset_tables(&mut self) -> Option<()> {
        self.scalar_type = self.file.get_uint32()?;
        let numtables = self.file.get_uint16()?;
        self.search_range = self.file.get_uint16()?;
        self.entry_selector = self.file.get_uint16()?;
        self.range_shift = self.file.get_uint16()?;

        for _ in 0..numtables {
            let tag = self.file.get_string(4)?;
            let table = OffsetTable {
                checksum: self.file.get_uint32()?,
                offset: self.file.get_uint32()?,
                length: self.file.get_uint32()?,
            };

            self.offset_tables.insert(tag.clone(), table);

            println!("{:#?}", tag.clone());

            if tag != "head"
                && match self.table_cs(table.offset, table.length) {
                    Ok(val) => val,
                    Err(_) => return None,
                } != table.checksum
            {
                return None;
            }
        }

        Some(())
    }

    fn table_cs(&mut self, offset: u32, length: u32) -> Result<u32, String> {
        let old = self.file.seek(offset as usize)?;
        let mut sum = 0;

        for _ in 0..((length + 3) / 4) {
            let temp = match self.file.get_uint32() {
                Some(val) => val,
                None => 0,
            };
            sum = (sum as u64 + temp as u64) as u32;
        }

        self.file.seek(old)?;
        return Ok(sum);
    }

    fn read_head(&mut self) -> Option<()> {
        if !self.offset_tables.contains_key("head".into()) {
            return None;
        }
        match self
            .file
            .seek(self.offset_tables.get("head".into())?.offset as usize)
        {
            Ok(val) => val,
            Err(_) => return None,
        };

        self.version = self.file.get_float32()?;
        self.font_revision = self.file.get_float32()?;
        self.checksum_adjustment = self.file.get_uint32()?;
        self.magic_number = self.file.get_uint32()?;
        if self.magic_number != 0x5f0f3cf5 {
            return None;
        }
        self.flags = self.file.get_uint16()?;
        self.units_per_em = self.file.get_uint16()?;
        //creation and modified dates, no idea how to handle them in rust
        let _ = self.file.get_uint32()? + self.file.get_uint32()?;
        let _ = self.file.get_uint32()? + self.file.get_uint32()?;
        self.xmin = self.file.get_int16()?;
        self.ymin = self.file.get_int16()?;
        self.xmax = self.file.get_int16()?;
        self.ymax = self.file.get_int16()?;
        self.mac_style = self.file.get_uint16()?;
        self.lowest_rec_ppem = self.file.get_uint16()?;
        self.font_direction_hint = self.file.get_int16()?;
        self.index_to_loc_format = self.file.get_int16()?;
        self.glyph_data_format = self.file.get_int16()?;

        Some(())
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
        self.length = self.file.get_uint16()?;
        let _ = self.file.seek(old);

        Some(())
    }

    fn get_glyph_offset(&mut self, index: u32) -> Option<u32> {
        if !self.offset_tables.contains_key("head".into()) {
            return None;
        }
        let table = self.offset_tables.get("loca".into());

        if self.index_to_loc_format == 1 {
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

    pub fn length(&self) -> u16 {
        self.length
    }
}
