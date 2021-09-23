struct Key {
    short: String,
    long: Vec<char>,
}

impl Key{
    fn new(short:String, word:String) -> Key{
        let mut long_key = vec!();
        for i in 0..word.len() {
            long_key.push(word.chars()
                              .collect::<Vec<char>>()[i % short.len()])
        }
        Key {
            short,
            long: long_key
        }
    }
}

fn main() {
    let word = String::from("hallo welt").trim()
                                         .to_ascii_lowercase()
                                         .replace(" ", "")
                                         .to_string();
    let key = Key::new(String::from("ab"), word);
    println!("{:?}", key.long)
}
