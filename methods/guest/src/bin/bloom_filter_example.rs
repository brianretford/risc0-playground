// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]

pub extern crate externc_libm as libm;

use bloomfilter::Bloom;
use risc0_zkvm_guest::env;
risc0_zkvm_guest::entry!(main);
use getrandom::*;

// use a decent rand() function
pub fn f42(buf: &mut [u8]) -> Result<(), Error> {
    for i in 0..buf.len() {
        buf[i] = 42;
    }
    Ok(())
}
register_custom_getrandom!(f42);

pub fn main() {
    let mut bloom = Bloom::new(10, 80);
    let mut k = vec![0u8, 16];
    getrandom(&mut k).unwrap();
    assert!(bloom.check(&k) == false);
    bloom.set(&k);
    assert!(&bloom.check(&k));
    env::commit(&bloom.check(&k));
}

