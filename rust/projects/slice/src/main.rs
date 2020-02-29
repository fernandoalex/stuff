fn main() {
    let mut s = String::from("Hello World");
    let word = first_word(&s);
    s.clear();
}

// this func is grabbing a reference from a string and return a usize
// the idea here is to turn the index of the string that contains a white space
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
