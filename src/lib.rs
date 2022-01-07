pub mod ick_use {
    pub use ::anyhow::{bail, ensure, format_err, Context as _, Error};
    pub use ::log;
    // You currently have to include clap directly in your Cargo.toml because it uses a direct
    // path internally.
    pub use ::clap::Parser;

    /// Like `anyhow::Result`, but defaults the `Ok` case to `()`.
    ///
    /// You can use this as a replacement for `std::result::Result` as it functions exactly the
    /// same when supplied with 2 type arguments.
    pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;
}

#[doc(hidden)]
pub use ::env_logger;
pub use ::qu_derive::ick;
