#!/bin/sh

export API_TOKEN=FtpwGAy9ndcLXLUlH7i96rgXLgi2SzEdym2caXEsNUI=
export TEAM_ID=97
export OUTPUT='@output.json'

echo curl --user :$API_TOKEN -X POST -H "Content-Type: application/json" \
        -d $OUTPUT \
        https://davar.icfpcontest.org/teams/$TEAM_ID/solutions

curl --user :$API_TOKEN -X POST -H "Content-Type: application/json" \
        -d $OUTPUT \
        https://davar.icfpcontest.org/teams/$TEAM_ID/solutions

