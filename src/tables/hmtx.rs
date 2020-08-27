use crate::reader;
use crate::tables::offset::OffsetTable;

// Not much of documentation around the hmtx table, simple to grasp anyway
#[derive(Debug, Copy, Clone)]
pub struct HmtxLongHorMetric {
    pub advance_width: u16,
    pub left_side_bearing: i16,
}

#[derive(Debug, Clone)]
pub struct Hmtx {
    pub hmetrics: Vec<HmtxLongHorMetric>,
    pub left_side_bearings: Vec<i16>,
}

pub fn read(
    r: &mut reader::FontReader,
    hmtx_offset_table: OffsetTable,
    long_hor_metric_count: u16,
    glyph_count: u16,
) -> Option<Hmtx> {
    let _ = r.seek(hmtx_offset_table.offset as usize);
    let mut hmetrics = Vec::new();

    for _ in 0..long_hor_metric_count {
        hmetrics.push(HmtxLongHorMetric {
            advance_width: r.get_uint16()?,
            left_side_bearing: r.get_int16()?,
        });
    }

    let mut left_side_bearings = Vec::new();

    for _ in 0..(glyph_count - long_hor_metric_count) {
        left_side_bearings.push(r.get_int16()?);
    }

    return Some(Hmtx {
        hmetrics,
        left_side_bearings,
    });
}
