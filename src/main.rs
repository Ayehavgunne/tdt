use base64::{Engine as _, engine::general_purpose};
use dialoguer::Select;
use htmlescape;
use std::{io, str::FromStr};
use urlencoding::{decode, encode};

fn main() {
    let items = vec![
        "Base64 Decode",
        "Base64 Encode",
        "URL Decode",
        "URL Encode",
        "HTML Unescape",
        "HTML Escape",
        "JWT Decode",
        "Quit",
    ];
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
        6 => jwt_decode(),
        _ => (),
    }
}

fn base_64_decode() {
    let mut input_string = String::new();
    println!("Enter a Base64 encoded string to decode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = _base_64_decode(&mut input_string);
    match result {
        Ok(result_str) => println!("Result: {result_str}"),
        Err(err) => println!("{err}"),
    }
}

fn _base_64_decode(input: &mut String) -> Result<String, String> {
    let result = general_purpose::STANDARD_NO_PAD.decode(input.trim_end());
    match result {
        Ok(result_bytes) => match String::from_utf8(result_bytes) {
            Ok(result_str) => return Ok(result_str),
            Err(err) => Err(format!("Error converting bytes to string: {err}")),
        },
        Err(err) => return Err(format!("Error decoding Base64 string: {err}")),

    }
}

fn base_64_encode() {
    let mut input_string = String::new();
    println!("Enter a string to Base64 encode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = general_purpose::STANDARD_NO_PAD.encode(input_string.trim_end());
    println!("Result: {result}");
}

fn url_decode() {
    let mut input_string = String::new();
    println!("Enter a URL encoded string to decode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = decode(input_string.trim_end());
    match result {
        Ok(result_str) => println!("Result: {result_str}"),
        Err(err) => println!("Error decoding URL encoded string: {err}"),
    }
}

fn url_encode() {
    let mut input_string = String::new();
    println!("Enter a string to URL encode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = encode(input_string.trim_end());
    println!("Result: {result}");
}

fn html_unescape() {
    let mut input_string = String::new();
    println!("Enter a HTML escaped string to unescape:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = htmlescape::decode_html(input_string.as_str());
    match result {
        Ok(result_str) => println!("Result: {result_str}"),
        Err(err) => println!("Error decoding HTML escaped string: {err:?}"),
    }
}

fn html_escape() {
    let mut input_string = String::new();
    println!("Enter a string to HTML escape:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let result = htmlescape::encode_attribute(input_string.as_str());
    println!("Result: {result}");
}

fn jwt_decode() {
    let mut input_string = String::new();
    println!("Enter a JWT token to decode:");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let jwt_parts: Vec<&str> = input_string.split(".").collect();
    let header = _base_64_decode(&mut String::from_str(jwt_parts[0]).unwrap());
    let payload = _base_64_decode(&mut String::from_str(jwt_parts[1]).unwrap());
    match header {
        Ok(header_str) => println!("Header Data: {header_str}"),
        Err(err) => println!("Error decoding header: {err}"),
    }
    match payload {
        Ok(payload_str) => println!("Payload Data: {payload_str}"),
        Err(err) => println!("Error decoding payload: {err}"),
    }
}
