mod processor;
mod reader;
mod tables;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TrueTypeFont {
    file: reader::FontReader,
    offset_sub_table: tables::OffsetSubTable,
    offset_tables: HashMap<String, tables::OffsetTable>,
    head: tables::Head,
    maxp: tables::Maxp,
    cmap: tables::Cmap,
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
        let cmap = file.read_cmap(*offset_tables.get("cmap")?)?;

        return Some(TrueTypeFont {
            file,
            offset_sub_table,
            offset_tables,
            head,
            maxp,
            cmap,
        });
    }
}
