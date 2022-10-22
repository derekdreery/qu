//! `qu` has opinions on CLI programs so you don't have to! It uses...
//!
//!  - `clap` for argument parsing,
//!  - `tracing` for logging infra,
//!  - `tokio` for async.
//!
//! Both `tokio` and `clap` usage are optional. If you don't want to use `tracing` either than you
//! don't need this crate - just do `fn main() { .. }`.
//!
//! # Examples
//!
//! ```rust
//! use qu::ick_use::*;
//! use std::path::PathBuf;
//!
//! // This struct must not contain fields called `verbose` or `quiet` as these are used by `qu`.
//! #[derive(Debug, Clap)] // can use `clap::Parser` instead of `Clap`.
//! struct Opt {
//!     file_name: Option<PathBuf>,
//! }
//!
//! // This function must contain exactly one argument that implements `clap::Parser`, and should return
//! // `qu::ick_use::Result` (although in reality your selected return type is ignored and
//! // `qu::ick_use::Result` is always used). The body of th method is copied verbatim.
//! #[qu::ick]
//! fn main(opt: Opt) -> Result {
//!     log::warn!("you'll see this unless you do -q");
//!     log::info!(
//!         "(use -v to get info) selected file: {:?}",
//!         opt.file_name
//!     );
//!     Ok(())
//! }
//! ```
//!
//! Having arguments is optional - if you don't use them you'll still get `-h`, `-v`, and `-q`.
//!
//! ```rust
//! use qu::ick_use::Result;
//!
//! #[qu::ick]
//! fn main() -> Result {
//!     log::info!("wooooo");
//!     Ok(())
//! }
//! ```
//!
//! # Description
//!
//! This crate contains the macro `qu::ick` that sets up argument parsing, logging and error
//! handling with minimal boilerplate. It can do this because it decides for you what your
//! configuration should be. If you need a different configuration I'd recommend copying out
//! the generated code from the derive macro and tweaking it according to your needs.
//!
//! # Acknowledgements
//!
//! This crate is based on `quicli` and most of the ideas are directly copied from that crate.
//! `quicli` may be an alternative if you don't like the opinions this crate has.

pub mod ick_use {
    pub use ::anyhow::{bail, ensure, format_err, Context as _, Error};
    pub use ::clap::{self, Parser as Clap};
    pub use ::log;

    /// Like `anyhow::Result`, but defaults the `Ok` case to `()`.
    ///
    /// You can use this as a replacement for `std::result::Result` as it functions exactly the
    /// same when supplied with 2 type arguments.
    pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;
}

#[doc(hidden)]
pub use ::env_logger;
pub use ::qu_derive::ick;
