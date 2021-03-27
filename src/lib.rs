pub mod ick_use {
    pub use ::anyhow::{format_err, ensure, bail, Error, Context as _};
    pub use ::log;
    pub use ::structopt::StructOpt;

    /// Like `anyhow::Result`, but defaults the `Ok` case to `()`.
    ///
    /// You can use this as a replacement for `std::result::Result` as it functions exactly the
    /// same when supplied with 2 type arguments.
    pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;
}

#[doc(hidden)]
pub use ::env_logger;
pub use ::qu_derive::ick;
