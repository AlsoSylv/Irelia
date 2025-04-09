#[cfg(any(
    all(feature = "__hyper_nativetls", feature = "__hyper_rustls"),
    all(feature = "__reqwest_nativetls", feature = "__reqwest_rustls"),
    all(feature = "ws_rustls", feature = "ws_nativetls"),
))]
compile_error!("NativeTls and Rustls cannot be used together");

#[cfg(not(any(
    feature = "__hyper_rustls",
    feature = "__reqwest_rustls",
    feature = "__hyper_nativetls",
    feature = "__reqwest_nativetls",
    feature = "ws_rustls",
    feature = "ws_nativetls",
)))]
compile_error!("You need to either enable the `nativetls` or `rustls` feature");

#[cfg(any(
    feature = "__hyper_nativetls",
    feature = "__reqwest_nativetls",
    feature = "ws_nativetls"
))]
mod nativetls;
#[cfg(any(
    feature = "__hyper_rustls",
    feature = "__reqwest_rustls",
    feature = "ws_rustls"
))]
mod rustls;

#[cfg(any(
    feature = "__hyper_nativetls",
    feature = "__reqwest_nativetls",
    feature = "ws_nativetls"
))]
pub use nativetls::*;
#[cfg(any(
    feature = "__hyper_rustls",
    feature = "__reqwest_rustls",
    feature = "ws_rustls"
))]
pub use rustls::*;
