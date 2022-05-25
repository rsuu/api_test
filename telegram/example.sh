#!/bin/bash
curl -F document=@"path/to/some.file" https://api.telegram.org/bot<token>/sendDocument?chat_id=<chat_id>

