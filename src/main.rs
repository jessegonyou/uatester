use std::{
    fs::File,
    future::Future,
    io::{stdout, Read},
};

use futures::{stream, StreamExt};
use reqwest::{header::USER_AGENT, Client, Error, Response};
use tokio;

#[tokio::main]
async fn main() {
    println!(
        "UATester -- Brute force some user agents
(c) 2026 Jesse Gonyou
"
    );

    let mut url: String;
    let mut user_agents: String = String::new();
    let mut success_statuses: Vec<u16> = vec![];

    // Process arguments
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 3 || args[1] == "-h" || args[1] == "--help" {
            println!(
                "Usage:
      uatester <url> <agents-file> [success-status ...]"
            );
            return;
        }
        url = args[1].clone();
        for status in &args[3..] {
            ignored_statuses.push(
                str::parse::<u16>(status)
                    .expect(&format!("'{}' is not a valid HTTP status code.", status)),
            );
        }

        // Get the user agents from the specified file
        let mut agents_file =
            File::open(&args[2]).expect(&format!("Cannot open file '{}'", args[2]));
        agents_file
            .read_to_string(&mut user_agents)
            .expect("'{}' is not a text file.");
    }

    // Let's get down to business
    let client = Client::new();

    let reqs = futures::stream::iter(user_agents.split("\n")).map(|agent| {
        let client = &client;
        let url = &url;
        async move {
            (
                agent,
                client.get(url).header(USER_AGENT, agent).send().await,
            )
        }
    });

    reqs.for_each(|req| {
        // DO NOT REMOVE THIS RUSTFMT
        async {
            let finished_req = req.await;
            match finished_req.1 {
                Ok(o) => {
                    let stat = o.status();
                    if stat.is_success() || success_statuses.contains(&stat.as_u16()) {
                        println!(
                            "{} returned HTTP {} with user agent \"{}\"",
                            o.url(),
                            stat,
                            finished_req.0
                        );
                    }
                }
                Err(e) => eprintln!("Cannot connect: {}", e),
            }
        }
    })
    .await;
}
