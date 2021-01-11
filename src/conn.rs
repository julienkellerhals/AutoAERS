use reqwest::blocking::{Client, Request};
use reqwest::blocking::ClientBuilder;
use reqwest::header;
use reqwest::Error;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::prelude::*;


pub fn connect() {
    dotenv().ok();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("secret"));

    let client = Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()
        .unwrap();

    let session_id = get_session_id(&client);

    let username = env::var("username").unwrap();
    let password = env::var("password").unwrap();

    let form = [
        // ("auth_key", session_id),
        // ("referer", "http://www.airline-empires.com/index.php?/page/home.html".to_string()),
        ("ips_username", username),
        ("ips_password", password),
        // ("rememberMe", "1".to_string())
    ];
    let response = client
        .post("http://www.airline-empires.com/index.php?app=core&module=global&section=login&do=process")
        .form(&form)
        .send()
        .unwrap();

    let test = response.headers();
    println!("{:?}", test);

    let raw_html = response.text().unwrap();

    let mut file = File::create("output.html").unwrap();
    file.write_all(raw_html.as_bytes());

    // Ok(())
}

fn get_session_id(client: &Client) -> String {
    let resp = client.get("http://www.airline-empires.com/index.php?/page/home.html").send().unwrap();
    let cookie = resp.cookies().next().unwrap();
    println!("{:?}", cookie);
    let session_id = cookie.value().to_string();
    println!("{:?}", session_id);

    let test = resp.headers();
    println!("{:?}", test);

    return session_id
}