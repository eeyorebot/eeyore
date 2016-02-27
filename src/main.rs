extern crate hyper;
extern crate hubcaps;
extern crate iron;

use hyper::Client;
use hubcaps::Github;
use iron::prelude::*;
use iron::status;

fn main() {
    let client = Client::new();
    let github = Github::new(
        "my-cool-user-agent/0.1.0",
        &client,
        None::<String>,
        );

    let repo = github.repo("rust-lang", "rust");

    let labels = repo.labels();

    let output = labels.list().unwrap().iter().map(|label| {
        format!("{:?}", label)
    }).collect::<Vec<_>>().join("\n");

    Iron::new(move |_: &mut Request| {
        Ok(Response::with((
            status::Ok,
            &output[..]
        )))
    }).http("localhost:3000").unwrap();
}
