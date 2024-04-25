use base64::{engine::general_purpose, Engine as _};
use wasm_bindgen::prelude::*;

use image::io::Reader;
use image::ImageFormat;
use js_sys::{ArrayBuffer, Uint8Array};
use std::io::Cursor;
use web_sys::{Blob, FileReader, HtmlImageElement};

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
    blob: Option<Blob>,
}

#[wasm_bindgen]
impl FileHolder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        web_sys::console::log_1(&JsValue::from_str("Hello world! This is WASM speaking."));
        Self { blob: None }
    }

    pub fn set_blob(&mut self, blob: Blob) {
        self.blob = Some(blob);
    }

    pub fn render_with_blurry(&mut self, value: f32) {
        let closure = Closure::new(render_image_with_blurriness(value) as Box<dyn FnMut(JsValue)>);
        let _promise = self.blob.as_mut().unwrap().array_buffer().then(&closure);

        closure.forget();
    }

    pub fn render(&mut self) {
        let reader = FileReader::new().unwrap();
        let blob = self.blob.as_mut().unwrap();
        reader.read_as_data_url(blob).unwrap();
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

        let image_elem = document
            .create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        image_elem.set_src(&string);

        document.body().unwrap().append_child(&image_elem).unwrap();
    })
}

fn render_image_with_blurriness(blur_value: f32) -> Box<dyn FnMut(JsValue)> {
    Box::new(move |e: wasm_bindgen::JsValue| {
        let uint8 = Uint8Array::new(&e).to_vec();

        let rdr = Reader::new(Cursor::new(uint8))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        rdr.unsharpen(blur_value, 100000);
        rdr.grayscale();

        let mut vec = Cursor::new(Vec::new());
        rdr.write_to(&mut vec, ImageFormat::Png).unwrap();
        let base64 = general_purpose::STANDARD.encode(vec.into_inner());
        let string = format!("data:image/png;base64, {base64}");

        let document = web_sys::window().unwrap().document().unwrap();

        let image_elem = document
            .create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        image_elem.set_src(&string);

        document.body().unwrap().append_child(&image_elem).unwrap();
    })
}
