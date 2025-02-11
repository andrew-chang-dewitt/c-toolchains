use anyhow::anyhow;

mod app;
mod logging;

use app::App;

fn main() -> anyhow::Result<()> {
    let app = App::new().map_err(|e| anyhow!("Error: {e:#?}"))?;
    app.run()
}
