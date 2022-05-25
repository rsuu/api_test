#curl -F name="@test.jpg" https://telegra.ph/upload

post-data() {
    curl -X POST \
        -H 'Content-Type: application/json' \
        -d '{
          "access_token": "<token>",
          "title": "sss",
          "author_name":"test",
          "content": [{"tag":"p","children":["Hello,+world!"]}, {"tag":"img","attrs":{"src":"https://telegra.ph/file/<id>.jpg"}}, {"tag":"img","attrs":{"src":"https://telegra.ph/file/<id>.jpg"}}],
          "return_content":"true"
          }' \
        https://api.telegra.ph/createPage

}

post-data
