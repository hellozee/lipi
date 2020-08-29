// Yes it is spelt correctly

pub struct GlyphData {
    pub end_pts_of_contours: Vec<u16>,
    pub instruction_length: Option<u16>,
    pub instructions: Option<Vec<u8>>,
    pub flags: Vec<u8>,
    pub x_coordinates: Vec<i16>,
    pub y_coordinates: Vec<i16>,
}

pub struct Glyph {
    pub number_of_contours: i16,
    pub xmin: i16,
    pub ymin: i16,
    pub xmax: i16,
    pub ymax: i16,
    pub glyph_data: GlyphData,
}

pub struct Component {
    pub index: u16,
    pub matrix: (f64, f64, f64, f64, f64, f64),
    pub destination_index: i32,
    pub source_index: i32,
}
