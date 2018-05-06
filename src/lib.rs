
extern crate font_atlas;
extern crate image;
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate distance_field;
extern crate png;
extern crate ddsfile;

use std::collections::HashMap;
use image::{ImageBuffer, Luma, DynamicImage};
use font_atlas::rasterize::CharInfo;
use distance_field::DistanceFieldExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Box {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// Character Info.  This is a slightly paired-down version of font-atlas::CharInfo
/// without 'chr' and 'fontsize' fields (which I find unnecessary).
#[derive(Debug, Serialize, Deserialize)]
pub struct CInfo {
    /// The inner bounding box, excluding margins
    pub inner_bounding_box: Box,
    /// The amount that the pen should move +x before drawing the character
    pub pre_draw_advance: f32,
    /// The amount that the pen should move +x after drawing the character
    pub post_draw_advance: f32,
    /// The amount of y that the pen should move for drawing
    /// this specific character.  This value gets reset after
    /// drawing.
    pub height_offset: f32,
}

impl From<CharInfo> for CInfo {
    fn from(c: CharInfo) -> CInfo {
        CInfo {
            inner_bounding_box: Box {
                x: c.bounding_box.x as f32,
                y: c.bounding_box.y as f32,
                w: c.bounding_box.w as f32,
                h: c.bounding_box.h as f32
            },
            pre_draw_advance: c.pre_draw_advance.0,
            post_draw_advance: c.post_draw_advance.0,
            height_offset: c.height_offset,
        }
    }
}

/// This is an enhanced version of font_atlas::Atlas, with additional detail
/// about the font atlas
#[derive(Debug, Serialize, Deserialize)]
pub struct FontAtlas {
    /// The name of the font
    pub font_name: String,
    /// The line height
    pub line_height: f32,
    /// The font size (in px)
    pub fontsize: f32,
    /// The margin between characters in the font map image, measured in pixels
    pub margin: f32,
    /// Character information
    pub map: HashMap<char, CInfo>,
}

pub fn build(fontfilename: &str, big_fontsize: f32, big_margin: u32, small_width: u32,
             output_big: bool, codepoint_ranges: &[(u32, u32)])
             -> (FontAtlas, ImageBuffer<Luma<u8>, Vec<u8>>)
{
    let font = match font_atlas::load_font(fontfilename) {
        Ok(f) => f,
        Err(e) => panic!("Error loading font: {:?}", e)
    };

    let (atlas, bitmap, line_height) = font.make_atlas_all(
        big_fontsize,
        big_margin,
        64, // starting bitmap width (may grow)
        64, // starting bitmap height (may grow)
        codepoint_ranges);

    println!("Original bitmap size is {}", bitmap.width);
    let shrink = bitmap.width as f32 / small_width as f32;

    use std::path::Path;
    let fontfilestem = Path::new(fontfilename)
        .file_stem().unwrap()
        .to_string_lossy().into_owned();

    // Output big image to png (optional)
    if output_big {
        let width = bitmap.width() as u32;
        let height = bitmap.height() as u32;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let index: usize = (x + width*y) as usize;
            Luma([bitmap.raw()[index]])
        });
        img.save(&*format!("{}.png", fontfilestem)).unwrap();
    }

    // Apply signed-distance function
    let sdfimg = {
        let width = bitmap.width() as u32;
        let height = bitmap.height() as u32;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let index: usize = (x + width*y) as usize;
            Luma([bitmap.raw()[index]])
        });
        let img = DynamicImage::ImageLuma8(img);
        img.distance_field(distance_field::Options {
            size: (small_width, small_width),
            max_distance: big_margin as u16, // applied to input
            ..Default::default()
        })
    };

    let font_atlas = FontAtlas {
        font_name: fontfilestem,
        line_height: line_height / shrink as f32,
        fontsize: big_fontsize / shrink as f32,
        margin: big_margin as f32 / shrink,
        map: atlas.char_info.into_iter().map(|(c,ci)| {
            let mut cinfo: CInfo = From::from(ci);
            cinfo.inner_bounding_box.x /= shrink;
            cinfo.inner_bounding_box.y /= shrink;
            cinfo.inner_bounding_box.w /= shrink;
            cinfo.inner_bounding_box.h /= shrink;
            cinfo.pre_draw_advance /= shrink;
            cinfo.post_draw_advance /= shrink;
            cinfo.height_offset /= shrink;
            (c,cinfo)
        }).collect(),
    };

    (font_atlas, sdfimg)
}
