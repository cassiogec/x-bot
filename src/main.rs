use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct Commit {
    url: String,
    sha: String,
    node_id: String,
    html_url: String,
    comments_url: String,
    commit: CommitDetails,
}

#[derive(Deserialize, Debug)]
struct CommitDetails {
    url: String,
    author: Contributor,
    committer: Contributor,
    message: String,
}

#[derive(Deserialize, Debug)]
struct Contributor {
    name: String,
    email: String,
    date: DateTime<Utc>,
}

fn main() {
    println!("Hello, world!");
}
