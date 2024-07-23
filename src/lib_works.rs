use wasm_bindgen::prelude::*;
use ndarray::{ArrayD, arr1};

#[wasm_bindgen]
extern "C" { }

#[wasm_bindgen]
pub struct Float32Array {
    inner: ArrayD<f32>
}

impl From<ArrayD<f32>> for Float32Array {
    fn from(value: ArrayD<f32>) -> Self { Self { inner: value } }
}

#[wasm_bindgen]
impl Float32Array {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Array dtype float32, {}", self.inner)
    }
}

#[wasm_bindgen]
pub struct Float64Array {
    inner: ArrayD<f64>
}

impl From<ArrayD<f64>> for Float64Array {
    fn from(value: ArrayD<f64>) -> Self { Self { inner: value } }
}

#[wasm_bindgen]
impl Float64Array {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Array dtype float64, {}", self.inner)
    }
}

#[wasm_bindgen]
pub fn make_float32() -> Float32Array {
    arr1(&[1f32, 2., 3., 4.]).into_dyn().into()
}

#[wasm_bindgen]
pub fn make_float64() -> Float64Array {
    arr1(&[1f64, 2., 3., 4.]).into_dyn().into()
}

#[wasm_bindgen]
pub fn sin_float32(arr: &Float32Array) -> Float32Array {
    arr.inner.map(|e| e.sin()).into()
}

#[wasm_bindgen]
pub fn cos_float32(arr: &Float32Array) -> Float32Array {
    arr.inner.map(|e| e.cos()).into()
}

#[wasm_bindgen]
pub fn sin_float64(arr: &Float64Array) -> Float64Array {
    arr.inner.map(|e| e.sin()).into()
}

#[wasm_bindgen]
pub fn cos_float64(arr: &Float64Array) -> Float64Array {
    arr.inner.map(|e| e.cos()).into()
}