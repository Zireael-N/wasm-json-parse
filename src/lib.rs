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
static mut DATA: Option<SliceContainer> = None;

#[derive(Debug)]
struct SliceContainer<'a> {
    slice: &'a mut [u8],
    layout: std::alloc::Layout,
}

impl<'a> SliceContainer<'a> {
    unsafe fn realloc(&mut self, new_size: usize) {
        let ptr = std::alloc::realloc(self.slice.as_mut_ptr(), self.layout, new_size);
        self.slice = std::slice::from_raw_parts_mut(ptr, new_size);
    }
}

/**
  * Apparently converting std::slice::from_raw_parts
  * into a Box does not deallocate memory when the Box is dropped.
  */
impl<'a> Drop for SliceContainer<'a> {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.slice.as_mut_ptr(), self.layout) };
    }
}

#[wasm_bindgen]
pub fn allocate_buffer(size: usize) -> *const u8 {
    unsafe {
        if let Some(v) = &mut DATA {
            v.realloc(size);
        } else {
            let align = std::mem::align_of::<u8>();
            let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
            let ptr = std::alloc::alloc(layout);
            DATA = Some(SliceContainer {
                slice: std::slice::from_raw_parts_mut(ptr, size),
                layout,
            });
        }
        DATA.as_ref().unwrap().slice.as_ptr()
    }
}

#[wasm_bindgen]
pub fn parse_json_move() -> u8 {
    serde_json::from_str::<Value>(
        unsafe { &std::str::from_utf8_unchecked(DATA.as_ref().unwrap().slice) }
    ).as_u8()
}

#[wasm_bindgen]
pub fn parse_json_move_typed() -> u8 {
    serde_json::from_str::<Canada>(
        unsafe { &std::str::from_utf8_unchecked(DATA.as_ref().unwrap().slice) }
    ).as_u8()
}
