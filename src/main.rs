mod reader;
mod tables;
pub mod truetype;

fn main() {
    let tt = match truetype::TrueTypeFont::new("FontAwesome.ttf".into()) {
        Some(val) => val,
        None => panic!("There some problem with the file."),
    };

    println!("{:?}", tt);
}
