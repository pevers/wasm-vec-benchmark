#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use wasm_bindgen::prelude::*;
pub struct TestData {
    contents: Vec<u16>,
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for TestData {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(8u32);
        inform(84u32);
        inform(101u32);
        inform(115u32);
        inform(116u32);
        inform(68u32);
        inform(97u32);
        inform(116u32);
        inform(97u32);
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for TestData {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::WasmRefCell;
        Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for TestData {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<TestData>;
        assert_not_null(ptr);
        let js = Box::from_raw(ptr);
        (*js).borrow_mut();
        js.into_inner()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::__rt::core::convert::From<TestData> for wasm_bindgen::JsValue {
    fn from(value: TestData) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __wbg_testdata_new(_: u32) -> u32 {
            {
                ::std::rt::begin_panic("cannot convert to JsValue outside of the wasm target")
            }
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_testdata_new(ptr),
            )
        }
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::RefFromWasmAbi for TestData {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::Ref<'static, TestData>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<TestData>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::RefMutFromWasmAbi for TestData {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RefMut<'static, TestData>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<TestData>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow_mut()
    }
}
impl wasm_bindgen::convert::OptionIntoWasmAbi for TestData {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
impl wasm_bindgen::convert::OptionFromWasmAbi for TestData {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
pub fn generate_test_data() -> TestData {
    let mut contents = Vec::new();
    for k in 0..50000 {
        contents.push(k);
    }
    TestData { contents }
}
pub struct GeneratedStream {
    offset: *const u16,
    size: usize,
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for GeneratedStream {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(15u32);
        inform(71u32);
        inform(101u32);
        inform(110u32);
        inform(101u32);
        inform(114u32);
        inform(97u32);
        inform(116u32);
        inform(101u32);
        inform(100u32);
        inform(83u32);
        inform(116u32);
        inform(114u32);
        inform(101u32);
        inform(97u32);
        inform(109u32);
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for GeneratedStream {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::WasmRefCell;
        Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for GeneratedStream {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<GeneratedStream>;
        assert_not_null(ptr);
        let js = Box::from_raw(ptr);
        (*js).borrow_mut();
        js.into_inner()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::__rt::core::convert::From<GeneratedStream> for wasm_bindgen::JsValue {
    fn from(value: GeneratedStream) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __wbg_generatedstream_new(_: u32) -> u32 {
            {
                ::std::rt::begin_panic("cannot convert to JsValue outside of the wasm target")
            }
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_generatedstream_new(ptr),
            )
        }
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::RefFromWasmAbi for GeneratedStream {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::Ref<'static, GeneratedStream>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<GeneratedStream>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::RefMutFromWasmAbi for GeneratedStream {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RefMut<'static, GeneratedStream>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<GeneratedStream>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow_mut()
    }
}
impl wasm_bindgen::convert::OptionIntoWasmAbi for GeneratedStream {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
impl wasm_bindgen::convert::OptionFromWasmAbi for GeneratedStream {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
pub fn get_content_stream() -> GeneratedStream {
    let test_data = generate_test_data();
    GeneratedStream {
        offset: test_data.contents.as_ptr(),
        size: test_data.contents.len(),
    }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
pub extern "C" fn __wasm_bindgen_generated_get_content_stream(
) -> <GeneratedStream as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret = { get_content_stream() };
    <GeneratedStream as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn get_content() -> Vec<u16> {
    let test_data = generate_test_data();
    test_data.contents
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
pub extern "C" fn __wasm_bindgen_generated_get_content(
) -> <Vec<u16> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret = { get_content() };
    <Vec<u16> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
