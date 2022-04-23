extern crate image;

use std::io::Cursor;

use image::{
    imageops::{resize, FilterType},
    GenericImageView,
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn resize_image(
    input_image: &[u8],
    target_height: f32,
    target_width: f32,
) -> Result<Vec<u8>, JsValue> {
    let img = image::load_from_memory(input_image).unwrap();

    let (img_height, img_width) = img.dimensions();

    let (resize_height, resize_width) = match (img_height, img_width) {
        (height, width) if height > width => {
            let width = (img_width as f32 / img_height as f32) * target_height;
            (width, target_height)
        }
        (height, width) if width > height => {
            let height = (img_height / img_width) as f32 * target_width;
            (target_width, height)
        }
        _ => (target_height, target_width),
    };

    let buffer = resize(
        &img,
        resize_width as u32,
        resize_height as u32,
        FilterType::Lanczos3,
    );

    let mut bytes: Vec<u8> = Vec::new();

    buffer
        .write_to(
            &mut Cursor::new(&mut bytes),
            image::ImageOutputFormat::Jpeg(75),
        )
        .unwrap();

    Ok(bytes)
}
