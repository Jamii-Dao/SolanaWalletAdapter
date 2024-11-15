use wasm_bindgen::JsValue;

use crate::{SemverVersion, StandardFunction, WalletError, WalletResult};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Disconnect(StandardFunction);

impl Disconnect {
    pub fn new(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        Ok(Self(StandardFunction::new(
            value,
            version,
            "disconnect",
            "standard",
        )?))
    }

    pub(crate) async fn call_disconnect(&self) -> WalletResult<()> {
        let outcome = self.0.callback.call0(&JsValue::null())?;

        let outcome = js_sys::Promise::resolve(&outcome);

        if let Some(error) = wasm_bindgen_futures::JsFuture::from(outcome).await.err() {
            let value: WalletError = error.into();
            return Err(WalletError::WalletDisconnectError(value.to_string()));
        }

        Ok(())
    }
}
