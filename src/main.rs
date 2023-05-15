mod args;
mod handle_file;
mod handle_virustotal;

use args::ascii_art;
use clap::Parser;
use std::process::exit;

#[tokio::main]
async fn main() {
    let _args = args::Args::parse();
    println!("{}", ascii_art::generate_art());
    println!("[+] Input file: {}!", _args.input_file_path);

    let result;
    if _args.calculate_md5 == true {
        println!("[+] Hash type: MD5");
        result = handle_file::calculate_md5(&_args.input_file_path);
    } else {
        println!("[-] Choose hash method!");
        exit(0);
    }

    match result {
        Ok(hash) => {
            println!("[+] MD5:{}", hash);

            let _response = handle_virustotal::check_md5_hash(&hash).await;
        }
        Err(error) => println!("[-] Error: {}", error),
    }
}
