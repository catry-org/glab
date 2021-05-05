pub fn get_text(lang: &str, id: &str) -> String {
    let ctx = json_gettext::static_json_gettext_build!(
        "ko_KR",
        "ko_KR",
        "i18n/ko_KR.json",
        "en_US",
        "i18n/en_US.json"
    )
    .unwrap();

    return json_gettext::get_text!(ctx, lang, id).unwrap().to_string();
}
