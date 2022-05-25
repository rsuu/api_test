#!/bin/bash
echo -e "\n\e[38;5;146m      ,.....,\e[0m"
echo -e "\e[38;5;146m   pd\"\"\"\`\`\`\"\"\"bq\e[0m"
echo -e "\e[38;5;146m  6P           YA\e[0m"
echo -e "\e[38;5;146m 6M'  \e[0mdMpMMMo.\e[38;5;146m  \`Mb\e[0m  ,pW\"Wq.  dMMMMAo. dM    MF"
echo -e "\e[38;5;146m MN   \e[0mMM    MM\e[38;5;146m   8M\e[0m 6W'   \`Wb MM   \`Wb  VA  ,V"
echo -e "\e[38;5;146m YM.  \e[0mMM    MM\e[38;5;146m  ,M9\e[0m YA.   ,A9 MM   ,AP   VVV\""
echo -e "\e[38;5;146m  Mb  \e[0mJM    MM\e[38;5;146m  dM\e[0m   \`Ybmd9'  MMbmmd'    ,V"
echo -e "\e[38;5;146m   Ybq,______,pdY\e[0m             MM        ,V"
echo -e "\e[38;5;146m     \"\"\"\`\`\`\`\"\"\"\e[0m               JM       Ob\" \e[38;5;146mv\e[0m0.1"
echo -e "\e[0m"

# Make sure we have the required binaries
has_curl=`which curl 2>/dev/null || echo false`
has_jq=`which jq 2>/dev/null || echo false`
has_aria2=`which aria2c 2>/dev/null || echo false`

if [ $has_curl = "false" ]; then
	echo -e " ! Error: Missing curl binary\n"
	exit 1
fi

if [ $has_jq = "false" ]; then
	echo -e " ! Error: Missing jq binary\n"
	exit 1
fi

# Make sure the supplied URL looks valid
if [ "$(grep -Poc 'nopy.to\/([a-zA-Z0-9]{8})\/(.*?)$' <<< $1)" -lt 1 ]; then
	echo -e " ! Error: Only nopy.to URLs are supported\n"
	exit 1
fi

# Parse the code and file from URL
code=`echo $1 | sed -n 's/.*nopy.to\/\([a-zA-Z0-9]\{8\}\)\/.*/\1/p'`
file=`echo $1 | sed -n 's/.*nopy.to\/[a-zA-Z0-9]\{8\}\/\(.*\)\/\?/\1/p'`

# Fetch the session
echo -e " Fetching session ...\n"
sessionreq=`curl -s --data-urlencode "code=$code" --data-urlencode "file=$file" -X POST https://data.nopy.to/file`

if [ "$(echo $sessionreq | jq -r '.status')" != "ok" ]; then
	echo -e " ! Error: Session request failed\n"
	exit 1
fi

if [ "$(echo $sessionreq | jq -r '.msg.error_fatal')" != "false" ]; then
	echo -e " ! Error: Nopy is having technical issues\n"
	exit 1
fi

if [ -f "$(echo $sessionreq | jq -r '.msg.filename')" ]; then
	echo -e " ! Error: File \"$(echo $sessionreq | jq -r '.msg.filename')\" already exists\n"
	exit 1
fi

# Fetch the download URL
echo -e " Requesting download ticket ...\n"
downloadreq=`curl -s --data-urlencode "code=$code" --data-urlencode "fid=$(echo $sessionreq | jq -r '.msg.fid')" --data-urlencode "request=$(echo $sessionreq | jq -r '.msg.request')" --data-urlencode "session=$(echo $sessionreq | jq -r '.msg.session')" -X POST https://data.nopy.to/download`

if [ "$(echo $downloadreq | jq -r '.status')" != "ok" ]; then
	echo -e " ! Error: Download request failed\n"
	exit 1
fi

# Finally, download the file
echo -e " Downloading \"$file\" from: $(echo $downloadreq | jq -r '.msg.server') ..."

if [ $has_aria2 = "false" ]; then
	echo -e ""
	curl -J -O "$(echo $downloadreq | jq -r '.msg.download')"
else
	aria2c -x 4 --summary-interval=0 --auto-file-renaming=false "$(echo $downloadreq | jq -r '.msg.download')"
fi

echo -e "\n Finished!\n"

