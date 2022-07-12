use std::collections::HashMap;

use crate::config::ConfigStack;
use crate::dialog_data;

#[test]
fn config_from() {
    let config_files = ["resources/mycroft.conf"];
    let cfg = ConfigStack::from(&config_files[..]);

    assert_eq!(cfg.get(&["lang"]).unwrap(), "en-us");
}

#[test]
fn config_from_vec_borrow() {
    let config_files = vec![String::from("resources/mycroft.conf")];
    let cfg = ConfigStack::from(&config_files);

    assert_eq!(cfg.get(&["lang"]).unwrap(), "en-us");
}

#[test]
fn config_from_vec() {
    let config_files = vec![String::from("resources/mycroft.conf")];
    let cfg = ConfigStack::from(config_files);

    assert_eq!(cfg.get(&["lang"]).unwrap(), "en-us");
}

#[test]
fn dialog_data_macro() {
    let data = dialog_data!([
        ("Life", "42"),
        ("Universe", "42"),
        ("Everything", "42"),
        ("Solong", "Thanks for all the fish!")]);

    assert_eq!(data["Universe"], "42");
    assert_eq!(data["Solong"], "Thanks for all the fish!");
}
