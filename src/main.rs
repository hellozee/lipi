mod truetype;

fn main() {
    let tt = match truetype::TrueTypeFont::new("FontAwesome.ttf".into()) {
        Some(val) => val,
        None => panic!("Can't read the file"),
    };

    println!("{:?}", tt);
}
