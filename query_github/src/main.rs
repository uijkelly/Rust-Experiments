// Jessica A Kelly
//
// Getting to know the GitHub API and accessing with Rust
//
// General Notes:
//  First attempt is in using_github_crate
//  This will be much more in the weeds
//  Use the json parsing explored previously to get at some information
//  Then later think about revisions and making it fully rustified.
//  For now... SIMPLE!
//
// List repositories for organizations.
//  https://developer.github.com/v3/repos/#list-organization-repositories
//
// List events for a user.
//  https://developer.github.com/v3/activity/events/#list-events-performed-by-a-user
//
//
// Modification History:
//

#[allow(unused_attributes)]
#[allow(dead_code)]

use rustc_serialize::json;
extern crate curl;
extern crate rustc_serialize;

use std::str;
use curl::easy::Easy;


#[derive(RustcDecodable)]
pub struct User {
    pub login: String,
    pub id: i32,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,
    pub name: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub email: String,
    pub hireable: bool,
    pub bio: Option<String>,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(RustcDecodable)]
pub struct Organization {

}
// Description: Goal is going to be to show repos added in the past week
//                                     show users added in the past week
//                                     get a list of all the events for a person
//
fn main() {
    // connect to API and get a list of repos in public space
    //

    // connect to API and get info about a single user

}
