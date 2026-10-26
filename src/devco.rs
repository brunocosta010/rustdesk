//! dev&Co. branding defaults, kept in their own module so upstream merges stay clean.
//!
//! The values go into the `DEFAULT_*` tables: users can still change them, and
//! a signed `custom.txt`, if any, is applied afterwards and wins.

use base::config::keys;
use hbb_common::config;

const RENDEZVOUS_SERVER: &str = "5.78.235.208";
const SERVER_KEY: &str = "3FX29EsVlS9+sqdRNMVdHznFJ1YCvWQUD9f5ytQoLTU=";

pub fn apply_defaults() {
    let mut settings = config::DEFAULT_SETTINGS.write().unwrap();
    settings
        .entry(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned())
        .or_insert_with(|| RENDEZVOUS_SERVER.to_owned());
    settings
        .entry(keys::OPTION_KEY.to_owned())
        .or_insert_with(|| SERVER_KEY.to_owned());
    drop(settings);

    config::DEFAULT_LOCAL_SETTINGS
        .write()
        .unwrap()
        .entry(keys::OPTION_THEME.to_owned())
        .or_insert_with(|| "dark".to_owned());
}
