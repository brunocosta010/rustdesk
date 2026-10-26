//! dev&Co. branding defaults, kept in their own module so upstream merges stay clean.
//!
//! Server and display values go into the `DEFAULT_*` tables, so users can still change
//! them; a signed `custom.txt`, if any, is applied afterwards and wins.

use base::config::keys;
use hbb_common::config;

const APP_NAME: &str = "devdesk";
const RENDEZVOUS_SERVER: &str = "5.78.235.208";
const SERVER_KEY: &str = "3FX29EsVlS9+sqdRNMVdHznFJ1YCvWQUD9f5ytQoLTU=";
// Read at build time so the password never lands in the (public) repository.
// A permanent password set locally on the machine takes precedence over it.
const PRESET_PASSWORD: Option<&str> = option_env!("DEVDESK_PRESET_PASSWORD");

pub fn apply_defaults() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
    let preset_password = PRESET_PASSWORD.filter(|p| !p.is_empty());

    let mut settings = config::DEFAULT_SETTINGS.write().unwrap();
    settings
        .entry(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned())
        .or_insert_with(|| RENDEZVOUS_SERVER.to_owned());
    settings
        .entry(keys::OPTION_KEY.to_owned())
        .or_insert_with(|| SERVER_KEY.to_owned());
    if preset_password.is_some() {
        settings
            .entry(keys::OPTION_VERIFICATION_METHOD.to_owned())
            .or_insert_with(|| "use-permanent-password".to_owned());
    }
    // Controlled side: drop the wallpaper while a session runs (restored when it ends).
    settings
        .entry(keys::OPTION_ALLOW_REMOVE_WALLPAPER.to_owned())
        .or_insert_with(|| "Y".to_owned());
    drop(settings);

    // Controlling side: "custom" quality is the only mode whose fps cap goes above 30.
    // Quality 50 is a bitrate ratio of 1.0 (balanced 0.67, best 1.5); the adaptive QoS
    // still lowers fps and bitrate when the network degrades.
    let mut display = config::DEFAULT_DISPLAY_SETTINGS.write().unwrap();
    for (k, v) in [
        (keys::OPTION_IMAGE_QUALITY, "custom"),
        (keys::OPTION_CUSTOM_IMAGE_QUALITY, "50"),
        (keys::OPTION_CUSTOM_FPS, "60"),
    ] {
        display.entry(k.to_owned()).or_insert_with(|| v.to_owned());
    }
    drop(display);

    config::BUILTIN_SETTINGS
        .write()
        .unwrap()
        .entry(keys::OPTION_HIDE_POWERED_BY_ME.to_owned())
        .or_insert_with(|| "Y".to_owned());

    if let Some(password) = preset_password {
        config::HARD_SETTINGS
            .write()
            .unwrap()
            .entry("password".to_owned())
            .or_insert_with(|| password.to_owned());
    }

    config::DEFAULT_LOCAL_SETTINGS
        .write()
        .unwrap()
        .entry(keys::OPTION_THEME.to_owned())
        .or_insert_with(|| "dark".to_owned());
}
