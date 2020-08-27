use crate::reader;
use crate::tables::offset::OffsetTable;

#[derive(Debug, Clone)]
pub enum Loca {
    Short(Vec<u16>),
    Long(Vec<u32>),
}

pub fn read(
    r: &mut reader::FontReader,
    loca_offset_table: OffsetTable,
    glyph_count: u16,
    index_to_loc_format: i16,
) -> Option<Loca> {
    let _ = r.seek(loca_offset_table.offset as usize);

    match index_to_loc_format {
        0 => {
            let mut offsets = Vec::new();
            for _ in 0..(glyph_count + 1) {
                offsets.push(r.get_uint16()?);
            }

            return Some(Loca::Short(offsets));
        }
        1 => {
            let mut offsets = Vec::new();
            for _ in 0..(glyph_count + 1) {
                offsets.push(r.get_uint32()?);
            }

            return Some(Loca::Long(offsets));
        }
        val => {
            println!("{} is not a supported format for the loca table.", val);
            return None;
        }
    }
}
