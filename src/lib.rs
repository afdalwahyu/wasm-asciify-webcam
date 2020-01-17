use std::str::from_utf8;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const ASCII_CHAR: [char; 18] = [
    ' ', '.', '^', ',', ':', '_', '=', '~', '+', 
    'O', 'o', '*', '#', '&', '%', 'B', '@', '$',
];
const N_CHAR: u8 = ASCII_CHAR.len() as u8;

fn intensity_to_ascii(value: &u8) -> char {
    let step = 255u8 / N_CHAR;
    for i in 1..step {
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ASCII_CHAR[idx];
        }
    }

    ASCII_CHAR[(N_CHAR - 1) as usize]
}

#[wasm_bindgen(start)]
pub fn render() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let content = document.get_element_by_id("content").unwrap();

    let data = context.get_image_data(0.0, 0.0, 512.0, 512.0).unwrap().data();

    let source = image::GrayImage::from_raw(512, 512, data.to_vec()).unwrap();

    let ascii_art = source
        .pixels()
        .map(|p| intensity_to_ascii(&p[0]))
        .collect::<String>();

    let subs = ascii_art
        .as_bytes()
        .chunks(512)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("<br>");

    content.set_inner_html(&format!("{}", subs));
}
