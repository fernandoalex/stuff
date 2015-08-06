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
