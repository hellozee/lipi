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
    maxp: tables::Maxp,
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
        let maxp = file.read_maxp(*offset_tables.get("maxp")?)?;

        return Some(TrueTypeFont {
            file,
            offset_sub_table,
            offset_tables,
            head,
            maxp,
        });
    }
}
