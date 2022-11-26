#![no_main]

pub extern crate externc_libm as libm;

use png_decoder::*;
use risc0_zkvm_guest::{sha, env};
risc0_zkvm_guest::entry!(main);

pub fn main() {
    // TODO take image as input
    let gray1_png_bytes: &[u8] = include_bytes!("../../../../../dssim/tests/gray1-gray.png");
    let (hdr, gray1_raw) = decode(gray1_png_bytes).unwrap();
    let image_hash = sha::digest_u8_slice(&gray1_raw);
    // commit to the hash of the image 
    env::commit(&image_hash);
    // proove that you know it's width and height
    env::commit(&hdr.width);
    env::commit(&hdr.height);
}

