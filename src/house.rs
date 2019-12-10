use crate::paint::Config;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct House {
  config: Config,
}

#[wasm_bindgen]
impl House {
  pub fn new(js_val: &JsValue) -> Self {
    House {
      config: Config::from(js_val),
    }
  }

  pub fn update(&mut self, js_val: &JsValue) {
    self.config = Config::from(js_val);
  }

  pub fn render(&self, context: &CanvasRenderingContext2d) {
    // Set line width
    context.set_line_width(10.0);

    // Wall
    context.stroke_rect(75.0, 140.0, 150.0, 110.0);

    // Door
    context.fill_rect(130.0, 190.0, 40.0, 60.0);

    // Roof
    context.move_to(50.0, 140.0);
    context.line_to(150.0, 60.0);
    context.line_to(250.0, 140.0);
    context.close_path();
    context.stroke();
  }
}
