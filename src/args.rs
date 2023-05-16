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

use clap::{Parser, ArgAction};

pub mod ascii_art {
    pub fn generate_art() -> String {
        
        //‎ is an empty character
        let art = format!(r"‎
        _   _           _       _____ _               _             
       | | | |         | |     /  __ \ |             | |            
       | |_| | __ _ ___| |__   | /  \/ |__   ___  ___| | _____ _ __ 
       |  _  |/ _` / __| '_ \  | |   | '_ \ / _ \/ __| |/ / _ \ '__|
       | | | | (_| \__ \ | | | | \__/\ | | |  __/ (__|   <  __/ |   
       \_| |_/\__,_|___/_| |_|  \____/_| |_|\___|\___|_|\_\___|_|   
       by {}                          v{}                                                           
       =============================================================                                                             
      ",env!("CARGO_PKG_AUTHORS"),env!("CARGO_PKG_VERSION"));
        art
    }
}

#[derive(Parser, Debug)]
#[command(author, 
    about =
    "=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=
    \nCalculates the MD5 hash of a specified file and uses the VirusTotal API to check its reputation based on
    \ncommunity votes.
    \nNOTE: Generate your own VirusTotal API key.
    \n=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=", 
    long_about =
    ascii_art::generate_art())]
pub struct Args {
    #[arg(short = 'f', long = "file", name = "FILE_PATH", help = "Suspicious file path")]
    pub input_file_path: String,

    #[arg(short = 'm', long = "md5", action = ArgAction::SetTrue, help = "calculate file's MD5", required = true)]
    pub calculate_md5: bool,
}