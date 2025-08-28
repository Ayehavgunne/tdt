# tdt
A collection of common dev tools that can be used from the command line. TerminalDevTools

## Tools
- Base 64 Decode
- Base 64 Encode
- URL Decode
- URL Encode
- URL Parse
- HTML Unescape
- HTML Escape
- JWT Decode
- Format JSON
- Format YAML
- YAML to JSON
- TOML to JSON

## How to use
Just calling the bare `tdt` command will present you with a numbered list to choose the tool you would like to use. Just type the number for the associated tool and hit enter.
```
$ tdt
What do you want to do?

1) Base64 Decode
2) Base64 Encode
...
```

You can also just provide the name of the tool you would like to use. You can optionally provide the data to operate on as the next argument
```
tdt base64-decode <optionally provide some base64 data here>
```

Use `tdt help` or `tdt -l` to see all the names of the tools you can use.

To get help with a specific tool you can do `tdt <tool name here> --help`
