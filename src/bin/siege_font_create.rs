
extern crate siege_font;
extern crate image;
extern crate bincode;
extern crate png;
extern crate ddsfile;

use std::env;
use siege_font::build;

use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use ddsfile::{DxgiFormat, D3D10ResourceDimension, AlphaMode};

fn main() {
    let args: Vec<String> = env::args().map(|e| e.to_owned()).collect();
    if args.len() < 5 {
        println!("Usage:  create [fontfile] [fontsize] [big_margin] [small_width]");
        return;
    }
    let filename = &args[1];
    let fontsize: f32 = args[2].parse::<f32>().unwrap();
    let big_margin: u32 = args[3].parse::<u32>().unwrap();
    let small_width: u32 = args[4].parse::<u32>().unwrap();

    let mut codepoint_ranges: Vec<(u32,u32)> = Vec::new();
    for i in 5..args.len() {
        match &*args[i] {
            "Basic Latin" => codepoint_ranges.push((0x0000, 0x007F)),
            "Specials" => codepoint_ranges.push((0xFFF0, 0xFFFD)),
            "Latin-1 Supplement" => codepoint_ranges.push((0x0080, 0x00FF)),
            "Latin-1 Supplement PW" => {
                // Modified range which skips chars in Planewalker that are just open boxes
                // Skipped: AA, AC, AF-B3, B9-BA, BC-BE
                codepoint_ranges.push((0x0080, 0x00A9));
                codepoint_ranges.push((0x00AB, 0x00AB));
                codepoint_ranges.push((0x00AD, 0x00AE));
                codepoint_ranges.push((0x00B4, 0x00B8));
                codepoint_ranges.push((0x00BB, 0x00BB));
                codepoint_ranges.push((0x00BF, 0x00BF));
                codepoint_ranges.push((0x00C0, 0x00FF));
            },
            "CJK Symbols and Punctuation" => codepoint_ranges.push((0x3000, 0x303F)),
            "Katakana" => codepoint_ranges.push((0x30A0, 0x30FF)),
            "Hiragana" => codepoint_ranges.push((0x3040, 0x309F)),
            "Cyrillic" => codepoint_ranges.push((0x0400, 0x04FF)),
            "Arabic" => codepoint_ranges.push((0x0600, 0x06FF)),
            "CJK Unified Ideographs 1" => codepoint_ranges.push((0x4E00, 0x5C00)),
            "CJK Unified Ideographs 2" => codepoint_ranges.push((0x5C00, 0x6800)),
            "CJK Unified Ideographs 3" => codepoint_ranges.push((0x6800, 0x7400)),
            "CJK Unified Ideographs 4" => codepoint_ranges.push((0x7400, 0x8000)),
            "CJK Unified Ideographs 5" => codepoint_ranges.push((0x8000, 0x9000)),
            "CJK Unified Ideographs 6" => codepoint_ranges.push((0x9000, 0x9FFF)),
            "General Punctuation" => { // 2000 - 206F
                // Some ranges are not printable and so we skip them
                codepoint_ranges.push((0x2010, 0x2010));
                codepoint_ranges.push((0x2012, 0x2027));
                codepoint_ranges.push((0x2030, 0x205E));
            },
            "Currency Symbols" => codepoint_ranges.push((0x20A0, 0x20CF)),
            "Latin Extented-A" => codepoint_ranges.push((0x0100, 0x017F)),
            "Spacing Modifier Letters" => codepoint_ranges.push((0x02B0, 0x02FF)),
            //
            "Box Drawing" => codepoint_ranges.push((0x2500, 0x257F)),
            "Runic" => codepoint_ranges.push((0x16A0, 0x16FF)),
            // More TBD.
            _ => {}
        }
    }

    // Build the FontAtlas and the SDF image
    let (atlas, imgbuf) = build(filename, fontsize, big_margin, small_width, true,
                                &*codepoint_ranges);

    // Create DDS file
    let mut dds = ddsfile::Dds::new_dxgi(
        small_width, small_width, None,
        DxgiFormat::R8_UNorm,
        None, None, None, false,
        D3D10ResourceDimension::Texture2D,
        AlphaMode::Opaque).unwrap();

    // Copy data from imgbuf
    {
        let data: &mut [u8] = dds.get_mut_data(0).unwrap();
        for (i,pixel) in imgbuf.pixels().enumerate() {
            data[i] = pixel.data[0];
        }
    }

    // Save dds file
    let filestem = Path::new(filename)
        .file_stem().unwrap()
        .to_string_lossy().into_owned();
    let pathstring = format!("{}.dds", filestem);
    let path = Path::new(&*pathstring);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    dds.write(w).unwrap();

    let mut atlasfile = File::create(&*format!("{}.bin", filestem)).unwrap();
    bincode::serialize_into(&mut atlasfile, &atlas).unwrap();
}
