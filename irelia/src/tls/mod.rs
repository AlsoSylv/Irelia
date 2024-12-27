#[cfg(all(feature = "nativetls", feature = "rustls"))]
compile_error!("NativeTls and Rustls cannot be used together");

#[cfg(not(any(feature = "nativetls", feature = "rustls")))]
compile_error!("You need to either enable the `nativetls` or `rustls` feature");

#[cfg(feature = "nativetls")]
mod nativetls;
#[cfg(feature = "rustls")]
mod rustls;

#[cfg(feature = "nativetls")]
pub use nativetls::*;
#[cfg(feature = "rustls")]
pub use rustls::*;
