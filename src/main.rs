mod ezpr_file_format;

slint::include_modules!();

use slint::ComponentHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let mainwin = match EZPRWindow::new() {
        Ok(m) => m,
        Err(e) => { return Err(anyhow::anyhow!(e)); }
    };

    mainwin.run()?;

    Ok(())
}
