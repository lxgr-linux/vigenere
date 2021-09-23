struct Key {
    short: String,
    long: Vec<char>,
}

enum Crypt{
    // Crypt enum
    En,
    De,
}

fn abc() -> Vec<char>{
    // Returns the an alphabetic list
    String::from("abcdefghijklmnopqrstuvwxyz")
                                .chars().collect()
}

fn all_in_abc(kt:&str) -> bool{
    // Checks if all chars in string are actually in the alphabet
    let abc = abc();
    for cha in kt.chars(){
        if !abc.contains(&cha){
            return false;
        }
    }
    true
}

fn get_modulo(x:u32, s:u32, t:u32) -> u32{
    // Encrypts a number
    (x*s+t) % 26
}

fn string_to_nums(s:&str) -> Vec<u32>{
    // Converts a String to a Vec<u32>
    let mut list = vec!();
    let abc:Vec<char> = abc();
    
    for cha in s.chars(){
        list.push(abc.iter()
                  .position(|&r| cha == r)
                  .unwrap() as u32);
    }
    list
}

fn nums_to_string(nums:Vec<u32>) -> String{
    // Converts a Vec<u32> to a String
    let mut s = String::new();
    let abc:Vec<char> = abc();
    
    for num in nums.iter(){
        s.push(abc[*num as usize]);
    }
    s
}

impl Key {
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
