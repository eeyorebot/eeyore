extern crate hyper;
extern crate hubcaps;
extern crate iron;
extern crate router;
extern crate inth_oauth2;
extern crate cookie;
extern crate oven;

use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType, Location};
use iron::modifiers::Header;

use oven::prelude::*;
use hubcaps::Github;
use router::Router;
use inth_oauth2::provider::GitHub;
use inth_oauth2::token::Token;
use std::env;

fn main() {
    let cookie_signing_key = env::var("SECRET")
        .expect("SECRET must be specified to sign cookies").as_bytes().to_vec();

    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        Ok(Response::with((
            status::Ok,
            Header(ContentType::html()),
            "<html><body><div><a href='/oauth'>Log in with Github</a></div></body></html>"
        )))
    });

    router.get("/oauth", |_: &mut Request| {
        let oauth_client = github_client();

        let auth_uri = oauth_client.auth_uri(Some("write:repo_hook,public_repo"), None).unwrap();
        Ok(Response::with((
            status::Found,
            Header(Location(auth_uri.to_string())),
            format!("You are being <a href='{}'>redirected</a>.", auth_uri),
        )))
    });

    router.get("/callback", |request: &mut Request| {
        let oauth_client = github_client();

        let url = request.url.clone();
        let generic_url = url.into_generic_url();

        let query_params = generic_url.query_pairs().unwrap();
        let (_, code) = query_params.into_iter().find(|&(ref key, _)| {
            *key == String::from("code")
        }).unwrap();

        let bearer_token = oauth_client.request_token(&Default::default(), code.trim()).unwrap();

        let redirect_uri = String::from("/repos");
        let mut response = Response::with((
            status::Found,
            Header(Location(redirect_uri.clone())),
            format!("You are being <a href='{}'>redirected</a>.", redirect_uri),
        ));
        response.set_cookie(cookie::Cookie::new(
            String::from("access_token"), String::from(bearer_token.access_token())
        ));
        Ok(response)
    });

    router.get("/repos", |request: &mut Request| {
        let access_token = request.get_cookie("access_token");
        match access_token {
            Some(token) => {
                let repos = authorized_repos(&token.value);

                let output = repos.into_iter().map(|r| {
                    format!("<tr><td>{}</td><td><a href='/enable?repo={}'>Enable</a></td><td><a href='/disable?repo={}'>Disable</a></td></tr>", r.full_name, r.full_name, r.full_name)
                }).collect::<Vec<_>>().join("");

                Ok(Response::with((
                    status::Ok,
                    Header(ContentType::html()),
                    format!("<html><body><table>{}</table></body></html>", output),
                )))
            },
            None => { // Not logged in
                let redirect_uri = String::from("/");
                Ok(Response::with((
                    status::Found,
                    Header(Location(redirect_uri.clone())),
                    format!("You are being <a href='{}'>redirected</a>.", redirect_uri),
                )))
            },
        }
    });

    let mut chain = Chain::new(router);
    chain.link(oven::new(cookie_signing_key));
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn github_client() -> inth_oauth2::Client<GitHub> {
    inth_oauth2::Client::<GitHub>::new(
        env::var("CLIENT_ID").expect("Github OAuth CLIENT_ID must be specified"),
        env::var("CLIENT_SECRET").expect("Github OAuth CLIENT_SECRET must be specified"),
        env::var("REDIRECT_URI").ok()
    )
}

fn authorized_repos(access_token: &str) -> Vec<hubcaps::rep::Repo> {
    let user_client = hyper::Client::new();
    let user_github = Github::new(
        "my-cool-user-agent/0.1.0",
        &user_client,
        Some(access_token),
    );
    let repos = user_github.repos().list().unwrap();
    // TODO: filter to only return repositories on which the user has admin permissions
    // TODO: paginate to get all repos, not currently supported by hubcaps
    repos
}
