use figlet_rs::FIGfont;

fn main() {
    let font = FIGfont::from_file("resources/roman.flf").unwrap();
    let figure = font.convert("Hackerman Mode 030");

    print!("{}", figure.unwrap().to_string())
}
