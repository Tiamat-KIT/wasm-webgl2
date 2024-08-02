mod utils;
use wasm_bindgen::prelude::*;
use serde::{Serialize,Deserialize};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

#[wasm_bindgen]
pub fn gen_context(id: &str) -> web_sys::HtmlCanvasElement{
    let document = gloo::utils::document();
    let element = document.get_element_by_id(id).unwrap();
    let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas   
}

#[wasm_bindgen]
pub fn webgl_context(canvas: &HtmlCanvasElement) -> web_sys::WebGl2RenderingContext{
    let context = canvas.get_context("webgl2")
        .unwrap().unwrap().dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    context
}

#[wasm_bindgen]
pub fn clear_black(canvas: &HtmlCanvasElement,gl: &web_sys::WebGl2RenderingContext){
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}
