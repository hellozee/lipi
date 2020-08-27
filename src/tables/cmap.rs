use crate::reader;
use crate::tables::offset::OffsetTable;
mod processor;

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
    // TODO: No idea about Format13.0 and Format14.0, will hack them another day not now
}

#[derive(Debug, Clone)]
pub struct Cmap {
    pub index: CmapIndex,
    pub encodings: Vec<CmapEncoding>,
    pub format_table: CmapFormatTable,
}

pub fn read(r: &mut reader::FontReader, cmap_offset_table: OffsetTable) -> Option<Cmap> {
    let _ = r.seek(cmap_offset_table.offset as usize);

    let version = r.get_uint16()?;
    let subtable_count = r.get_uint16()?;

    let index = CmapIndex {
        version,
        subtable_count,
    };

    let encodings = processor::cmap_encoding_tables(r, subtable_count)?;
    let format_table = processor::cmap_format_table(r)?;

    return Some(Cmap {
        index,
        encodings,
        format_table,
    });
}
