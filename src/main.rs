extern crate hyper;
extern crate hubcaps;
extern crate iron;
extern crate router;

use hyper::Client;
use hubcaps::Github;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use router::Router;

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

    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        Ok(Response::with((
            status::Ok,
            Header(ContentType::html()),
            "<html><body><a href='/labels'>List Rust's Labels</a></body></html>"
        )))
    });
    router.get("/labels", move |_: &mut Request| {
        Ok(Response::with((
            status::Ok,
            &output[..]
        )))
    });

    Iron::new(router).http("localhost:3000").unwrap();
}
