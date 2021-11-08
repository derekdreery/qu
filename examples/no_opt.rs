use qu::ick_use::*;

#[qu::ick]
fn main() -> Result {
    log::warn!("you'll see this unless you do -qq");
    Ok(())
}
