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

use std::process::exit;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Meta {
    count: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    self_link: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Attributes {
    date: u64,
    verdict: String,
    value: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Data {
    attributes: Attributes,
    #[serde(rename = "type")]
    item_type: String,
    id: String,
    links: Links,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ApiResponse {
    meta: Meta,
    data: Vec<Data>,
    links: Links,
}

static API_KEY: &str = "<API_KEY>";

pub async fn check_md5_hash(hash: &str) -> Result<(), Box<dyn std::error::Error>> {

    if API_KEY == "<API_KEY>"{
        println!("[-] Generate your own VirusTotal API key!");
        exit(0);
    }

    let url = format!("https://www.virustotal.com/api/v3/files/{}/votes", &hash);
    println!("[+] URL: {:?}", url);
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("x-apikey", API_KEY)
        .header("content-type", "application/json")
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.text().await?;
            let api_response: ApiResponse = serde_json::from_str(&body).unwrap();

            if api_response.meta.count > 0 {
                for i in 0..api_response.data.len() {
                    if let Some(vote) = api_response.data.get(i) {
                        println!("[+] vote {}: {:#?}", i, vote.attributes.verdict);
                    }
                }
            }
            else{
                println!("[-] community votes not found!");
            }
        }
        _ => {
            panic!("[-] Error: '{}'", res.status());
        }
    }

    Ok(())
}
