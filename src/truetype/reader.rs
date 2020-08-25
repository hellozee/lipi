#[derive(Debug)]
pub struct BinaryReader {
    pos: usize,
    data: Vec<u8>,
}

impl BinaryReader {
    pub fn new(filename: String) -> Result<Self, std::io::Error> {
        return Ok(BinaryReader {
            pos: 0,
            data: std::fs::read(filename)?,
        });
    }

    pub fn seek(&mut self, pos: usize) -> Result<usize, String> {
        if pos >= self.data.len() {
            return Err(String::from("Reached the end of file"));
        }

        let oldpos = self.pos;
        self.pos = pos;
        return Ok(oldpos);
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
}
