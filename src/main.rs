use crossterm::terminal::*;
use figlet_rs::{FIGfont, FIGure};
use std::io::Result;
use std::vec::Vec;

#[derive(Debug)]
struct FIGDimmensions {
    width: u32,
    rows: u32,
    line_height: u32,
    characters: u32,
    breakpoints: Vec<u32>,
}


fn main() -> Result<()> {
    let font = FIGfont::from_file("resources/roman.flf").unwrap();
    let space_width: u32 = font.convert(" ").unwrap().characters.first().unwrap().width;

    let intro_string: String = "Hackerman Mode 030".to_string();
    let str_words: Vec<&str> = intro_string.split(" ").collect();

    let main_fig = font.convert("Hackerman Mode 030");

    let (cols, rows) = size()?;
    let dimmensions: FIGDimmensions = calculate_dimmensions(main_fig.unwrap(), cols);

    if dimmensions.rows.gt(&(1 as u32)) {
        for word in str_words {
            let w = calculate_width(font.convert(word).unwrap());
        }
    }

  //  println!("{:?}", dimmensions);

    Ok(())
}

fn calculate_width(figure: FIGure) -> u32 {
    let mut width: u32 = 0;

    for char in figure.characters {
        width += char.width;
    }

    return width;
}

fn calculate_dimmensions(figure: FIGure, cols: u16) -> FIGDimmensions {
    let mut width: u32 = 0;
    let mut count = 0;
    let mut bwidth = 0;
    let mut breakpoints: Vec<u32> = Vec::new();

    for char in figure.characters {
        bwidth += char.width;

        if bwidth.gt(&(cols as u32)) {
            bwidth = char.width;
            breakpoints.push(count)
        }

        width += char.width;
        count += 1
    }

    let rows = ((width / &(cols as u32)) as u32) + (((width % &(cols as u32)) > 0) as u32);

    return FIGDimmensions {
        width,
        rows,
        line_height: figure.height,
        characters: count,
        breakpoints,
    };
}
