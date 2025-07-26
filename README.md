# RUReady

Utility to copy a report of today’s carpoolers when you are ready to leave.

## Installation

```shell
cargo install --path .
```

## Configuration

Copy to `%APPDATA%\RUReady\config\ruready.toml` the following content:

```toml
carpoolers = [
    "Alice",
    "Bob",
    "Charlie"
]
me = "Zoe"
```

## Usage

Just run `ruready`, select the carpoolers of the day, and hit Enter to copy the report to your clipboard.