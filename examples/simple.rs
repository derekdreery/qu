use qu::ick_use::*;
use std::path::PathBuf;

// This struct must not contain fields called `verbose` or `quiet` as these are used by `qu`.
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    file_name: PathBuf,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    log::warn!("you'll see this unless you do -q");
    log::info!(
        "(use -v to get info) selected file: {}",
        opt.file_name.display()
    );
    Ok(())
}
