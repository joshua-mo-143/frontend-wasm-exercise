use base64::{engine::general_purpose, Engine as _};
use wasm_bindgen::prelude::*;

use js_sys::{ArrayBuffer, Uint8Array};
use web_sys::{
    Blob, CanvasRenderingContext2d, File, FileReader, HtmlCanvasElement, HtmlImageElement,
};

#[wasm_bindgen]
extern "C" {
    pub type Buffer;

    #[wasm_bindgen(method, getter)]
    fn buffer(this: &Buffer) -> ArrayBuffer;

    #[wasm_bindgen(method, getter, js_name = byteOffset)]
    fn byte_offset(this: &Buffer) -> u32;

    #[wasm_bindgen(method, getter)]
    fn length(this: &Buffer) -> u32;

}

#[wasm_bindgen(start)]
fn main() -> Result<(), String> {
    Ok(())
}

#[wasm_bindgen]
pub struct FileHolder {
    internal: Blob,
}

#[wasm_bindgen]
impl FileHolder {
    #[wasm_bindgen(constructor)]
    pub fn new(internal: Blob) -> Self {
        Self { internal }
    }

    pub fn render(&self) {
        let reader = FileReader::new().unwrap();
        reader.read_as_data_url(&self.internal).unwrap();
        let onloadend: Closure<dyn Fn(web_sys::Event)> =
            Closure::new(render_image_as_data_url() as Box<dyn Fn(web_sys::Event)>);
        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
        onloadend.forget();
    }
}

fn render_image_as_data_url() -> Box<dyn Fn(web_sys::Event)> {
    Box::new(|e: web_sys::Event| {
        let filereader = e.target().unwrap().dyn_into::<FileReader>().unwrap();
        let vec = filereader.result().unwrap();
        let string: String = serde_wasm_bindgen::from_value(vec).unwrap();

        let document = web_sys::window().unwrap().document().unwrap();

        let _ctx = document
            .query_selector("canvas")
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let image_elem = document
            .create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        image_elem.set_src(&string);

        document.body().unwrap().append_child(&image_elem).unwrap();
    })
}
