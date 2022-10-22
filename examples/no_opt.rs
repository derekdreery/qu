use qu::ick_use::*;

#[qu::ick]
fn main() -> Result {
    event!(Level::WARN, "you'll see this unless you do -qq");
    Ok(())
}
