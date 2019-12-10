use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
extern crate serde_json;

#[wasm_bindgen]
#[derive(Serialize, Debug, Copy, Clone, Deserialize)]
pub struct Dims {
  pub width: u16,
  pub height: u16,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Config {
  pub dims: Dims,
}

#[wasm_bindgen]
impl Config {
  pub fn from(js_val: &JsValue) -> Self {
    js_val.into_serde().unwrap()
  }
}

pub trait Animation {
  fn new(config: Config) -> Self;
  fn update(&mut self, config: Config);
  fn render(&self, context: &CanvasRenderingContext2d);
}
