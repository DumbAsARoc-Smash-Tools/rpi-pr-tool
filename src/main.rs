mod ezpr_file_format;
mod oauth;
mod settings;

slint::include_modules!();

use oauth2::TokenResponse;
use tokio::task::JoinHandle;

use std::io::Write;

use lazy_static::lazy_static;
use slint::ComponentHandle;

use std::sync::{Arc, Mutex};

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

    // let testint: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let mainwin = match EZPRWindow::new() {
        Ok(m) => m,
        Err(e) => { return Err(anyhow::anyhow!(e)); }
    };

    let mainwin_weak = mainwin.as_weak();
    // let testint_weak = Arc::downgrade(&testint);
    mainwin.on_oauth_start_auth_button_clicked(move || {
        // let testint_strong = testint_weak.
        //     upgrade().unwrap();
        // let mut testint_lock = testint_strong.
        //     lock().unwrap();
        
        // *testint_lock = 5;
        // println!("Testint: {}", *testint_lock);

        // drop(testint_lock);
        // drop(testint_strong);

        let mut webserver_lock = OAUTH_WEBSERVER_HANDLE.lock().unwrap();
        if webserver_lock.is_some() {
            println!("Webserver is runing currently");
            return;
        }

        let (url, csrf_state, client) = oauth::StartGGOAuth::get_oauth_token(8080);
        let mw_w = mainwin_weak.clone();
        let mw = mainwin_weak.unwrap();
        mw.set_oauth_hyperlink_url(url.as_str().into());

        *webserver_lock = Some(tokio::spawn(async move {
            let code = oauth::StartGGOAuth::server_code_exchange(
                8080,
                csrf_state,
                client).await;

            match code {
                None => {
                    println!("Failed to get Start.GG Code.");
                },
                Some(code) => {
                    let (tag, url) = oauth::get_startgg_user_and_profile_icon(
                        code.access_token().secret())
                        .await.unwrap();

                    let (has_pfp, img_path) = match url {
                        Some(u) => {
                            println!("{u}");
                            let pfp_path: std::path::PathBuf = "res/pfp.jpg".into();
                            let mut pfp_file = std::fs::File::create(&pfp_path).unwrap();
                            let bytes = reqwest::get(u).await
                                .unwrap()
                                .bytes().await
                                .unwrap();
                            pfp_file.write(&bytes).unwrap();

                            (true, Some(pfp_path))
                        },
                        None => {
                            (false, None)
                        }
                    };

                    mw_w.upgrade_in_event_loop(move |win| {
                        win.global::<StartGGState>().set_startgg_user(tag.into());
                        win.global::<StartGGState>().set_has_pfp(has_pfp);
                        if img_path.is_some() {
                            win.global::<StartGGState>().set_pfp(
                                slint::Image::load_from_path(img_path.unwrap().as_path()).unwrap()
                            );
                        }
                        win.set_oauth_step(OAuthStep::CODEOBTAINED);
                    }).unwrap();
                }
            }

            let mut webserver_lock = OAUTH_WEBSERVER_HANDLE.lock().unwrap();
            *webserver_lock = None;
        }));
    });

    let mainwin_weak = mainwin.as_weak();
    mainwin.on_oauth_hyperlink_clicked(move || {
        match open::that(mainwin_weak.unwrap().get_oauth_hyperlink_url()) {
            Ok(_) => {},
            Err(e) => {
                println!("open::this() error: {}", e);
            }
        };
    });

    // let mainwin_weak = mainwin.as_weak();
    mainwin.on_oauth_cancel_button_clicked(move || {
        let mut webserver_lock = OAUTH_WEBSERVER_HANDLE.lock().unwrap();
        if webserver_lock.is_some() {
            webserver_lock.as_mut().unwrap().abort();
            *webserver_lock = None;
        }
    });

    mainwin.on_oauth_close_button_clicked(move || {
        // This block of code is simply to
        // make sure that the webserver thread is
        // well and truly gone.
        let mut webserver_lock = OAUTH_WEBSERVER_HANDLE.lock().unwrap();
        if webserver_lock.is_some() {
            webserver_lock.as_mut().unwrap().abort();
            *webserver_lock = None;
        }
    });

    mainwin.run()?;

    Ok(())
}
