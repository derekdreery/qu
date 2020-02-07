pub mod ick_use {
    pub use ::anyhow::Error;
    pub use ::log;
    pub use ::structopt::StructOpt;

    pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;
}

pub use ::env_logger;
pub use ::qu_derive::ick;
