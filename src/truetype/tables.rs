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

#[derive(Debug, Copy, Clone)]
pub struct Maxp {
    pub version: f32,                  // 0x00010000 (1.0)
    pub glyph_count: u16,              // the number of glyphs in the font
    pub max_points: u16,               // points in non-compound glyph
    pub max_contours: u16,             // contours in non-compound glyph
    pub max_component_points: u16,     // points in compound glyph
    pub max_component_contours: u16,   // contours in compound glyph
    pub max_zones: u16,                // set to 2
    pub max_twilight_points: u16,      // points used in Twilight Zone (Z0)
    pub max_storage: u16,              // number of Storage Area locations
    pub max_function_defs: u16,        // number of FDEFs
    pub max_instruction_defs: u16,     // number of IDEFs
    pub max_stack_elements: u16,       // maximum stack depth
    pub max_size_of_instructions: u16, // maximum stack depth
    pub max_component_elements: u16,   // number of glyphs referenced at top level
    pub max_component_depth: u16, // levels of recursion, set to 0 if font has only simple glyphs
}
