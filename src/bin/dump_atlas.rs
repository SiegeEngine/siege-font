
extern crate siege_font;
extern crate image;
extern crate bincode;

use std::env;
use std::fs::File;
use siege_font::FontAtlas;

fn main() {
    let args: Vec<String> = env::args().map(|e| e.to_owned()).collect();
    if args.len() != 2 {
        println!("Usage:  dump_atlas [atlas.bin]");
        return;
    }
    let file = &args[1];

    let atlasfile = File::open(file).unwrap();
    let atlas: FontAtlas = bincode::deserialize_from(&atlasfile).unwrap();
    println!("{:?}", atlas);
}
