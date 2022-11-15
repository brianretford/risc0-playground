// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]

pub extern crate externc_libm as libm;

use risc0_zkvm_guest::env;
use dssim_core::{RGBAPLU, ToRGBAPLU};
use dssim_core::Dssim;
use png_decoder::decode;
use imgref::{Img, ImgVec};
use rgb::*;
risc0_zkvm_guest::entry!(main);

fn load() -> ImgVec<RGBAPLU> {
    let gray1_png_bytes: &[u8] = include_bytes!("../../../../../dssim/tests/gray1-gray.png");
    let (hdr, gray1_raw) = decode(gray1_png_bytes).unwrap();
    Img::new(gray1_raw.as_rgba().to_rgbaplu(), hdr.width as usize, hdr.height as usize)
}

pub fn main() {
    let comparer = Dssim::new();
    let img1 = load();
    let gray1 = comparer.create_image(&img1).unwrap();
    let gray2 = comparer.create_image(&img1).unwrap();
    let score : f64 = f64::from(comparer.compare(&gray1, gray2).0);
    env::commit(&score);
}
