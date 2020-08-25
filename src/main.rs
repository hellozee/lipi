mod truetype;

fn main() {
    let mut tt = match truetype::TrueType::new("FontAwesome.ttf".into()) {
        Some(val) => val,
        None => panic!("Can't read the file"),
    };

    for i in 0..tt.length() {
        println!(
            "{:#?}",
            match tt.read_glyph(i as u32) {
                Some(val) => val,
                None => panic!("Can't read the glyph"),
            }
        );
    }
}
