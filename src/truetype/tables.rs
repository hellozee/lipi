use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone)]
pub struct OffsetSubTable {
    pub scalar_type: u32, // A tag to indicate the OFA scaler to be used to rasterize this font
    pub numtables: u16,   // number of tables
    pub search_range: u16, // (maximum power of 2 <= numTables)*16
    pub entry_selector: u16, // log2(maximum power of 2 <= numTables)
    pub range_shift: u16, // numTables*16-searchRange
}

#[derive(Debug, Copy, Clone)]
pub struct OffsetTable {
    pub checksum: u32, // checksum for this table
    pub offset: u32,   // offset from beginning of sfnt
    pub length: u32,   // length of this table in byte (actual length not padded length)
}

#[derive(Debug, Copy, Clone)]
pub struct Head {
    pub version: f32,             // 0x00010000 if (version 1.0)
    pub font_revision: f32,       // set by font manufacturer
    pub checksum_adjustment: u32, // separate checksum for head
    pub magic_number: u32,        // set to 0x5F0F3CF5
    pub flags: u16,               // afaik has a total of 14 flags
    pub units_per_em: u16,        // range from 64 to 16384
    pub created: NaiveDateTime,   // international date
    pub modified: NaiveDateTime,  // international date
    pub xmin: i16,                // for all glyph bounding boxes
    pub ymin: i16,                // for all glyph bounding boxes
    pub xmax: i16,                // for all glyph bounding boxes
    pub ymax: i16,                // for all glyph bounding boxes
    pub mac_style: u16,           // bold | italic | underline | outline | shadow | condensed
    pub lowest_rec_ppem: u16,     // smallest readable size in pixels
    pub font_direction_hint: i16, // 0 | 1 | 2 | -1 | -2
    pub index_to_loc_format: i16, // 0 for short offsets, 1 for long
    pub glyph_data_format: i16,   // 0 for current format
}
