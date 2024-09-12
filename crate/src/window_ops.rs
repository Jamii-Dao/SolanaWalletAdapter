use wasm_bindgen_futures::wasm_bindgen::JsValue;
use web_sys::{js_sys::Object, Document, Window};

use crate::{WalletAdapterError, WalletAdapterResult};

/// Operations on a browser window.
/// `Window` and `Document` object must be present otherwise
/// an error is thrown.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WindowOps {
    window: Window,
    document: Document,
}

impl WindowOps {
    /// Get the `Window` and `Document` object in the current browser window
    pub fn new() -> WalletAdapterResult<'static, Self> {
        let window = if let Some(window) = web_sys::window() {
            window
        } else {
            return Err(WalletAdapterError::MissingAccessToBrowserWindow);
        };

        let document = if let Some(document) = window.document() {
            document
        } else {
            return Err(WalletAdapterError::MissingAccessToBrowserDocument);
        };

        Ok(Self { window, document })
    }

    /// Get an entry in the `Window` object
    pub fn get_entry(&self, property: &str) -> Option<Object> {
        self.window.get(property)
    }

    /// Convert as [JsValue](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html) of
    /// into an [Option] where `undefined` or `null` is converted to an `Option::None`
    pub fn as_option(value: &JsValue) -> Option<&JsValue> {
        if value.is_null() || value.is_undefined() {
            return Option::None;
        }

        Some(value)
    }
}
