/*
Copyright (C) 2023 Matan Shitrit (0xMegaByte)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
