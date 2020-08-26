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

#[derive(Debug, Copy, Clone)]
pub enum CmapPlatform {
    Unicode,   // Indicates Unicode version
    Macintosh, // Script Manager code.
    Microsoft, // Microsoft encoding.
}

#[derive(Debug, Copy, Clone)]
pub struct CmapIndex {
    pub version: u16,        // Version number (Set to zero)
    pub subtable_count: u16, // Number of encoding subtables
}

#[derive(Debug, Copy, Clone)]
pub struct CmapEncoding {
    pub platform_id: CmapPlatform, // Platform identifier
    pub platform_specific_id: u16, // Platform-specific encoding identifier
    pub offset: u32,               // Offset of the mapping table
}

#[derive(Debug, Copy, Clone)]
pub struct CmapFormat0 {
    pub format: u16,                  // Set to 0
    pub length: u16,                  // Length in bytes of the subtable (set to 262 for format 0)
    pub language_code: u16,           // Language code
    pub glyph_index_array: [u8; 256], // An array that maps character codes to glyph index values
}

#[derive(Debug, Copy, Clone)]
pub struct CmapFormat2 {
    pub format: u16,
    pub length: u16,
    pub language_code: u16,
    pub sub_header_keys: [u16; 256], // Array that maps high bytes to subHeaders: value is index * 8
}

#[derive(Debug, Copy, Clone)]
pub struct CmapFormat4Segment {
    pub id_range_offset: u16, // Offset in bytes to glyph indexArray, or 0
    pub start_code: u16,      // Starting character code for each segment
    pub end_code: u16,        // Ending character code for each segment, last = 0xFFFF.
    pub id_delta: u16,        // Delta for all character codes in segment
}

#[derive(Debug, Clone)]
pub struct CmapFormat4 {
    pub format: u16,
    pub length: u16,
    pub language_code: u16,
    pub segcount_x2: u16,    // 2 * segCount
    pub search_range: u16,   // 2 * (2**FLOOR(log2(segCount)))
    pub entry_selector: u16, // log2(searchRange/2)
    pub range_shift: u16,    // (2 * segCount) - searchRange
    pub segments: Vec<CmapFormat4Segment>,
}

#[derive(Debug, Clone)]
pub struct CmapFormat6 {
    pub format: u16,
    pub length: u16,
    pub language_code: u16,
    pub first_code: u16,             // First character code of subrange
    pub entry_count: u16,            // Number of character codes in subrange
    pub glyph_index_array: Vec<u16>, // Array of glyph index values for character codes in the range
}

#[derive(Debug, Copy, Clone)]
pub struct CmapFormat80Group {
    pub start_char_code: u32,  // First character code in this group
    pub end_char_code: u32, // Last character code in this group; same condition as listed above for the startCharCode
    pub start_glyph_code: u32, // Glyph index corresponding to the starting character code
}

#[derive(Debug, Clone)]
pub struct CmapFormat80 {
    pub format: f32,
    pub length: u32,
    pub language_code: u32,
    pub is_32: [u8; 8192], // Tightly packed array of bits (8K bytes total) indicating whether the particular 16-bit (index) value is the start of a 32-bit character code
    pub n_groups: u32,     // Number of groupings which follow
    pub groups: Vec<CmapFormat80Group>,
}

#[derive(Debug, Clone)]
pub struct CmapFormat100 {
    pub format: f32,
    pub length: u32,
    pub language_code: u32,
    pub start_char_code: u32, // First character code covered
    pub num_chars: u32,       // Number of character codes covered
    pub glyphs: Vec<u16>,     // Array of glyph indices for the character codes covered
}

pub type CmapFormat120Group = CmapFormat80Group;

#[derive(Debug, Clone)]
pub struct CmapFormat120 {
    pub format: f32,
    pub length: u32,
    pub language_code: u32,
    pub n_groups: u32, // Number of groupings which follow
    pub groups: Vec<CmapFormat120Group>,
}

#[derive(Debug, Clone)]
pub enum CmapFormatTable {
    Format0(CmapFormat0),
    Format2(CmapFormat2),
    Format4(CmapFormat4),
    Format6(CmapFormat6),
    Format80(CmapFormat80),
    Format100(CmapFormat100),
    Format120(CmapFormat120),
    // No idea about Format13.0 and Format14.0, will hack them another day not now
}

#[derive(Debug, Clone)]
pub struct Cmap {
    pub index: CmapIndex,
    pub encodings: Vec<CmapEncoding>,
    pub format_table: CmapFormatTable,
}
