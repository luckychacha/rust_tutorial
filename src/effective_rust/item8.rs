#[derive(Debug, Default)]
struct BaseDetails {
    given_name: String,
    preferred_name: Option<String>,
    middle_name: Option<String>,
    family_name: String,
    mobile_phone_e164: Option<String>,
}

// ...
fn test() {
    let dizzy = BaseDetails {
        given_name: "Dizzy".to_owned(),
        preferred_name: None,
        middle_name: None,
        family_name: "Mixer".to_owned(),
        mobile_phone_e164: None,
    };
    let dizzy = BaseDetails {
        given_name: "Dizzy".to_owned(),
        family_name: "Mixer".to_owned(),
        ..Default::default()
    };
}
