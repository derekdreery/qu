use qu::ick_use::*;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbosity: u16,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    log::warn!("you'll only see this if you use verbose");
    log::info!("hello info");
    Ok(())
}
