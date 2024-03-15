
<img width="714" alt="rust-api-test" src="https://github.com/prasadjivane/rust-api-test/assets/26869583/10445b8d-7bbb-46a9-bfef-e49662a9176b">

[<img alt="github" src="https://img.shields.io/badge/Github-rust%20api%20test-blue" height="30">](https://github.com/prasadjivane/rust-api-test)
[<img alt="crates.io" src="https://img.shields.io/badge/crates.io-V0.1.0-green" height="30">](https://crates.io/crates/rust-api-test)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rust%20api%20test-orange" height="30">](https://docs.rs/crate/rust-api-test)
[<img alt="releases" src="https://img.shields.io/badge/Releases%20V0.1.0-8A2BE2" height="30">](https://github.com/prasadjivane/rust-api-test/releases)

# rust-api-test

`rust-api-test` is a Rust package that provides a command-line interface (CLI) for testing REST APIs in real-time. It allows Rust developers to easily perform HTTP GET, POST, PUT, and DELETE requests from the command line.

## Features

- Supports GET, POST, PUT, and DELETE HTTP methods.
- Allows sending JSON data in the request body.
- Provides real-time feedback on the response including status code and response body.

## Installation

To use `rust-api-test` in your Rust project, simply add it as a dependency in your `Cargo.toml` file:

```ssh
[dependencies]
rust-api-test = "0.1.0"
```

## Usage

Once installed, you can use `rust-api-test` from the command line:

```
rust-api-test
```
Follow the on-screen instructions to select the HTTP method, enter the URL, and provide optional JSON data for POST and PUT requests.

## Example

```Enter your choice:
1. GET
2. POST
3. PUT
4. DELETE
5. Exit
```
## Dependencies

-   reqwest - HTTP client for Rust.
-   serde_json - JSON serialization and deserialization library for Rust.


## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to open an [issue](https://github.com/prasadjivane/rust-api-test/issues) or submit a [pull](https://github.com/prasadjivane/rust-api-test/pulls) request on [GitHub](https://github.com/prasadjivane).

## License

This project is licensed under the [MIT License](https://github.com/prasadjivane/rust-api-test?tab=MIT-1-ov-file) - see the LICENSE file for details.
