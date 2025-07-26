# RUReady

Utility to copy a report of today’s carpoolers when you are ready to leave.

## Installation

```shell
cargo install --path .
```

## Configuration

Copy the following content to the configuration file according to your platform:

* Windows: `%APPDATA%\RUReady\config\ruready.toml`
* macOS: `~/Library/Application Support/RUReady/ruready.toml`

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
