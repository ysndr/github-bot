use github_types::PushEvent;
use hubcaps::{Credentials, Github};
use hubcaps::issues::{Issue, IssueOptions};
use warp::{Filter, Rejection, reject};
use warp_github_webhook::{webhook, Kind};

mod actions;

#[derive(Debug)]
struct HubcapsError(String);

impl reject::Reject for HubcapsError {}



#[tokio::main]
async fn main() -> Result<(), i32> {
    let filter = warp::path("github-hook")
        .and(webhook(Kind::PUSH, ""))
        .and_then(push_handler);




    Ok(())
}

async fn push_handler(event: PushEvent) -> Result<Issue, Rejection> {

    let client = Github::new(
        "ysndr-github-bot",
        Credentials::Token("bad134507e55f003ae6df0f3cfc282d64a5f3021".into())).unwrap();

    let issues = client.repo("ysndr", "github-api-playground").issues();
    let issue = IssueOptions::new::<_,_,String,String>(
         String::from("Hook generated issue"),
         Some(String::from("Hey looks a programmatically created issue")),
         None,
         None,
         vec!());

    issues.create(&issue)
        .await
        .map_err(|e: hubcaps::errors::Error| {
            reject::custom(HubcapsError(format!("{}", e)))
        })


}


#[tokio::test]
async fn matches_a_github_event() {

    let event = include_str!("../test_res/push-event-body.txt");


}
