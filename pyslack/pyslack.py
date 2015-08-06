#curl -X POST --data-urlencode 'payload={"channel": "#general", "username": "webhookbot", "text": "This is posted to #general and comes from a bot named webhookbot.", "icon_emoji": ":ghost:"}' https://hooks.slack.com/services/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

# Simple python to send msg to #generak

import sys
import urllib2
import json

url = sys.argv[1]

payload = {
    "username": "webhookbot",
    "text": sys.argv[2],
}

#Serialize obj to a JSON formatted str using this conversion table.
data = json.dumps(payload)

req = urllib2.Request(url, data)

response = urllib2.urlopen(req)
the_page = response.read()
print the_page
