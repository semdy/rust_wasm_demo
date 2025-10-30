use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

#[wasm_bindgen]
pub fn draw_circle(canvas_id: &str) -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");

    let document = window.document().expect("window should have a document");

    let canvas = document
        .get_element_by_id(canvas_id)
        .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok())
        .expect("canvas element not found");

    canvas.set_width(500);
    canvas.set_height(500);

    let context = canvas
        .get_context("2d")
        .expect("failed to get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("context is not of type 2d");

    context.begin_path();

    context.arc(250.0, 250.0, 100.0, 0.0, std::f64::consts::PI * 2.0)?;

    context.set_fill_style_str("blue");
    // context.set_fill_style(&JsValue::from_str("blue"));

    context.fill();

    context.set_stroke_style_str("black");
    // context.set_stroke_style(&JsValue::from_str("black"));

    context.stroke();
    Ok(())
}
