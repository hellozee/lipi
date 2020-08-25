mod truetype;

fn main() {
    let _ = match truetype::TrueTypeFont::new("FontAwesome.ttf".into()) {
        Some(val) => val,
        None => panic!("Can't read the file"),
    };
}
