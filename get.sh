#!/bin/sh

export API_TOKEN=FtpwGAy9ndcLXLUlH7i96rgXLgi2SzEdym2caXEsNUI=
export TEAM_ID=97
export OUTPUT='@output.json'

echo rm metadata.json
rm metadata.json

curl --user :$API_TOKEN -X GET https://davar.icfpcontest.org/teams/$TEAM_ID/solutions > metadata.json

echo python parse.py
python parse.py

