// A small program to de/en-crypt with the vigenere procedure.
//
// Usage:
//  - For interactive use: 
// ```shell
//  $ cargo run
// ```
//
//  - For noninteractive use: 
// ```shell
//  $ cargo run <encrypt/decrypt> <key> <text>
// ```
//
// by lxgr <lxgr@protonmail.com>

use std::env;
use vigenere::*;

struct Key {
    // Key class
    long: Vec<char>,
}

impl Key {
    fn new(short:&str, word:&str) -> Key{
        // generates a new Key
        let mut long_key = vec!();
        for i in 0..word.len() {
            long_key.push(short.chars()
                               .collect::<Vec<char>>()[i % short.len()])
        }
        Key {
            long: long_key
        }
    }
    fn to_anti_key(&mut self) {
        // Makes the Key an anti key
        let mut new_long = vec!();
        for k in self.long.iter() {
            new_long.push(abc()[(26-abc().iter()
                                         .position(|&r| r == *k)
                                         .unwrap()) % 26])
        }
        self.long = new_long;
    }
}

fn encrypt(nums:&Vec<u32>, key:Key) -> Vec<u32>{
    // Encrypts a Vec<u32>
    let mut gt_nums = vec!();
    for (num, k) in nums.iter().zip(key.long.iter()) {
        gt_nums.push(get_modulo(*num, abc().iter()
                                .position(|&r| r == *k)
                                .unwrap() as u32));
    }
    gt_nums
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 4 && args.len() != 1 {
        panic!("Error: This program needs to have 3 or 0 arguments, not {}!",
               args.len()-1);
    }

    let en:Crypt = {
        if args.len() == 1 {
            loop{
                // Gets information about process
                break match &read_from_input("[E]ncrypt or [D]ecrypt?")
                                            .to_ascii_lowercase()[..]{
                    "e" => Crypt::En,
                    "d" => Crypt::De,
                    _ => continue,
                };
            }
        } else {
            match &args[1].trim().to_ascii_lowercase()[..]{
                "encrypt" => Crypt::En,
                "decrypt" => Crypt::De,
                _ => panic!("Invalid en/decryption operator")
            }
        }
    };
    
    let mut key_list = vec!();
    if args.len() == 1 {
        for text in ["text", "key word"]{
            key_list.push(
                loop{
                    // Gets text input, that should be decrypted 
                    let kt = read_from_input(&format!("Enter {}:", text)[..])
                                               .replace(" ", "")
                                               .to_ascii_lowercase();
                    if !all_in_abc(&kt){
                    continue;
                    }
                    break kt
                }
            );
        }
    } else {
        for i in 0..2 {
            key_list.push(
                {
                    let kt = args[2 + i].replace(" ", "").to_ascii_lowercase();
                    if !all_in_abc(&kt){
                        panic!("The text is only allowed to contain characters and spaces")
                        }   
                    kt
                }
            );
        }
    };

    let kt = &key_list[0][..];
    let mut key = Key::new(&key_list[1][..], kt);
    match en {
        Crypt::De => key.to_anti_key(),
        Crypt::En => {}
    }
    let nums = string_to_nums(kt);
    let new_nums = encrypt(&nums, key);

    println!("{}", nums_to_string(new_nums));
}
