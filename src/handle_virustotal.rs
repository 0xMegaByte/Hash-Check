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
