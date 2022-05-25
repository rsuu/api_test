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
