package main

// [{"First": "James", "Last": "Bond", "Age": 12}, {"First": "Miss", "Last": "Moneypenny", "Age": 33}]


s := `[{"First": "James", "Last": "Bond", "Age": 12}, {"First": "Miss", "Last": "Moneypenny", "Age": 33}]`

// JSON Unmarshal takes a slice of bytes, so we need to convert it

bs := []byte(s)