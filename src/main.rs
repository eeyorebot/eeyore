extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::Github;

fn main() {
    let client = Client::new();
    let github = Github::new(
        "my-cool-user-agent/0.1.0",
        &client,
        None::<String>,
        );

    let repo = github.repo("rust-lang", "rust");

    let labels = repo.labels();

    for l in labels.list().unwrap() {
        println!("{:?}", l)
    }
}
