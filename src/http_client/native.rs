//! HTTP Client adapter for architecture-native libraries.

#[cfg(all(feature = "curl-client", not(target_arch = "wasm32")))]
pub use super::isahc::IsahcClient as NativeClient;

#[cfg(all(feature = "wasm-client", target_arch = "wasm32"))]
pub use super::wasm::WasmClient as NativeClient;
