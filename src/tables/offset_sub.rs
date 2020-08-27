use crate::reader;

#[derive(Debug, Copy, Clone)]
pub struct OffsetSubTable {
    pub scalar_type: u32, // A tag to indicate the OFA scaler to be used to rasterize this font
    pub numtables: u16,   // number of tables
    pub search_range: u16, // (maximum power of 2 <= numTables)*16
    pub entry_selector: u16, // log2(maximum power of 2 <= numTables)
    pub range_shift: u16, // numTables*16-searchRange
}

pub fn read(r: &mut reader::FontReader) -> Option<OffsetSubTable> {
    let scalar_type = r.get_uint32()?;
    let numtables = r.get_uint16()?;
    let search_range = r.get_uint16()?;
    let entry_selector = r.get_uint16()?;
    let range_shift = r.get_uint16()?;

    return Some(OffsetSubTable {
        scalar_type,
        numtables,
        search_range,
        entry_selector,
        range_shift,
    });
}
