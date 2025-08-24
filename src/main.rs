use base64::{Engine as _, engine::general_purpose};
use std::io;
use urlencoding::{decode, encode};
use dialoguer::Select;
use htmlescape;
use jsonwebtoken::{Validation};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    custom_claim: String,
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
}

fn main() {
    let items = vec!["Base64 Decode", "Base64 Encode", "URL Decode", "URL Encode", "HTML Unescape", "HTML Escape", "Quit"];
    let selection = Select::new()
        .with_prompt("What do want to do?")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();
    match selection {
        0 => base_64_decode(),
        1 => base_64_encode(),
        2 => url_decode(),
        3 => url_encode(),
        4 => html_unescape(),
        5 => html_escape(),
        _ => (),
    }
}

fn base_64_decode() {
    let mut input_string = String::new();
    println!("Enter a Base64 encoded string to decode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = general_purpose::STANDARD_NO_PAD.decode(input_string.trim_end());
    match result {
        Ok(result_bytes) => match String::from_utf8(result_bytes) {
            Ok(result_str) => println!("Result: {}", result_str),
            Err(err) => println!("Error converting bytes to string: {}", err),
        },
        Err(err) => println!("Error decoding Base64 string: {}", err),
    }
}

fn base_64_encode() {
    let mut input_string = String::new();
    println!("Enter a string to Base64 encode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = general_purpose::STANDARD_NO_PAD.encode(input_string.trim_end());
    println!("Result: {}", result);
}

fn url_decode() {
    let mut input_string = String::new();
    println!("Enter a URL encoded string to decode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = decode(input_string.trim_end());
    match result {
        Ok(result_str) => println!("Result: {}", result_str),
        Err(err) => println!("Error decoding URL encoded string: {}", err),
    }
}

fn url_encode() {
    let mut input_string = String::new();
    println!("Enter a string to URL encode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = encode(input_string.trim_end());
    println!("Result: {}", result);
}

fn html_unescape() {
    let mut input_string = String::new();
    println!("Enter a HTML escaped string to unescape:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = htmlescape::decode_html(input_string.as_str());
    match result {
        Ok(result_str) => println!("Result: {}", result_str),
        Err(_) => println!("Error decoding HTML escaped string"),
    }
}

fn html_escape() {
    let mut input_string = String::new();
    println!("Enter a string to HTML escape:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = htmlescape::encode_attribute(input_string.as_str());
    println!("Result: {}", result);
}

// fn jwt_decode() {
//     let mut input_string = String::new();
//     let mut dec_secret = String::new();
//     println!("Enter a JWT token to decode:");
//     io::stdin()
//         .read_line(&mut input_string)
//         .expect("Failed to read line");
//     println!("Enter the JWT secret:");
//     io::stdin()
//         .read_line(&mut dec_secret)
//         .expect("Failed to read line");
//     let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
//     validation.required_spec_claims = HashSet::new();
//     validation.validate_aud = false;
//     let res = jsonwebtoken::decode::<Claims>(&input_string, &dec_secret, &validation);
//     println!("res {:?}", res);
// }
