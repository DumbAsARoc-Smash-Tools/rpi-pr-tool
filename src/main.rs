mod ezpr_file_format;
mod oauth;
mod settings;

slint::include_modules!();

use std::sync::Mutex;
use tokio::task::JoinHandle;

use std::io::Write;

use lazy_static::lazy_static;
use oauth2::TokenResponse;
use slint::ComponentHandle;

lazy_static! {
    static ref OAUTH_WEBSERVER_HANDLE: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
}


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

    let mainwin_weak = mainwin.as_weak();
    mainwin.on_oauth_start_auth_button_clicked(move || {
        // let mut webserver_lock = OAUTH_WEBSERVER_HANDLE.lock().unwrap();
        // if webserver_lock.is_some() {
        //     println!("Webserver is runing currently");
        //     return;
        // }

        // let handle = tokio::spawn(async move {
        //     let code = oauth::StartGGOAuth::get_oauth_token(8080).await;
        //     println!("Token: {}", code.clone().unwrap().access_token().secret());

        //     // let s = settings::RPIPRSettings::new()
        //     //     .set_token(code.clone());
        //     // match serialize(&s) {
        //     //     Err(e) => {
        //     //         println!("Error: {}", e);
        //     //     },
        //     //     _ => {}
        //     // };
        // });

        // *webserver_lock = Some(handle);
    });

    // mainwin.on_oauth_signin_button_clicked(move || {
    //     println!("oauth button clicked");
    // });

    mainwin.on_oauth_cancel_button_clicked(move || {
        println!("oauth cancel button clicked");
    });

    mainwin.on_oauth_close_button_clicked(move || {
        println!("close window");
    });

    mainwin.run()?;

    Ok(())
}
