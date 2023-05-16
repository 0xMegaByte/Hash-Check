# Hash Check
1-Day Challenge: I aimed to learn Rust language within a day, seeking a simple project to work on.\
With the help of ChatGPT, I found a suitable project idea that allowed me to delve into Rust's syntax and features,\
accelerating my learning process effectively.

## Introduction

The Hash Check Rust project is a tool that allows users to provide a file, and through the Rust code, it calculates the MD5 hash of the file.\
It then utilizes the Virustotal API to retrieve community votes and determine the file's reputation, providing a convenient way to check the file's trustworthiness.

## Build

To build the Hash Check Rust project, follow these steps:

1. Ensure that you have Rust installed on your system.\
    If not, you can download and install Rust from the official website: [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. Clone the Hash Check repository from GitHub using the following command:\
```git clone https://github.com/0xMegaByte/Hash-Check.git```
3. Navigate to the project's directory:
```cd Hash-Check```
4. Build the project using Cargo, the Rust package manager:
``` cargo build ```


## Run

To run the Hash Check Rust project, execute the following command:\
```cargo run -- <file_path>```

Replace `<file_path>` with the path to the file you want to check. The project will calculate the MD5 hash of the file and retrieve its reputation from the Virustotal API. The result will be displayed in the console.

Make sure you have an active internet connection and the necessary permissions to access the file.

## Dependencies

The Hash Check Rust project relies on the following dependencies:

- [reqwest](https://crates.io/crates/reqwest): A Rust HTTP client for making API requests.
- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json): Libraries for JSON serialization and deserialization.
- [clap](https://crates.io/crates/clap) A simple to use, efficient, and full-featured Command Line Argument Parser.
- [md5](https://crates.io/crates/md5) provides the MD5 hash function.
- [tokio](https://crates.io/crates/tokio) An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.

These dependencies are managed and resolved automatically by Cargo.

## Contributing

If you would like to contribute to this project, please feel free to submit a pull request. I welcome any suggestions or improvements that you may have.

## Authors

This project was created by Matan Shitrit [@0xMegaByte](https://twitter.com/0xMegaByte).

## License

This project is licensed under the [GPL-3.0](https://opensource.org/license/gpl-3-0/). See the LICENSE file for more information.

