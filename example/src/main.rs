use qu::ick_use::*;
use std::path::PathBuf;

// This struct must not contain fields called `verbose` or `quiet` as these are used by `qu`.
#[derive(Debug, Clap)]
struct Opt {
    file_name: PathBuf,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    event!(Level::WARN, "you'll see this unless you do -qq");
    event!(
        Level::INFO,
        "selected file: {} (by default, use -q to hide info)",
        opt.file_name.display()
    );
    event!(
        Level::TRACE,
        "you'll only see this if you do -vv (by default)"
    );
    Ok(())
}
