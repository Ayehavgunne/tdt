use base64::{Engine as _, engine::general_purpose};
use console::Term;
use htmlescape;
use std::str::FromStr;
use urlencoding::{decode, encode};
mod select;
use select::{Option, Select};
use clap::Parser;

#[derive(Parser)]
struct Args {
    tool: String,
}

fn main() {
    let args = Args::parse();
    let select = Select {
        options: vec![
            Option {
                message: "Base64 Decode",
                arg_name: Some("base64_decode"),
                handle: base_64_decode,
            },
            Option {
                message: "Base64 Encode",
                arg_name: Some("base64_encode"),
                handle: base_64_encode,
            },
            Option {
                message: "URL Decode",
                arg_name: Some("url_decode"),
                handle: url_decode,
            },
            Option {
                message: "URL Encode",
                arg_name: Some("url_encode"),
                handle: url_encode,
            },
            Option {
                message: "HTML Unescape",
                arg_name: Some("html_unescape"),
                handle: html_unescape,
            },
            Option {
                message: "HTML Escape",
                arg_name: Some("html_escape"),
                handle: html_escape,
            },
            Option {
                message: "JWT Decode",
                arg_name: Some("jwt_decode"),
                handle: jwt_decode,
            },
            Option {
                message: "Quit",
                arg_name: None,
                handle: |_, _| {},
            },
        ],
        message: "What do you want to do?",
        args: args,
        term: Term::stdout(),
    };
    let arg_selection = select.arg_match();
    match arg_selection {
        Some(option) => (option.handle)(&select.args, &select.term),
        None => select.interact(),
    }
}

fn base_64_decode(_a: &Args, t: &Term) {
    t.write_line("Enter a Base64 encoded string to decode:")
        .unwrap();
    let input = t.read_line().unwrap();
    let result = _base_64_decode(&input);
    match result {
        Ok(result_str) => t
            .write_line(format!("Result: {result_str}").as_str())
            .unwrap(),
        Err(err) => t.write_line(format!("{err}").as_str()).unwrap(),
    }
}

fn _base_64_decode(input: &str) -> Result<String, String> {
    let result = general_purpose::STANDARD_NO_PAD.decode(input);
    match result {
        Ok(result_bytes) => match String::from_utf8(result_bytes) {
            Ok(result_str) => return Ok(result_str),
            Err(err) => Err(format!("Error converting bytes to string: {err}")),
        },
        Err(err) => return Err(format!("Error decoding Base64 string: {err}")),
    }
}

fn base_64_encode(_a: &Args, t: &Term) {
    t.write_line("Enter a string to Base64 encode:").unwrap();
    let input = t.read_line().unwrap();
    let result = general_purpose::STANDARD_NO_PAD.encode(input);
    t.write_line(format!("Result: {result}").as_str()).unwrap();
}

fn url_decode(_a: &Args, t: &Term) {
    t.write_line("Enter a URL encoded string to decode:")
        .unwrap();
    let input = t.read_line().unwrap();
    let result = decode(input.as_str());
    match result {
        Ok(result_str) => t
            .write_line(format!("Result: {result_str}").as_str())
            .unwrap(),
        Err(err) => t
            .write_line(format!("Error decoding URL encoded string: {err}").as_str())
            .unwrap(),
    }
}

fn url_encode(_a: &Args, t: &Term) {
    t.write_line("Enter a string to URL encode:")
        .expect("error wrting line to stdout");
    let input = t.read_line().unwrap();
    let result = encode(input.as_str());
    t.write_line(format!("Result: {result}").as_str()).unwrap();
}

fn html_unescape(_a: &Args, t: &Term) {
    t.write_line("Enter a HTML escaped string to unescape:")
        .expect("error wrting line to stdout");
    let input = t.read_line().unwrap();
    let result = htmlescape::decode_html(input.as_str());
    match result {
        Ok(result_str) => t
            .write_line(format!("Result: {result_str}").as_str())
            .unwrap(),
        Err(err) => t
            .write_line(format!("Error decoding HTML escaped string: {err:?}").as_str())
            .unwrap(),
    }
}

fn html_escape(_a: &Args, t: &Term) {
    t.write_line("Enter a string to HTML escape:")
        .expect("error wrting line to stdout");
    let input = t.read_line().unwrap();
    let result = htmlescape::encode_attribute(input.as_str());
    t.write_line(format!("Result: {result}").as_str()).unwrap();
}

fn jwt_decode(_a: &Args, t: &Term) {
    t.write_line("Enter a JWT token to decode:")
        .expect("error wrting line to stdout");
    let input = t.read_line().unwrap();
    let jwt_parts: Vec<&str> = input.split(".").collect();
    let header = _base_64_decode(&mut String::from_str(jwt_parts[0]).unwrap());
    let payload = _base_64_decode(&mut String::from_str(jwt_parts[1]).unwrap());
    match header {
        Ok(header_str) => t
            .write_line(format!("Header Data: {header_str}").as_str())
            .unwrap(),
        Err(err) => t
            .write_line(format!("Error decoding header: {err}").as_str())
            .unwrap(),
    }
    match payload {
        Ok(payload_str) => t
            .write_line(format!("Payload Data: {payload_str}").as_str())
            .unwrap(),
        Err(err) => t
            .write_line(format!("Error decoding payload: {err}").as_str())
            .unwrap(),
    }
}
