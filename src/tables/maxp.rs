use crate::reader;
use crate::tables::offset::OffsetTable;

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

pub fn read(r: &mut reader::FontReader, maxp_offset_table: OffsetTable) -> Option<Maxp> {
    let _ = r.seek(maxp_offset_table.offset as usize);

    return Some(Maxp {
        version: r.get_float32()?,
        glyph_count: r.get_uint16()?,
        max_points: r.get_uint16()?,
        max_contours: r.get_uint16()?,
        max_component_points: r.get_uint16()?,
        max_component_contours: r.get_uint16()?,
        max_zones: r.get_uint16()?,
        max_twilight_points: r.get_uint16()?,
        max_storage: r.get_uint16()?,
        max_function_defs: r.get_uint16()?,
        max_instruction_defs: r.get_uint16()?,
        max_stack_elements: r.get_uint16()?,
        max_size_of_instructions: r.get_uint16()?,
        max_component_elements: r.get_uint16()?,
        max_component_depth: r.get_uint16()?,
    });
}
