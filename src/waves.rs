use crate::paint::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Waves {
  config: Config,
}

#[wasm_bindgen]
impl Waves {
  pub fn new(js_val: &JsValue) -> Self {
    Waves {
      config: Config::from(js_val),
    }
  }

  pub fn update(&mut self, js_val: &JsValue) {
    self.config = Config::from(js_val);
  }

  pub fn render(&self, context: &CanvasRenderingContext2d) {
    context.fill_rect(
      0.0,
      0.0,
      self.config.dims.width as f64,
      self.config.dims.height as f64,
    );
  }
}
