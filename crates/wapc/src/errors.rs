//! Library-specific error types and utility functions

/// Error type for waPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Error returned when waPC can't find one of the waPC-protocol functions.
  #[error("No such function in Wasm module")]
  NoSuchFunction(String),
  /// I/O related error.
  #[error("I/O Error: {0}")]
  IO(#[from] std::io::Error),
  /// Miscellaneous error.
  #[error("WebAssembly failure: {0}")]
  WasmMisc(String),
  /// Error during a host call.
  #[error("Error during host call: {0}")]
  HostCallFailure(Box<dyn std::error::Error + Sync + Send>),
  /// Context creation failed.
  #[error("Context creation failed: {0}")]
  Context(String),
  /// Initialization Failed.
  #[error("Initialization failed: {0}")]
  InitFailed(String),
  /// Could not create unique execution context for function call.
  #[error("Could not create new execution context: {0}")]
  NewContext(String),
  /// Error during a guest call.
  #[error("Guest call failure: {0}")]
  GuestCallFailure(String),
  /// Error occurred while swapping out one module for another.
  #[error("Module replacement failed: {0}")]
  ReplacementFailed(String),
  /// Error originating from a WASM Engine provider.
  #[error("WASM Provider failure: {0}")]
  ProviderFailure(Box<dyn std::error::Error + Sync + Send>),
  /// General errors.
  #[error("General: {0}")]
  General(String),
}

#[cfg(test)]
mod tests {
  #[allow(dead_code)]
  fn needs_sync_send<T: Send + Sync>() {}

  #[test]
  fn assert_sync_send() {
    needs_sync_send::<super::Error>();
  }
}
