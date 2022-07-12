
use crate::config::ConfigStack;

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
