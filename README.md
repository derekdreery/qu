`qu` - has opinions on CLI programs so you don't have to!

# Example

```rust
use qu::ick_use::*;
use std::path::PathBuf;

// This struct must not contain fields called `verbose` or `quiet` as these are used by `qu`.
#[derive(Debug, Parser)]
struct Opt {
    #[clap(parse(from_os_str))]
    file_name: PathBuf,
}

// This function must contain exactly one argument that implements `clap::Parser`, and should return
// `qu::ick_use::Result` (although in reality your selected return type is ignored and
// `qu::ick_use::Result` is always used). The body of th method is copied verbatim.
#[qu::ick]
fn main(opt: Opt) -> Result {
    log::warn!("you'll see this unless you do -q");
    log::info!(
        "(use -v to get info) selected file: {}",
        opt.file_name.display()
    );
    Ok(())
}
```

# Description

This crate contains the macro `qu::ick` that sets up argument parsing, logging and error
handling with minimal boilerplate. It can do this because it decides for you what your
configuration should be. If you need a different configuration I'd recommend copying out
the generated code from the derive macro and tweaking it according to your needs.

# Acknowledgements

This crate is based on `quicli` and most of the ideas are directly copied from that crate.
`quicli` may be an alternative if you don't like the opinions this crate has.
