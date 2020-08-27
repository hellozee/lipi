use crate::reader;
use crate::tables::offset::OffsetTable;

#[derive(Debug, Copy, Clone)]
pub struct Hhea {
    pub version: f32,                 // 0x00010000 (1.0)
    pub ascent: i16,                  // Distance from baseline of highest ascender
    pub descent: i16,                 // Distance from baseline of lowest descender
    pub line_gap: i16,                // typographic line gap
    pub advance_width_max: u16,       // must be consistent with horizontal metrics
    pub min_left_side_bearing: i16,   // must be consistent with horizontal metrics
    pub min_right_side_bearing: i16,  // must be consistent with horizontal metrics
    pub x_max_extent: i16,            // max(lsb + (xMax-xMin))
    pub caret_slope_rise: i16, // used to calculate the slope of the caret (rise/run) set to 1 for vertical caret
    pub caret_slope_run: i16,  // 0 for vertical
    pub caret_offset: i16,     // set value to 0 for non-slanted fonts
    pub metric_data_format: i16, // 0 for current format
    pub num_of_long_hor_metrics: u16, // number of advance widths in metrics table
}

pub fn read(r: &mut reader::FontReader, hhea_offset_table: OffsetTable) -> Option<Hhea> {
    let _ = r.seek(hhea_offset_table.offset as usize);

    let version = r.get_float32()?;
    let ascent = r.get_int16()?;
    let descent = r.get_int16()?;
    let line_gap = r.get_int16()?;
    let advance_width_max = r.get_uint16()?;
    let min_left_side_bearing = r.get_int16()?;
    let min_right_side_bearing = r.get_int16()?;
    let x_max_extent = r.get_int16()?;
    let caret_slope_rise = r.get_int16()?;
    let caret_slope_run = r.get_int16()?;
    let caret_offset = r.get_int16()?;

    // these are all reserved bits, god knows why?
    for _ in 0..4 {
        let _ = r.get_int16()?;
    }

    let metric_data_format = r.get_int16()?;
    let num_of_long_hor_metrics = r.get_uint16()?;

    return Some(Hhea {
        version,
        ascent,
        descent,
        line_gap,
        advance_width_max,
        min_left_side_bearing,
        min_right_side_bearing,
        x_max_extent,
        caret_slope_rise,
        caret_slope_run,
        caret_offset,
        metric_data_format,
        num_of_long_hor_metrics,
    });
}
