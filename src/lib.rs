use std::fmt;

use wasm_bindgen::prelude::*;
use num_complex::Complex;
use ndarray::{ArrayD, arr1};

#[wasm_bindgen]
extern "C" { }

#[wasm_bindgen]
pub struct JsArray {
    inner: DynArray
}

impl From<DynArray> for JsArray {
    fn from(value: DynArray) -> Self { JsArray { inner: value } }
}

#[wasm_bindgen]
impl JsArray {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Array dtype {}, {}", self.inner.dtype(), self.inner)
    }
}

#[derive(Debug, Clone)]
pub enum DynArray {
    Float32(ArrayD<f32>),
    Float64(ArrayD<f64>),
    Complex64(ArrayD<Complex<f32>>),
    Complex128(ArrayD<Complex<f64>>),
}

impl From<ArrayD<f32>> for DynArray {
    fn from(value: ArrayD<f32>) -> Self { DynArray::Float32(value) }
}
impl From<ArrayD<f64>> for DynArray {
    fn from(value: ArrayD<f64>) -> Self { DynArray::Float64(value) }
}
impl From<ArrayD<Complex<f32>>> for DynArray {
    fn from(value: ArrayD<Complex<f32>>) -> Self { DynArray::Complex64(value) }
}
impl From<ArrayD<Complex<f64>>> for DynArray {
    fn from(value: ArrayD<Complex<f64>>) -> Self { DynArray::Complex128(value) }
}

impl fmt::Display for DynArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynArray::Float32(arr) => { fmt::Display::fmt(arr, f) },
            DynArray::Float64(arr) => { fmt::Display::fmt(arr, f) },
            DynArray::Complex64(arr) => { fmt::Display::fmt(arr, f) },
            DynArray::Complex128(arr) => { fmt::Display::fmt(arr, f) },
        }
    }
}

impl DynArray {
    pub fn dtype(&self) -> &'static str {
        match self {
            DynArray::Float32(_) => "float32",
            DynArray::Float64(_) => "float64",
            DynArray::Complex64(_) => "complex64",
            DynArray::Complex128(_) => "complex128",
        }
    }

    pub fn sin(&self) -> DynArray {
        match self {
            DynArray::Float32(arr) => arr.map(|e| e.sin()).into(),
            DynArray::Float64(arr) => arr.map(|e| e.sin()).into(),
            DynArray::Complex64(arr) => arr.map(|e| e.sin()).into(),
            DynArray::Complex128(arr) => arr.map(|e| e.sin()).into(),
        }
    }

    pub fn cos(&self) -> DynArray {
        match self {
            DynArray::Float32(arr) => arr.map(|e| e.cos()).into(),
            DynArray::Float64(arr) => arr.map(|e| e.cos()).into(),
            DynArray::Complex64(arr) => arr.map(|e| e.cos()).into(),
            DynArray::Complex128(arr) => arr.map(|e| e.cos()).into(),
        }
    }
}

#[wasm_bindgen]
pub fn make_float32() -> JsArray {
    DynArray::from(arr1(&[1f32, 2., 3., 4.]).into_dyn()).into()
}

#[wasm_bindgen]
pub fn make_float64() -> JsArray {
    DynArray::from(arr1(&[1f64, 2., 3., 4.]).into_dyn()).into()
}

#[wasm_bindgen]
pub fn sin(val: &JsArray) -> JsArray {
    val.inner.sin().into()
}

#[wasm_bindgen]
pub fn cos(val: &JsArray) -> JsArray {
    val.inner.cos().into()
}
