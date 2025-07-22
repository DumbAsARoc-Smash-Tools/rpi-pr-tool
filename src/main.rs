mod ezpr_file_format;
mod oauth;
mod settings;

slint::include_modules!();

use std::io::Write;

use oauth2::TokenResponse;
use slint::ComponentHandle;


fn serialize(set: &settings::RPIPRSettings) -> anyhow::Result<()> {

    let serialized = serde_json::to_string_pretty(set)?;
    let mut file = std::fs::File::create("test")?;
    write!(file, "{}", serialized)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // let set = settings::RPIPRSettings::new();
    // serialize(&set)?;

    let mainwin = match EZPRWindow::new() {
        Ok(m) => m,
        Err(e) => { return Err(anyhow::anyhow!(e)); }
    };

    // let mainwin_weak = mainwin.as_weak();
    mainwin.on_first_button_clicked(move || {
        let _handle = slint::spawn_local(async move {
            let code = oauth::StartGGOAuth::get_oauth_token(8080).await;
            println!("Token: {}", code.clone().unwrap().access_token().secret());

            let s = settings::RPIPRSettings::new()
                .set_token(code.clone());
            match serialize(&s) {
                Err(e) => {
                    println!("Error: {}", e);
                },
                _ => {}
            };
        }).unwrap();
    });

    mainwin.run()?;

    Ok(())
}
