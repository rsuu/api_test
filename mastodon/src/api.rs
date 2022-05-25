/*
Client_key=
Client_secret=
Token=''
Redirect_uri='urn:ietf:wg:oauth:2.0:oob'
Web='https://mastodon.social'

get-media-id() {
    curl -X POST \
        -F "access_token=<Token>" \
        -F 'file=@test.jpg' \
        $Web/api/v1/media
}

post-data() {
    curl -X POST \
        -H 'Content-Type: application/json' \
        -d '{"access_token": "<Token>",
          "token_type": "Bearer",
          "status":"test",
          "media_ids": [<media-id>],
          "visibility":"private" }' \
        $Web/api/v1/statuses

}
*/

#[derive(Debug, Clone)]
pub struct PostData<'a> {
    pub info: &'a Info,
    pub media_ids: &'a [&'a str],
    pub status: &'a str,
    pub token_type: &'a str,
    pub api_uri: String,
    pub is_private: bool,
}

#[derive(Debug, Clone)]
pub struct PostFile<'a> {
    pub info: &'a Info,
    pub api_uri: String,
}

#[derive(Debug, Clone)]
pub struct Info {
    pub token: String,
    pub client_key: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub server_uri: String,
}

impl<'a> PostData<'a> {
    pub fn new(
        info: &'a Info,
        media_ids: &'a [&'a str],
        token_type: &'a str,
        status: &'a str,
        is_private: bool,
    ) -> Self {
        let api_uri = format!("{}/api/v1/statuses", info.server_uri);
        Self {
            info,
            api_uri,
            media_ids,
            status,
            token_type,
            is_private,
        }
    }
}

impl<'a> PostFile<'a> {
    pub fn new(info: &'a Info) -> Self {
        let api_uri = format!("{}/api/v1/media", info.server_uri);
        Self { info, api_uri }
    }
}

impl Info {
    pub fn new(
        token: &str,
        client_key: &str,
        client_secret: &str,
        redirect_uri: &str,
        server_uri: &str,
    ) -> Self {
        Self {
            token: token.to_string(),
            client_key: client_key.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            server_uri: server_uri.to_string(),
        }
    }
}
