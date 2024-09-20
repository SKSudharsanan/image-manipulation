use wasm_bindgen::prelude::*;
use image::{load_from_memory, DynamicImage, ImageOutputFormat};
use std::io::Cursor;

#[wasm_bindgen]
pub fn flip_horizontal(input: &[u8]) -> Vec<u8> {
    // Load image from the input bytes
    let img = load_from_memory(input).expect("Failed to load image");

    // Flip the image horizontally
    let flipped_img = img.fliph();

    // Create a Cursor wrapping the Vec<u8> to enable Write + Seek
    let mut output = Cursor::new(Vec::new());

    // Write the flipped image to the output buffer in PNG format
    flipped_img
        .write_to(&mut output, ImageOutputFormat::Png)
        .expect("Failed to write image");

    // Return the underlying Vec<u8> from the Cursor
    output.into_inner()
}

#[wasm_bindgen]
pub fn flip_vertical(input: &[u8]) -> Vec<u8> {
    // Load image from the input bytes
    let img = load_from_memory(input).expect("Failed to load image");

    // Flip the image horizontally
    let flipped_img = img.flipv();

    // Create a Cursor wrapping the Vec<u8> to enable Write + Seek
    let mut output = Cursor::new(Vec::new());

    // Write the flipped image to the output buffer in PNG format
    flipped_img
        .write_to(&mut output, ImageOutputFormat::Png)
        .expect("Failed to write image");

    // Return the underlying Vec<u8> from the Cursor
    output.into_inner()
}
