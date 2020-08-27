use crate::reader;
use crate::tables::offset::OffsetTable;

#[derive(Debug, Copy, Clone)]
pub struct NameRecord {
    pub platform_id: u16,          // Platform identifier code.
    pub platform_specific_id: u16, // Platform-specific encoding identifier.
    pub language_id: u16,          // Language identifier.
    pub name_id: u16,              // Name identifiers.
    pub length: u16,               // Name string length in bytes.
    pub offset: u16,               // Name string offset in bytes from stringOffset.
}

#[derive(Debug, Clone)]
pub struct Name {
    pub format: u16,                   // Format selector. Set to 0.
    pub count: u16,                    // The number of nameRecords in this name table.
    pub string_offset: u16, // Offset in bytes to the beginning of the name character strings.
    pub name_records: Vec<NameRecord>, // The name records array.
}

pub fn read(r: &mut reader::FontReader, name_offset_table: OffsetTable) -> Option<Name> {
    let _ = r.seek(name_offset_table.offset as usize);

    let format = r.get_uint16()?;
    let count = r.get_uint16()?;
    let string_offset = r.get_uint16()?;
    let mut name_records = Vec::new();

    for _ in 0..count {
        name_records.push(NameRecord {
            platform_id: r.get_uint16()?,
            platform_specific_id: r.get_uint16()?,
            language_id: r.get_uint16()?,
            name_id: r.get_uint16()?,
            length: r.get_uint16()?,
            offset: r.get_uint16()?,
        });
    }

    return Some(Name {
        format,
        count,
        string_offset,
        name_records,
    });
}
