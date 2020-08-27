use crate::reader;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct OffsetTable {
    pub checksum: u32, // checksum for this table
    pub offset: u32,   // offset from beginning of sfnt
    pub length: u32,   // length of this table in byte (actual length not padded length)
}

fn checksum(r: &mut reader::FontReader, offset: u32, length: u32) -> u32 {
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

pub fn read(r: &mut reader::FontReader, numtables: u16) -> Option<HashMap<String, OffsetTable>> {
    let mut offset_tables = HashMap::new();
    for _ in 0..numtables {
        let tag = r.get_string(4)?;
        let table = OffsetTable {
            checksum: r.get_uint32()?,
            offset: r.get_uint32()?,
            length: r.get_uint32()?,
        };

        offset_tables.insert(tag.clone(), table);

        if tag != "head" && checksum(r, table.offset, table.length) != table.checksum {
            println!("Checksums don't match for the {} table", tag);
            return None;
        }
    }
    Some(offset_tables)
}
