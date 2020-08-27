use crate::reader;
use crate::tables::offset::OffsetTable;
use chrono::NaiveDateTime;

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

pub fn read(r: &mut reader::FontReader, head_offset_table: OffsetTable) -> Option<Head> {
    let _ = r.seek(head_offset_table.offset as usize);

    let version = r.get_float32()?;
    let font_revision = r.get_float32()?;
    let checksum_adjustment = r.get_uint32()?;
    let magic_number = r.get_uint32()?;
    if magic_number != 0x5f0f3cf5 {
        println!("Wrong magic number is head table");
        return None;
    }
    let flags = r.get_uint16()?;
    let units_per_em = r.get_uint16()?;
    let created = r.get_date()?;
    let modified = r.get_date()?;
    let xmin = r.get_int16()?;
    let ymin = r.get_int16()?;
    let xmax = r.get_int16()?;
    let ymax = r.get_int16()?;
    let mac_style = r.get_uint16()?;
    let lowest_rec_ppem = r.get_uint16()?;
    let font_direction_hint = r.get_int16()?;
    let index_to_loc_format = r.get_int16()?;
    let glyph_data_format = r.get_int16()?;

    return Some(Head {
        version,
        font_revision,
        checksum_adjustment,
        magic_number,
        flags,
        units_per_em,
        created,
        modified,
        xmin,
        ymin,
        xmax,
        ymax,
        mac_style,
        lowest_rec_ppem,
        font_direction_hint,
        index_to_loc_format,
        glyph_data_format,
    });
}
