use crate::{Args, Tools, error, print, read_input, success};
use base64::{Engine as _, engine::general_purpose};
use console::Term;
use htmlescape;
use serde_json;
use serde_yaml_ng;
use std::str::FromStr;
use toml;
use url::Url;
use urlencoding::{decode, encode};

pub fn base_64_decode(a: &Args, t: &Term) {
    let msg = "Enter a Base64 encoded string to decode:";
    let input = match &a.tool {
        Some(Tools::Base64Decode { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };
    let result = _base_64_decode(&input);
    match result {
        Ok(result_str) => success(t, result_str.as_str(), None, a.plain),
        Err(err) => error(t, err.to_string().as_str(), None),
    }
}

fn _base_64_decode(input: &str) -> Result<String, String> {
    let result = general_purpose::STANDARD_NO_PAD.decode(input);
    match result {
        Ok(result_bytes) => match String::from_utf8(result_bytes) {
            Ok(result_str) => return Ok(result_str),
            Err(err) => return Err(format!("Error converting bytes to string: {err}")),
        },
        Err(err) => return Err(format!("Error decoding Base64 string: {err}")),
    }
}

pub fn base_64_encode(a: &Args, t: &Term) {
    let msg = "Enter a string to Base64 encode:";
    let input = match &a.tool {
        Some(Tools::Base64Encode { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };

    let result = general_purpose::STANDARD_NO_PAD.encode(input);
    success(t, result.as_str(), None, a.plain);
}

pub fn url_decode(a: &Args, t: &Term) {
    let msg = "Enter a URL encoded string to decode:";
    let input = match &a.tool {
        Some(Tools::URLDecode { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };

    let result = decode(input.as_str());
    match result {
        Ok(result_str) => success(t, result_str.to_string().as_str(), None, a.plain),
        Err(err) => error(
            t,
            err.to_string().as_str(),
            Some("Error decoding URL encoded string"),
        ),
    }
}

pub fn url_encode(a: &Args, t: &Term) {
    let msg = "Enter a string to URL encode:";
    let input = match &a.tool {
        Some(Tools::URLEncode { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };

    let result = encode(input.as_str());
    success(t, result.to_string().as_str(), None, a.plain);
}

pub fn html_unescape(a: &Args, t: &Term) {
    let msg = "Enter a HTML escaped string to unescape:";
    let input = match &a.tool {
        Some(Tools::HTMLUnescape { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };

    let result = htmlescape::decode_html(input.as_str());
    match result {
        Ok(result_str) => success(t, result_str.as_str(), None, a.plain),
        Err(err) => error(
            t,
            format!("{err:?}").as_str(),
            Some("Error decoding HTML escaped string"),
        ),
    }
}

pub fn html_escape(a: &Args, t: &Term) {
    let msg = "Enter a string to HTML escape:";
    let input = match &a.tool {
        Some(Tools::HTMLEscape { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };

    let result = htmlescape::encode_attribute(input.as_str());
    success(t, result.as_str(), None, a.plain);
}

pub fn jwt_decode(a: &Args, t: &Term) {
    let msg = "Enter a JWT token to decode:";
    let mut header = match &a.tool {
        Some(Tools::JWTDecode {
            data: _,
            header,
            payload: _,
        }) => *header,
        _ => false,
    };
    let mut payload = match &a.tool {
        Some(Tools::JWTDecode {
            data: _,
            header: _,
            payload,
        }) => *payload,
        _ => false,
    };
    let input = match &a.tool {
        Some(Tools::JWTDecode {
            data,
            header: _,
            payload: _,
        }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };

    let jwt_parts: Vec<&str> = input.split(".").collect();
    if jwt_parts.len() < 2 {
        error(t, "Invalid JWT token", None);
        return;
    }

    if !header && !payload {
        header = true;
        payload = true;
    }

    if header {
        let header = _base_64_decode(&mut String::from_str(jwt_parts[0]).unwrap());
        match header {
            Ok(header_str) => success(t, header_str.as_str(), Some("Header Data"), a.plain),
            Err(err) => error(t, err.as_str(), Some("Error decoding header")),
        }
    }

    if payload {
        let payload = _base_64_decode(&mut String::from_str(jwt_parts[1]).unwrap());
        match payload {
            Ok(payload_str) => success(t, payload_str.as_str(), Some("Payload Data"), a.plain),
            Err(err) => error(t, err.as_str(), Some("Error decoding payload")),
        }
    }
}

pub fn url_parse(a: &Args, t: &Term) {
    let msg = "Enter a URL to parse:";
    let input = match &a.tool {
        Some(Tools::URLParse { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, None, None)
        }),
        _ => {
            print(t, msg);
            read_input(t, None, None)
        }
    };
    match Url::parse(&input) {
        Ok(url) => {
            success(
                t,
                format!("{}", url.scheme()).as_str(),
                Some("Scheme"),
                false,
            );
            success(
                t,
                format!("{}", url.username()).as_str(),
                Some("Username"),
                false,
            );
            success(
                t,
                format!("{}", url.password().unwrap_or("")).as_str(),
                Some("Password"),
                false,
            );
            success(
                t,
                format!("{}", url.host_str().unwrap_or("")).as_str(),
                Some("Host"),
                false,
            );
            success(
                t,
                format!(
                    "{}",
                    url.port().map_or("".to_string(), |port| port.to_string())
                )
                .as_str(),
                Some("Port"),
                false,
            );
            success(t, format!("{}", url.path()).as_str(), Some("Path"), false);
            success(
                t,
                format!("{}", url.query().unwrap_or("")).as_str(),
                Some("Query"),
                false,
            );
            success(
                t,
                format!("{}", url.fragment().unwrap_or("")).as_str(),
                Some("Fragment"),
                false,
            );
        }
        Err(err) => error(t, err.to_string().as_str(), None),
    }
}

pub fn format_json(a: &Args, t: &Term) {
    let msg = "Enter some JSON to format (type 'END' on a new line to stop input):";
    let input = match &a.tool {
        Some(Tools::FormatJSON { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }),
        _ => {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }
    };
    let result: serde_json::Value = match serde_json::from_str(input.as_str()) {
        Ok(val) => val,
        Err(err) => {
            error(t, err.to_string().as_str(), None);
            return;
        }
    };
    match serde_json::to_string_pretty(&result) {
        Ok(json_str) => success(t, json_str.as_str(), None, true),
        Err(err) => error(t, err.to_string().as_str(), None),
    }
}

pub fn format_yaml(a: &Args, t: &Term) {
    let msg = "Enter some YAML to format (type 'END' on a new line to stop input):";
    let input = match &a.tool {
        Some(Tools::FormatYAML { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }),
        _ => {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }
    };
    let result: serde_yaml_ng::Value = match serde_yaml_ng::from_str(input.as_str()) {
        Ok(val) => val,
        Err(err) => {
            error(t, err.to_string().as_str(), None);
            return;
        }
    };
    match serde_yaml_ng::to_string(&result) {
        Ok(json_str) => success(t, json_str.as_str(), None, true),
        Err(err) => error(t, err.to_string().as_str(), None),
    }
}

pub fn yaml_to_json(a: &Args, t: &Term) {
    let msg = "Enter some YAML to convert to JSON (type 'END' on a new line to stop input):";
    let input = match &a.tool {
        Some(Tools::YamlToJSON { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }),
        _ => {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }
    };
    let result: serde_yaml_ng::Value = match serde_yaml_ng::from_str(input.as_str()) {
        Ok(val) => val,
        Err(err) => {
            error(t, err.to_string().as_str(), None);
            return;
        }
    };
    match serde_json::to_string_pretty(&result) {
        Ok(json_str) => success(t, json_str.as_str(), None, true),
        Err(err) => error(t, err.to_string().as_str(), None),
    }
}

pub fn toml_to_json(a: &Args, t: &Term) {
    let msg = "Enter some TOML to convert to JSON (type 'END' on a new line to stop input):";
    let input = match &a.tool {
        Some(Tools::TomlToJSON { data }) => data.clone().unwrap_or_else(|| {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }),
        _ => {
            print(t, msg);
            read_input(t, Some(true), Some("END"))
        }
    };
    let result: toml::Value = match toml::from_str(input.as_str()) {
        Ok(val) => val,
        Err(err) => {
            error(t, err.to_string().as_str(), None);
            return;
        }
    };
    match serde_json::to_string_pretty(&result) {
        Ok(json_str) => success(t, json_str.as_str(), None, true),
        Err(err) => error(t, err.to_string().as_str(), None),
    }
}
