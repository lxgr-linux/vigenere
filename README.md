# Vigenere
A small program to de/en-crypt with the vigenere procedure.

## Usage
- For interactive use: 
```shell
 $ cargo run
```

- For noninteractive use: 
```shell
 $ cargo run <encrypt/decrypt> <key> <text>
```

Note: Just ascii characters and spaces are allowed in the text!

## Example

This encrypts the text "Hello world" with the key "hello":
```shell
 $ cargo run encrypt hello "Hello world"
```

## License
This software is licensed under the GPLv3 license.
