use clap::{Parser, Subcommand};
use console::{Style, Term};
use std::fmt;
mod select;
use select::{Select, SelectOption};
mod tools;
use tools::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    tool: Option<Tools>,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "List the available tools"
    )]
    list: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Make the output plain text"
    )]
    plain: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Tools {
    Base64Decode {
        data: Option<String>,
    },
    Base64Encode {
        data: Option<String>,
    },
    URLDecode {
        data: Option<String>,
    },
    URLEncode {
        data: Option<String>,
    },
    URLParse {
        data: Option<String>,
    },
    HTMLUnescape {
        data: Option<String>,
    },
    HTMLEscape {
        data: Option<String>,
    },
    JWTDecode {
        data: Option<String>,
        #[arg(long, default_value_t = false, help = "Output the header data")]
        header: bool,
        #[arg(long, default_value_t = false, help = "Output the payload data")]
        payload: bool,
    },
    FormatJSON {
        #[arg(help = "Enter JSON data and then type 'END' on a new line to stop input")]
        data: Option<String>,
    },
    FormatYAML {
        #[arg(help = "Enter YAML data and then type 'END' on a new line to stop input")]
        data: Option<String>,
    },
    YamlToJSON {
        #[arg(help = "Enter YAML data and then type 'END' on a new line to stop input")]
        data: Option<String>,
    },
    TomlToJSON {
        #[arg(help = "Enter TOML data and then type 'END' on a new line to stop input")]
        data: Option<String>,
    },
}

impl fmt::Display for Tools {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tools::Base64Decode { data: _ } => write!(f, "base64-decode"),
            Tools::Base64Encode { data: _ } => write!(f, "base64-encode"),
            Tools::URLDecode { data: _ } => write!(f, "url-decode"),
            Tools::URLEncode { data: _ } => write!(f, "url-encode"),
            Tools::URLParse { data: _ } => write!(f, "url-parse"),
            Tools::HTMLUnescape { data: _ } => write!(f, "html-unescape"),
            Tools::HTMLEscape { data: _ } => write!(f, "html-escape"),
            Tools::JWTDecode {
                data: _,
                header: _,
                payload: _,
            } => write!(f, "jwt-decode"),
            Tools::FormatJSON { data: _ } => write!(f, "format-json"),
            Tools::FormatYAML { data: _ } => write!(f, "format-yaml"),
            Tools::YamlToJSON { data: _ } => write!(f, "yaml-to-json"),
            Tools::TomlToJSON { data: _ } => write!(f, "toml-to-json"),
        }
    }
}

fn main() {
    let args = Args::parse();

    let tool_data: &Option<String> = match &args.tool {
        Some(Tools::Base64Decode { data }) => data,
        Some(Tools::Base64Encode { data }) => data,
        Some(Tools::URLDecode { data }) => data,
        Some(Tools::URLEncode { data }) => data,
        Some(Tools::URLParse { data }) => data,
        Some(Tools::HTMLUnescape { data }) => data,
        Some(Tools::HTMLEscape { data }) => data,
        Some(Tools::JWTDecode {
            data,
            header: _,
            payload: _,
        }) => data,
        Some(Tools::FormatJSON { data }) => data,
        Some(Tools::FormatYAML { data }) => data,
        Some(Tools::YamlToJSON { data }) => data,
        Some(Tools::TomlToJSON { data }) => data,
        None => &None,
    };
    let header_flag: bool = match &args.tool {
        Some(Tools::JWTDecode {
            data: _,
            header,
            payload: _,
        }) => *header,
        _ => false,
    };
    let payload_flag: bool = match &args.tool {
        Some(Tools::JWTDecode {
            data: _,
            header: _,
            payload,
        }) => *payload,
        _ => false,
    };

    let select = Select {
        options: vec![
            SelectOption {
                message: "Base64 Decode",
                arg_name: Some(Tools::Base64Decode {
                    data: tool_data.clone(),
                }),
                handle: base_64_decode,
            },
            SelectOption {
                message: "Base64 Encode",
                arg_name: Some(Tools::Base64Encode {
                    data: tool_data.clone(),
                }),
                handle: base_64_encode,
            },
            SelectOption {
                message: "URL Decode",
                arg_name: Some(Tools::URLDecode {
                    data: tool_data.clone(),
                }),
                handle: url_decode,
            },
            SelectOption {
                message: "URL Encode",
                arg_name: Some(Tools::URLEncode {
                    data: tool_data.clone(),
                }),
                handle: url_encode,
            },
            SelectOption {
                message: "URL Parse",
                arg_name: Some(Tools::URLParse {
                    data: tool_data.clone(),
                }),
                handle: url_parse,
            },
            SelectOption {
                message: "HTML Unescape",
                arg_name: Some(Tools::HTMLUnescape {
                    data: tool_data.clone(),
                }),
                handle: html_unescape,
            },
            SelectOption {
                message: "HTML Escape",
                arg_name: Some(Tools::HTMLEscape {
                    data: tool_data.clone(),
                }),
                handle: html_escape,
            },
            SelectOption {
                message: "JWT Decode",
                arg_name: Some(Tools::JWTDecode {
                    data: tool_data.clone(),
                    header: header_flag,
                    payload: payload_flag,
                }),
                handle: jwt_decode,
            },
            SelectOption {
                message: "Format JSON",
                arg_name: Some(Tools::FormatJSON {
                    data: tool_data.clone(),
                }),
                handle: format_json,
            },
            SelectOption {
                message: "Format YAML",
                arg_name: Some(Tools::FormatYAML {
                    data: tool_data.clone(),
                }),
                handle: format_yaml,
            },
            SelectOption {
                message: "YAML to JSON",
                arg_name: Some(Tools::YamlToJSON {
                    data: tool_data.clone(),
                }),
                handle: yaml_to_json,
            },
            SelectOption {
                message: "TOML to JSON",
                arg_name: Some(Tools::TomlToJSON {
                    data: tool_data.clone(),
                }),
                handle: toml_to_json,
            },
            SelectOption {
                message: "Quit",
                arg_name: None,
                handle: |_, _| {},
            },
        ],
        message: "What do you want to do?",
        args: &args,
        term: Term::stdout(),
    };

    if args.list {
        print(&select.term, "Available tools are:\n");
        select.list_options();
        return;
    }

    match select.arg_match(&args.tool.as_ref()) {
        Some(option) => (option.handle)(&select.args, &select.term),
        None => select.numbered_list(),
    }
}

fn read_input(t: &Term, multi_line: Option<bool>, end_line: Option<&str>) -> String {
    let ml = multi_line.unwrap_or(false);
    let el = end_line.unwrap_or("");
    if ml {
        let mut input = String::new();
        loop {
            let next_line = t.read_line().unwrap();
            if next_line.eq(el) {
                break;
            }
            input.push_str(format!("{next_line}\n").as_str());
        }
        return input;
    } else {
        return t.read_line().unwrap();
    }
}

fn print(t: &Term, msg: &str) {
    t.write_line(msg).unwrap();
}

fn success(t: &Term, data: &str, label: Option<&str>, plain: bool) {
    if !plain {
        let l = label.unwrap_or("Result");
        let green = Style::new().green();
        t.write_line(format!("{}: {}", green.apply_to(l), data).as_str())
            .unwrap()
    } else {
        t.write_line(data).unwrap();
    }
}

fn error(t: &Term, data: &str, label: Option<&str>) {
    let l = label.unwrap_or("Error");
    let red = Style::new().red();
    t.write_line(format!("{}: {}", red.apply_to(l), data).as_str())
        .unwrap()
}
