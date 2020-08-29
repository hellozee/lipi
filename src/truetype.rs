use crate::reader;
use crate::tables;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TrueTypeFont {
    file: reader::FontReader,
    offset_sub_table: tables::offset_sub::OffsetSubTable,
    offset_tables: HashMap<String, tables::offset::OffsetTable>,
    head: tables::head::Head,
    maxp: tables::maxp::Maxp,
    cmap: tables::cmap::Cmap,
    hhea: tables::hhea::Hhea,
    hmtx: tables::hmtx::Hmtx,
    loca: tables::loca::Loca,
    name: tables::name::Name,
}

impl TrueTypeFont {
    pub fn new(filename: String) -> Option<Self> {
        let mut file = reader::FontReader::new(filename);

        let offset_sub_table = tables::offset_sub::read(&mut file)?;
        let offset_tables = tables::offset::read(&mut file, offset_sub_table.numtables)?;
        let head = tables::head::read(&mut file, *offset_tables.get("head")?)?;
        let maxp = tables::maxp::read(&mut file, *offset_tables.get("maxp")?)?;
        let cmap = tables::cmap::read(&mut file, *offset_tables.get("cmap")?)?;
        let hhea = tables::hhea::read(&mut file, *offset_tables.get("hhea")?)?;
        let hmtx = tables::hmtx::read(
            &mut file,
            *offset_tables.get("hmtx")?,
            hhea.num_of_long_hor_metrics,
            maxp.glyph_count,
        )?;

        let loca = tables::loca::read(
            &mut file,
            *offset_tables.get("loca")?,
            maxp.glyph_count,
            head.index_to_loc_format,
        )?;

        let name = tables::name::read(&mut file, *offset_tables.get("name")?)?;

        // TODO: skipping the post table for now, could care less about postscript anyway

        return Some(TrueTypeFont {
            file,
            offset_sub_table,
            offset_tables,
            head,
            maxp,
            cmap,
            hhea,
            hmtx,
            loca,
            name,
        });
    }

    fn glyph_offset(&mut self, index: u32) -> Option<u32> {
        let loca_table_offset = self.offset_tables.get("loca".into())?.offset;
        let glyf_table_offset = self.offset_tables.get("glyf".into())?.offset;

        match self.head.index_to_loc_format {
            0 => {
                let old = self.file.seek((loca_table_offset + index * 4) as usize);
                let offset = self.file.get_uint32()?;
                let next = self.file.get_uint32()?;
                self.file.seek(old);

                if offset == next {
                    return Some(0);
                } else {
                    return Some(offset + glyf_table_offset);
                }
            }
            1 => {
                let old = self.file.seek((loca_table_offset + index * 2) as usize);
                let offset = self.file.get_uint16()? * 2;
                let next = self.file.get_uint16()? * 2;
                self.file.seek(old);

                if offset == next {
                    return Some(0);
                } else {
                    return Some(offset as u32 + glyf_table_offset);
                }
            }
            _ => return None,
        }
    }

    pub fn glyph(&mut self, index: u32) -> Option<tables::glyf::Glyph> {
        let offset = self.glyph_offset(index)?;

        let glyph_offset_table = self.offset_tables.get("glyf")?;

        if offset == 0 || offset >= glyph_offset_table.offset + glyph_offset_table.length {
            return None;
        }

        self.file.seek(offset as usize);
        let number_of_contours = self.file.get_int16()?;
        let xmin = self.file.get_int16()?;
        let ymin = self.file.get_int16()?;
        let xmax = self.file.get_int16()?;
        let ymax = self.file.get_int16()?;

        if number_of_contours < -1 {
            return None;
        }

        return Some(tables::glyf::Glyph {
            number_of_contours,
            xmin,
            ymin,
            xmax,
            ymax,
            glyph_data: match number_of_contours {
                -1 => self.compound_glyph()?,
                _ => self.simple_glyph(number_of_contours)?,
            },
        });
    }

    fn compound_glyph(&mut self) -> Option<tables::glyf::GlyphData> {
        let arg_1_and_2_are_words = 1;
        let args_are_xy_values = 2;
        let round_xy_to_grid = 4;
        let we_have_a_scale = 8;
        // obsolete should be zero = 16
        let more_components = 32;
        let we_have_an_x_and_y_scale = 64;
        let we_have_a_two_by_two = 128;
        let we_have_instructions = 256;
        let use_my_metrics = 512;
        let overlap_component = 1024;

        let mut flag = more_components;
        let mut end_pts_of_contours = Vec::new();
        let mut x_coordinates = Vec::new();
        let mut y_coordinates = Vec::new();
        let mut flags = Vec::new();

        while (flag & more_components) > 0 {
            flag = self.file.get_uint16()?;
            let mut component = tables::glyf::Component {
                index: self.file.get_uint16()?,
                matrix: (1., 0., 0., 1., 0., 0.),
                destination_index: 0,
                source_index: 0,
            };
            let (argument_1, argument_2) = match (flag & arg_1_and_2_are_words) > 0 {
                true => (self.file.get_int16()? as i32, self.file.get_int16()? as i32),
                false => (self.file.get_uint8()? as i32, self.file.get_uint8()? as i32),
            };

            if (flag & args_are_xy_values) > 0 {
                component.matrix.4 = argument_1 as f64;
                component.matrix.5 = argument_2 as f64;
            } else {
                component.destination_index = argument_1;
                component.source_index = argument_2;
            }

            if (flag & we_have_a_scale) > 0 {
                component.matrix.0 = self.file.get_f2dot14()? as f64;
                component.matrix.3 = component.matrix.0;
            } else if (flag & we_have_an_x_and_y_scale) > 0 {
                component.matrix.0 = self.file.get_f2dot14()? as f64;
                component.matrix.3 = self.file.get_f2dot14()? as f64;
            } else if (flag & we_have_a_two_by_two) > 0 {
                component.matrix.0 = self.file.get_f2dot14()? as f64;
                component.matrix.1 = self.file.get_f2dot14()? as f64;
                component.matrix.2 = self.file.get_f2dot14()? as f64;
                component.matrix.3 = self.file.get_f2dot14()? as f64;
            }

            let old = self.file.pos;
            match self.glyph(component.index as u32) {
                Some(glyph) => {
                    let simple_glyph = glyph.glyph_data;
                    let offset = x_coordinates.len();
                    for i in 0..simple_glyph.end_pts_of_contours.len() {
                        end_pts_of_contours
                            .push(simple_glyph.end_pts_of_contours[i] + offset as u16);
                    }

                    for i in 0..simple_glyph.x_coordinates.len() {
                        let mut x = simple_glyph.x_coordinates[i];
                        let mut y = simple_glyph.y_coordinates[i];

                        x = (component.matrix.0 * x as f64
                            + component.matrix.1 * y as f64
                            + component.matrix.4) as i16;

                        y = (component.matrix.2 * x as f64
                            + component.matrix.3 * y as f64
                            + component.matrix.5) as i16;

                        x_coordinates.push(x);
                        y_coordinates.push(y);
                        flags.push(simple_glyph.flags[i]);
                    }
                }

                None => {
                    self.file.seek(old);
                    continue;
                }
            };
            self.file.seek(old);
        }

        match (flag & we_have_instructions) > 0 {
            true => {
                let instruction_length = self.file.get_uint16()?;
                let mut instructions = Vec::new();
                for _ in 0..instruction_length {
                    instructions.push(self.file.get_uint8()?);
                }

                let instruction_length = Some(instruction_length);
                let instructions = Some(instructions);

                return Some(tables::glyf::GlyphData {
                    end_pts_of_contours,
                    instruction_length,
                    instructions,
                    flags,
                    x_coordinates,
                    y_coordinates,
                });
            }
            false => {
                let instruction_length = None;
                let instructions = None;

                return Some(tables::glyf::GlyphData {
                    end_pts_of_contours,
                    instruction_length,
                    instructions,
                    flags,
                    x_coordinates,
                    y_coordinates,
                });
            }
        }
    }

    fn simple_glyph(&mut self, number_of_contours: i16) -> Option<tables::glyf::GlyphData> {
        let on_curve = 1;
        let x_is_byte = 2;
        let y_is_byte = 4;
        let repeat = 8;
        let x_delta = 16;
        let y_delta = 32;

        let mut end_pts_of_contours = Vec::new();

        for _ in 0..number_of_contours {
            end_pts_of_contours.push(self.file.get_uint16()?);
        }

        let instruction_length = self.file.get_uint16()?;
        let mut instructions = Vec::new();
        for _ in 0..instruction_length {
            instructions.push(self.file.get_uint8()?);
        }

        if number_of_contours == 0 {
            return None;
        }

        let numpoints = end_pts_of_contours.len() + 1;

        let mut flags = Vec::new();
        let mut i = 0;

        while i < numpoints {
            let flag = self.file.get_uint8()?;
            flags.push(flag);
            if (flag & repeat) > 0 {
                let repeat_count = self.file.get_uint8()?;
                if repeat_count < 1 {
                    return None;
                }
                i += repeat_count as usize;
                for _ in 0..repeat_count {
                    flags.push(flag);
                }
            }
        }

        let mut x_coordinates = Vec::new();
        let mut y_coordinates = Vec::new();

        for i in 0..numpoints {
            let flag = flags[i];
            let mut value = 0;
            if (flag & x_is_byte) > 0 {
                if (flag & x_delta) > 0 {
                    value += self.file.get_uint8()? as i16;
                } else {
                    value -= self.file.get_uint8()? as i16;
                }
            } else if (!flag & x_delta) > 0 {
                value += self.file.get_int16()?;
            }
            x_coordinates.push(value);
        }

        for i in 0..numpoints {
            let flag = flags[i];
            let mut value = 0;
            if (flag & y_is_byte) > 0 {
                if (flag & y_delta) > 0 {
                    value += self.file.get_uint8()? as i16;
                } else {
                    value -= self.file.get_uint8()? as i16;
                }
            } else if (!flag & y_delta) > 0 {
                value += self.file.get_int16()?;
            }
            y_coordinates.push(value);
        }

        let instruction_length = Some(instruction_length);
        let instructions = Some(instructions);

        return Some(tables::glyf::GlyphData {
            end_pts_of_contours,
            instruction_length,
            instructions,
            flags,
            x_coordinates,
            y_coordinates,
        });
    }
}
