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
    hhea: tables::Hhea,
}

impl TrueTypeFont {
    pub fn new(filename: String) -> Option<Self> {
        let mut file = reader::FontReader::new(filename);

        let offset_sub_table = file.read_offset_subtable()?;
        let offset_tables = file.read_offset_tables(offset_sub_table.numtables)?;
        let head = file.read_head(*offset_tables.get("head")?)?;
        let maxp = file.read_maxp(*offset_tables.get("maxp")?)?;
        let cmap = file.read_cmap(*offset_tables.get("cmap")?)?;
        let hhea = file.read_hhea(*offset_tables.get("hhea")?)?;

        return Some(TrueTypeFont {
            file,
            offset_sub_table,
            offset_tables,
            head,
            maxp,
            cmap,
            hhea,
        });
    }
}
