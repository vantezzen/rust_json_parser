# Simple JSON Parser

This project contains a JSON parser in Rust.

> This was created as a project for learning Rust. The parser is not written to be fully spec-compliant and cannot really be used as a cargo package.

## Features

The parser supports:

- Objects
- Arrays
- Strings
- Numbers (positive, negative, except `e`)
- Constants (true, false, null)
- Nested structures

## Usage

```
cargo run -- [filename]
```

Example input (`demo.json`):

```JSON
{
  "contents": "string",
  "key": {
    "some": "value\" with quotes",
    "number": 1.9234,
    "negative": -1563.3222,
    "boolean": true,
    "other_boolean": false,
    "null": null
  },
  "array": ["string", "string"]
}
```

Example output:

```
Simple JSON Parser
Reading file demo.json
[src/main.rs:19] contents = Object(
    {
        "key": Object(
            {
                "some": String(
                    "value\" with quotes",
                ),
                "boolean": True,
                "number": Number(
                    "1.9234",
                ),
                "negative": Number(
                    "-1563.3222",
                ),
                "other_boolean": False,
                "null": Null,
            },
        ),
        "array": Array(
            [
                String(
                    "string",
                ),
                String(
                    "string",
                ),
            ],
        ),
        "contents": String(
            "string",
        ),
    },
)
```
