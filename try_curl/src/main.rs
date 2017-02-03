// Jessica A Kelly
//
// trying again, but using https://developer.github.com/guides/getting-started/
// as a guide and the curl library.
// this

extern crate curl;
use std::io::{stdout, Write};
use curl::easy::{Easy, List};

fn main() {
    println!("------------------------------");
    println!("user events for user uijkelly");
    github_user_events("uijkelly");
}

// get one from github API
// GET /users/:username/events
fn github_user_events(name: &str) {
    let mut easy = Easy::new();
    let mut url = "https://api.github.com/users/";

    easy.url("https://api.github.com/users/uijkelly").unwrap();

    let mut list = List::new();
    list.append("User-Agent: uijkelly").unwrap();
    list.append("Authorization: token INSERT HERE").unwrap();
    list.append("Accept: application/vnd.github.v3+json").unwrap();
    easy.http_headers(list).unwrap();

    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
