mod ezpr_file_format;
mod oauth;
mod queries;
mod settings;

slint::include_modules!();

use oauth2::TokenResponse;
use tokio::task::JoinHandle;

use std::io::Write;

use lazy_static::lazy_static;
use slint::ComponentHandle;

use std::sync::{Arc, Mutex};

use crate::ezpr_file_format::{EZPRFile, IEZPRFile};

lazy_static! {
    static ref OAUTH_WEBSERVER_HANDLE: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load default settings location
    let app_settings = match settings::RPIPRSettings::load_from_file(None::<String>) {
        Ok(set) => set,
        Err(e) => {
            let error_window = InitializationErrorWindow::new()?;
            let error_string = format!("{}", e);
            error_window.set_error_string(error_string.into());

            // let error_window_weak = error_window.as_weak();
            error_window.on_ok_clicked(move || {
                let _ = slint::quit_event_loop();
            });
            let _ = error_window.run();
            return Ok(());
        }
    };

    let app_settings_lock = match app_settings.lock() {
        Ok(set) => set,
        Err(e) => {
            let error_window = InitializationErrorWindow::new()?;
            let error_string = format!("{}", e);
            error_window.set_error_string(error_string.into());

            // let error_window_weak = error_window.as_weak();
            error_window.on_ok_clicked(move || {
                let _ = slint::quit_event_loop();
            });
            let _ = error_window.run();
            return Ok(());
        }
    };
    if app_settings_lock.get_token().is_some() {
        println!("Logged into Start.GG!");
        println!(
            "Token: {}",
            app_settings_lock
                .get_token()
                .unwrap()
                .access_token()
                .secret()
        );
    }
    drop(app_settings_lock);

    EZPRFile::new_file("test.ezpr")?;

    let mainwin = match EZPRWindow::new() {
        Ok(m) => m,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let mainwin_weak = mainwin.as_weak();
    let settings_weak = Arc::downgrade(&app_settings);
    mainwin.on_oauth_start_auth_button_clicked(move || {
        let settings_weak = settings_weak.clone();

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
            let code = oauth::StartGGOAuth::server_code_exchange(8080, csrf_state, client).await;

            match code {
                None => {
                    println!("Failed to get Start.GG Code.");
                }
                Some(code) => {
                    let settings_token_clone = code.clone();
                    let profile_info =
                        queries::get_startgg_user_and_profile_icon(code.access_token().secret())
                            .await
                            .unwrap();

                    let (has_pfp, img_path) = match &profile_info.pfp_url {
                        Some(u) => {
                            println!("{u}");
                            let pfp_path: std::path::PathBuf = "res/pfp.jpg".into();
                            let mut pfp_file = std::fs::File::create(&pfp_path).unwrap();
                            let bytes = reqwest::get(u).await.unwrap().bytes().await.unwrap();
                            pfp_file.write(&bytes).unwrap();

                            (true, Some(pfp_path))
                        }
                        None => (false, None),
                    };

                    mw_w.upgrade_in_event_loop(move |win| {
                        win.global::<StartGGState>()
                            .set_startgg_user(profile_info.username.into());
                        win.global::<StartGGState>().set_has_pfp(has_pfp);
                        if img_path.is_some() {
                            win.global::<StartGGState>().set_pfp(
                                slint::Image::load_from_path(img_path.unwrap().as_path()).unwrap(),
                            );
                        }

                        let settings_strong = settings_weak.upgrade().unwrap();
                        let mut settings_lock = settings_strong.lock().unwrap();
                        *settings_lock = settings_lock.set_token(Some(settings_token_clone));
                        settings_lock.save_to_file().unwrap();
                        drop(settings_lock);
                        drop(settings_strong);

                        win.set_oauth_step(OAuthStep::CODEOBTAINED);
                    })
                    .unwrap();
                }
            }

            let mut webserver_lock = OAUTH_WEBSERVER_HANDLE.lock().unwrap();
            *webserver_lock = None;
        }));
    });

    let mainwin_weak = mainwin.as_weak();
    mainwin.on_oauth_hyperlink_clicked(move || {
        match open::that(mainwin_weak.unwrap().get_oauth_hyperlink_url()) {
            Ok(_) => {}
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
