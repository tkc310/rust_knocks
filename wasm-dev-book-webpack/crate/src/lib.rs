extern crate wasm_bindgen;
extern crate tinymt;

use wasm_bindgen::prelude::*;
use tinymt::tinymt32;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn rand() -> u32 {
    let param = tinymt32::Param {
        mat1: 0x8F7011EE,
        mat2: 0xFC78FF1F,
        tmat: 0x3793fdff,
    };
    let seed = 1;
    tinymt32::from_seed(param, seed).gen()
}

#[wasm_bindgen]
pub fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

#[wasm_bindgen]
pub fn twice(slice: &[i32]) -> Vec<i32> {
    slice.iter().map(|x| x * 2).collect()
}

#[wasm_bindgen]
pub fn greeting() -> String {
    String::from("hello")
}

