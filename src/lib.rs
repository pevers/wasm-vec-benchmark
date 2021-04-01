use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TestData {
    contents: Vec<u16>
}

// Simulate generating some data
pub fn generate_test_data() -> TestData {
    let mut contents = Vec::new();
    for k in 0..50000 {
        contents.push(k);
    }
    TestData {
        contents
    }
}
#[wasm_bindgen]
pub struct GeneratedStream {
    offset: *const u16,
    size: usize
}

#[wasm_bindgen]
pub fn get_content_stream() -> GeneratedStream {
    let test_data = generate_test_data();
    GeneratedStream {
        offset: test_data.contents.as_ptr(),
        size: test_data.contents.len()
    }
}

#[wasm_bindgen]
pub fn get_content() -> Vec<u16> {
    let test_data = generate_test_data();
    test_data.contents
}
