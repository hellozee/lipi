use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct FontReader {
    pos: usize,
    data: Vec<u8>,
}

impl FontReader {
    pub fn new(filename: String) -> Self {
        return FontReader {
            pos: 0,
            data: match std::fs::read(filename) {
                Ok(val) => val,
                Err(_) => panic!("Can't read the file!!"),
            },
        };
    }

    pub fn seek(&mut self, pos: usize) -> usize {
        if pos >= self.data.len() {
            panic!("Reached the end of file");
        }

        let oldpos = self.pos;
        self.pos = pos;
        return oldpos;
    }

    pub fn get_uint8(&mut self) -> Option<u8> {
        let byte = self.data.get(self.pos)?.clone();
        self.pos += 1;
        return Some(byte);
    }

    pub fn get_uint16(&mut self) -> Option<u16> {
        let byte1 = self.get_uint8()?;
        let byte2 = self.get_uint8()?;
        return Some((byte1 as u16) << 8 | byte2 as u16);
    }

    pub fn get_uint32(&mut self) -> Option<u32> {
        let byte1 = self.get_uint16()?;
        let byte2 = self.get_uint16()?;
        return Some((byte1 as u32) << 16 | byte2 as u32);
    }

    pub fn get_int16(&mut self) -> Option<i16> {
        Some(self.get_uint16()? as i16)
    }

    pub fn get_int32(&mut self) -> Option<i32> {
        Some(self.get_uint32()? as i32)
    }

    pub fn get_float32(&mut self) -> Option<f32> {
        Some(self.get_int32()? as f32 / (1 << 16) as f32)
    }

    pub fn get_string(&mut self, length: usize) -> Option<String> {
        let mut result = String::new();

        for _ in 0..length {
            result.push(self.get_uint8()? as char);
        }

        return Some(result);
    }

    pub fn get_date(&mut self) -> Option<NaiveDateTime> {
        let unix_hfs_epoch_diff = 208284480;
        let time =
            ((self.get_uint32()? as u64) << 32 | self.get_uint32()? as u64) - unix_hfs_epoch_diff;
        let date = NaiveDateTime::from_timestamp(time as i64, 0);
        return Some(date);
    }
}
