// Jessica A Kelly
//
// Getting to know the GitHub API and accessing with Rust
//
// General Notes:
//  https://developer.github.com/v3/activity/
//  https://github.com/GlenDC/github-rust
//
// Modification History:
//  27JAN2017 - JAK using example from github-rust repo to get me started.
//   having some issues with using the code here, so going to put this away
//   and start over -- also the repo was last updated 2 years ago so...

extern crate github;

// Example of some public requests

use github::Client;
use github::error::*;
use github::activity::events::*;

fn main() {
    github_example();
}

// Description: main function from repo example. changing to use my username.
//  and then
fn github_example() {
    // simplest and most-used way of creating a client
        // requiring a User-Agent used for requests.
        let client = &Client::new("uijkelly"); // JAK update to use my username

        // An example of a request that we expect to fail,
        // because the repository doesn't exist (404).
        //println!("# Example: failed list_my_repo_events");
        //if let Err(err) = list_my_repo_events(client, "42") {
        //    println!("list_repo_events failed: {}", err);
        //}
        // Here's one that will work.
        println!("# Example: successful list_my_repo_events");
        if let Ok((events, resp)) =  list_my_repo_events(client, "Rust-Experiments") {
            //println!("Resp Debug info => {:?}", resp);
            //println!("-------------------------");
            //println!("Events {:?}",events );
            for event in events {
                //println!("event ({}) #{} at {} by {}...",
                //    event.event, event.id, event.created_at, event.actor.login);
                println!("EVENT! {:?}", event );
                println!("event #{} at {} by {}...",
                     event.id, event.created_at, event.actor.login);
            }
        }

        // An example of a request that we expect to fail,
        // because we are unauthorized (404).
        //println!("# Example: failed list_organisation_events");
        //if let Err(err) = list_my_organisation_events(client, "PortalGaming") {
        //    println!("list_organisation_events failed: {}", err);
        //}

        // Here's one that will work


        // An example of getting and quickly summarizing
        // the most recent public issue events of the repo of this Client Library.
        // Most structs and enums in this lib are also debug-able, as shown here.
        //println!("# Example: list_my_repo_issue_events for `github-rust`");
        //if let Ok((events, resp)) =  list_my_repo_issue_events(client, "github-rust") {
        //    println!("Debug info => {:?}", resp);
        //    for event in events {
        //        println!("event ({}) #{} at {} by {}...",
        //            event.event, event.id, event.created_at, event.actor.login);
        //    }
        //}
}

// Description: An example of getting and quickly summarizing the most recent public github events.
// In this case we simply print all error information found.
fn example_all_github(){
    let client = &Client::new("uijkelly"); // JAK update to use my username

    println!("# Example: list_events");
    match list_events(client) {
        Ok((events, resp)) => {
            println!("listing public events succesfull, we have {} requsts remaining of {}.
             Limit resets @ {}...",
                resp.rate.remaining, resp.rate.limit, resp.rate.reset);
            for event in events {
                println!("event #{} at {} by {}...",
                    event.id, event.created_at, event.actor.login);
            }
        }
        Err(err) => {
            println!("list_events => {}", err);
            if let ClientError::Http(http_error) = err {
                for error in http_error.errors {
                    println!("    {}", error);
                }
            }
        }
    }
}
