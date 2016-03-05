extern crate hyper;
extern crate hubcaps;
extern crate iron;
extern crate router;
extern crate inth_oauth2;

use hubcaps::Github;
use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType, Location};
use iron::modifiers::Header;
use router::Router;
use inth_oauth2::provider::GitHub;
use inth_oauth2::token::Token;
use std::env;

fn main() {
    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        Ok(Response::with((
            status::Ok,
            Header(ContentType::html()),
            "<html><body><div><a href='/oauth'>Log in with Github</a></div></body></html>"
        )))
    });

    router.get("/oauth", |_: &mut Request| {
        let oauth_client = inth_oauth2::Client::<GitHub>::new(
            env::var("CLIENT_ID").expect("Github OAuth CLIENT_ID must be specified"),
            env::var("CLIENT_SECRET").expect("Github OAuth CLIENT_SECRET must be specified"),
            env::var("REDIRECT_URI").ok()
        );

        let auth_uri = oauth_client.auth_uri(Some("write:repo_hook,public_repo"), None).unwrap();
        Ok(Response::with((
            status::Found,
            Header(Location(auth_uri.to_string())),
            format!("You are being <a href='{}'>redirected</a>.", auth_uri),
        )))
    });

    router.get("/callback", |request: &mut Request| {
        let oauth_client = inth_oauth2::Client::<GitHub>::new(
            env::var("CLIENT_ID").expect("Github OAuth CLIENT_ID must be specified"),
            env::var("CLIENT_SECRET").expect("Github OAuth CLIENT_SECRET must be specified"),
            env::var("REDIRECT_URI").ok()
        );

        let url = request.url.clone();
        let generic_url = url.into_generic_url();

        let query_params = generic_url.query_pairs().unwrap();
        let (_, code) = query_params.into_iter().find(|&(ref key, _)| {
            *key == String::from("code")
        }).unwrap();
        let bearer_token = oauth_client.request_token(&Default::default(), code.trim()).unwrap();

        let user_client = hyper::Client::new();
        let user_github = Github::new(
            "my-cool-user-agent/0.1.0",
            &user_client,
            Some(bearer_token.access_token()),
        );
        let repos = user_github.repos().list();

        Ok(Response::with((
            status::Ok,
            format!("{:?}", repos),
        )))
    });

    Iron::new(router).http("localhost:3000").unwrap();
}
