use crate::reader;
use crate::tables;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TrueTypeFont {
    file: reader::FontReader,
    offset_sub_table: tables::offset_sub::OffsetSubTable,
    offset_tables: HashMap<String, tables::offset::OffsetTable>,
    head: tables::head::Head,
    maxp: tables::maxp::Maxp,
    cmap: tables::cmap::Cmap,
    hhea: tables::hhea::Hhea,
    hmtx: tables::hmtx::Hmtx,
    loca: tables::loca::Loca,
}

impl TrueTypeFont {
    pub fn new(filename: String) -> Option<Self> {
        let mut file = reader::FontReader::new(filename);

        let offset_sub_table = tables::offset_sub::read(&mut file)?;
        let offset_tables = tables::offset::read(&mut file, offset_sub_table.numtables)?;
        let head = tables::head::read(&mut file, *offset_tables.get("head")?)?;
        let maxp = tables::maxp::read(&mut file, *offset_tables.get("maxp")?)?;
        let cmap = tables::cmap::read(&mut file, *offset_tables.get("cmap")?)?;
        let hhea = tables::hhea::read(&mut file, *offset_tables.get("hhea")?)?;
        let hmtx = tables::hmtx::read(
            &mut file,
            *offset_tables.get("hmtx")?,
            hhea.num_of_long_hor_metrics,
            maxp.glyph_count,
        )?;

        let loca = tables::loca::read(
            &mut file,
            *offset_tables.get("loca")?,
            maxp.glyph_count,
            head.index_to_loc_format,
        )?;

        return Some(TrueTypeFont {
            file,
            offset_sub_table,
            offset_tables,
            head,
            maxp,
            cmap,
            hhea,
            hmtx,
            loca,
        });
    }
}
