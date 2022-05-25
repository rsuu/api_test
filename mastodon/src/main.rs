use mastodon::api;
fn main() {
    let info = api::Info::new(
        "_s1XtYzhXG8k_1BtPNelb3iukE2qa38WEgk4Mod_8x4",
        "v9mJZDtC5kYrxW5PDJo8pNmheZBRYgauopkDG78xL3A",
        "JTPartcMc0pJJY84mukf2RRs3gDMYMZmPIl9Rd_12V8",
        "urn:ietf:wg:oauth:2.0:oob",
        "https://mastodon.social",
    );

    let files = api::PostFile::new(&info);
    let post = api::PostData::new(&info, &[], "Bearer", "test", true);

    println!("{:#?}", post);
}
