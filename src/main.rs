use anyhow::Result;
use tailrsc::get_args;

#[tokio::main]
async fn main() -> Result<()> {
    let _args = get_args();
    Ok(())
}
