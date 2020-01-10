use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::{
    Value,
    // Result,
}; 

trait AsUint8 {
    fn as_u8(&self) -> u8;
}

impl<T, E> AsUint8 for Result<T, E> {
    #[inline(always)]
    fn as_u8(&self) -> u8 {
        match self {
            Ok(_) => 1,
            Err(_) => 0,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Canada {
    #[serde(rename = "type")]
    pub obj_type: String,
    pub features: Vec<Feature>,
}

#[derive(Deserialize, Serialize)]
pub struct Feature {
    #[serde(rename = "type")]
    pub obj_type: String,
    pub properties: BTreeMap<String, String>,
    pub geometry: Geometry,
}

#[derive(Deserialize, Serialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub obj_type: String,
    pub coordinates: Vec<Vec<(f32, f32)>>,
}

#[wasm_bindgen]
pub fn parse_json(data: &str) -> u8 {
    serde_json::from_str::<Value>(data).as_u8()
}

#[wasm_bindgen]
pub fn parse_json_typed(data: &str) -> u8 {
    serde_json::from_str::<Canada>(data).as_u8()
}

// Warning: unsafe code ahead
static mut DATA: Option<Vec<u8>> = None;

#[wasm_bindgen]
pub fn allocate_buffer(size: usize) -> *const u8 {
    unsafe {
        DATA = None; // Dealloc old Vec

        let mut v = Vec::with_capacity(size);
        v.set_len(size);
        DATA = Some(v);
        DATA.as_ref().unwrap().as_ptr()
    }
}

#[wasm_bindgen]
pub fn parse_json_move() -> u8 {
    serde_json::from_str::<Value>(
        unsafe { &std::str::from_utf8_unchecked(DATA.as_ref().unwrap()) }
    ).as_u8()
}

#[wasm_bindgen]
pub fn parse_json_move_typed() -> u8 {
    serde_json::from_str::<Canada>(
        unsafe { &std::str::from_utf8_unchecked(DATA.as_ref().unwrap()) }
    ).as_u8()
}
