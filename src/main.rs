extern crate serde_json;

use serde_json::Value as JsonValue; // Used to parse json
use std::fs::File; // Can open a file
use std::io::prelude::*; // Used with the above
use std::io; // Read userinput


fn main() {
    let mut input = String::new();
    println!("Enter path for json file to read");
    match io::stdin().read_line(&mut input) { // reads the userstring to input var
        Ok(_) => {
            input.pop(); // remove trailing newline
        },
        Err(e) => println!("Whoops, something went wrong: {}", e)
    }

    //let mut file = File::open("../json/test.json").expect("Cannot open file!");
    let mut file = File::open(&input).expect("Cannot open file!"); // open the file

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read file"); // writes the content to contents var

    let v: JsonValue = serde_json::from_str(&contents).unwrap(); // parse json
    println!("{}", v["code"]); // check if loaded correctly
}
