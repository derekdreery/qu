use qu::ick_use::*;
use std::path::PathBuf;

// This struct must not contain fields called `verbose` or `quiet` as these are used by `qu`.
#[derive(Debug, Clap)]
struct Opt {
    file_name: PathBuf,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    log::warn!("you'll see this unless you do -qq");
    log::info!(
        "selected file: {} (by default, use -q to hide info)",
        opt.file_name.display()
    );
    log::trace!("you'll only see this if you do -vv (by default)");
    Ok(())
}
