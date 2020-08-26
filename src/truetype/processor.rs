use crate::truetype::reader;
use crate::truetype::tables;

pub fn table_cs(r: &mut reader::FontReader, offset: u32, length: u32) -> u32 {
    let old = r.seek(offset as usize);
    let mut sum = 0;

    for _ in 0..((length + 3) / 4) {
        let temp = match r.get_uint32() {
            Some(val) => val,
            None => 0,
        };
        sum = (sum as u64 + temp as u64) as u32;
    }
    r.seek(old);
    return sum;
}

pub fn cmap_encoding_tables(
    r: &mut reader::FontReader,
    count: u16,
) -> Option<Vec<tables::CmapEncoding>> {
    let mut encoding_tables = Vec::new();
    for _ in 0..count {
        let platform_id = match r.get_uint16()? {
            0 => tables::CmapPlatform::Unicode,
            1 => tables::CmapPlatform::Macintosh,
            2 => {
                println!(
                    "2 is not advised to be used a platform id and reserved for specific purpose"
                );
                return None;
            }
            3 => tables::CmapPlatform::Microsoft,
            val => {
                println!("{} is not a valid platform id.", val);
                return None;
            }
        };

        let platform_specific_id = r.get_uint16()?;
        let offset = r.get_uint32()?;

        encoding_tables.push(tables::CmapEncoding {
            platform_id,
            platform_specific_id,
            offset,
        });
    }

    return Some(encoding_tables);
}

pub fn read_cmap_format0(r: &mut reader::FontReader) -> Option<tables::CmapFormat0> {
    let format = 0;
    let length = r.get_uint16()?;
    let language_code = r.get_uint16()?;
    let mut glyph_index_array = [0; 256];
    for i in 0..256 {
        glyph_index_array[i] = r.get_uint8()?;
    }

    return Some(tables::CmapFormat0 {
        format,
        length,
        language_code,
        glyph_index_array,
    });
}

pub fn read_cmap_format2(r: &mut reader::FontReader) -> Option<tables::CmapFormat2> {
    let format = 2;
    let length = r.get_uint16()?;
    let language_code = r.get_uint16()?;
    let mut sub_header_keys = [0; 256];
    for i in 0..256 {
        sub_header_keys[i] = r.get_uint16()?;
    }

    return Some(tables::CmapFormat2 {
        format,
        length,
        language_code,
        sub_header_keys,
    });
}

pub fn read_cmap_format4(r: &mut reader::FontReader) -> Option<tables::CmapFormat4> {
    let format = 4;
    let length = r.get_uint16()?;
    let language_code = r.get_uint16()?;
    let segcount_x2 = r.get_uint16()?;
    let search_range = r.get_uint16()?;
    let entry_selector = r.get_uint16()?;
    let range_shift = r.get_uint16()?;
    let mut segments = Vec::new();

    let segcount = segcount_x2 / 2;

    for _ in 0..segcount {
        segments.push(tables::CmapFormat4Segment {
            id_range_offset: 0,
            start_code: 0,
            end_code: r.get_uint16()?,
            id_delta: 0,
        });
    }

    r.get_uint16()?; // reservedPad

    for i in 0..segcount as usize {
        segments[i].start_code = r.get_uint16()?;
    }

    for i in 0..segcount as usize {
        segments[i].id_delta = r.get_uint16()?;
    }

    for i in 0..segcount as usize {
        segments[i].id_range_offset = r.get_uint16()?;
    }

    return Some(tables::CmapFormat4 {
        format,
        length,
        language_code,
        segcount_x2,
        search_range,
        entry_selector,
        range_shift,
        segments,
    });
}

pub fn read_cmap_format6(r: &mut reader::FontReader) -> Option<tables::CmapFormat6> {
    let format = 6;
    let length = r.get_uint16()?;
    let language_code = r.get_uint16()?;
    let first_code = r.get_uint16()?;
    let entry_count = r.get_uint16()?;
    let mut glyph_index_array = Vec::new();

    for _ in 0..entry_count {
        glyph_index_array.push(r.get_uint16()?);
    }

    return Some(tables::CmapFormat6 {
        format,
        length,
        language_code,
        first_code,
        entry_count,
        glyph_index_array,
    });
}

pub fn read_cmap_format80(r: &mut reader::FontReader) -> Option<tables::CmapFormat80> {
    let format = 8.0;
    r.get_uint16()?; // just for that 32 bit float thing
    let length = r.get_uint32()?;
    let language_code = r.get_uint32()?;
    let mut is_32 = [0; 8192];

    for i in 0..8192 {
        is_32[i] = r.get_uint8()?;
    }

    let n_groups = r.get_uint32()?;
    let mut groups = Vec::new();

    for _ in 0..n_groups {
        groups.push(tables::CmapFormat80Group {
            start_char_code: r.get_uint32()?,
            end_char_code: r.get_uint32()?,
            start_glyph_code: r.get_uint32()?,
        });
    }

    return Some(tables::CmapFormat80 {
        format,
        length,
        language_code,
        is_32,
        n_groups,
        groups,
    });
}

pub fn read_cmap_format100(r: &mut reader::FontReader) -> Option<tables::CmapFormat100> {
    let format = 10.0;
    r.get_uint16()?; // just for that 32 bit float thing
    let length = r.get_uint32()?;
    let language_code = r.get_uint32()?;
    let start_char_code = r.get_uint32()?;
    let num_chars = r.get_uint32()?;
    let remaining_length = length - 20; // 5 * 4 byte entries
    let mut glyphs = Vec::new();
    for _ in 0..(remaining_length / 2) {
        // a glyph is 2 byte
        glyphs.push(r.get_uint16()?);
    }

    return Some(tables::CmapFormat100 {
        format,
        length,
        language_code,
        start_char_code,
        num_chars,
        glyphs,
    });
}
pub fn read_cmap_format120(r: &mut reader::FontReader) -> Option<tables::CmapFormat120> {
    let format = 10.0;
    r.get_uint16()?; // just for that 32 bit float thing
    let length = r.get_uint32()?;
    let language_code = r.get_uint32()?;
    let n_groups = r.get_uint32()?;
    let mut groups = Vec::new();

    for _ in 0..n_groups {
        groups.push(tables::CmapFormat120Group {
            start_char_code: r.get_uint32()?,
            end_char_code: r.get_uint32()?,
            start_glyph_code: r.get_uint32()?,
        });
    }

    return Some(tables::CmapFormat120 {
        format,
        length,
        language_code,
        n_groups,
        groups,
    });
}

pub fn cmap_format_table(r: &mut reader::FontReader) -> Option<tables::CmapFormatTable> {
    let table = match r.get_uint16()? {
        0 => tables::CmapFormatTable::Format0(read_cmap_format0(r)?),
        2 => tables::CmapFormatTable::Format2(read_cmap_format2(r)?),
        4 => tables::CmapFormatTable::Format4(read_cmap_format4(r)?),
        6 => tables::CmapFormatTable::Format6(read_cmap_format6(r)?),
        8 => tables::CmapFormatTable::Format80(read_cmap_format80(r)?),
        10 => tables::CmapFormatTable::Format100(read_cmap_format100(r)?),
        12 => tables::CmapFormatTable::Format120(read_cmap_format120(r)?),
        val => {
            println!(
                "{} is not a valid format table version, at least not supported by me.",
                val
            );
            return None;
        }
    };

    return Some(table);
}
