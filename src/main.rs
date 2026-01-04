#![allow(dead_code)]

mod ctfd;
mod pwncollege;

use std::collections::HashMap;

use ctfd::{CTFdClient, ChallengeSolver, TeamId, TeamPosition};
use pwncollege::{PWNCollegeClient};

use clap::Parser;
use rusqlite::Connection;

/// A Discord webhook bot to announce CTFd solves
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Toggle for challenge import
    #[arg(short, long, default_value = "false")]
    import_challenges: bool,

    /// Challenge module from which to import - only required with import
    #[arg(long, short='m')]
    challenges_module: Option<String>,

    /// CTFd URL
    #[arg(long, short = 'c')]
    ctfd_url: String,

    /// CTFd API Key
    #[arg(long, short = 'a')]
    ctfd_api_key: String,

    /// Refresh interval in seconds
    #[arg(short, long, default_value = "30")]
    refresh_interval_seconds: u64,
}

async fn import_challenges_from_module(ctfd_client: &CTFdClient, module: &str) {}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // import all the challenges in the module
    if args.import_challenges {
        println!("Starting import of challenges");
        match args.challenges_module {
            // ideally, challenge import will be set up such that 
            Some(module) => todo!("Implement the call to the module import here"),
            None => {
                eprintln!("error: --import-challenges requires --challenges-module");
                std::process::exit(1);
            }
        }
        return;
    }

    println!("Starting watcher");

    // let ctfd_client = CTFdClient::new(args.ctfd_url, args.ctfd_api_key);

    // // rather than starting an announced solves iter here, we want a way to track what challenges have already been solved by which users... a little more complicated
    // let newchall = ctfd_client.new_challenge("test_chall_create", 5, "test", "hype new flag").await.unwrap();
    // dbg!(newchall);

    // let response = ctfd_client.get_challenges_of_category("test").await.unwrap();
    // dbg!(response);

    let pwn_college_client = PWNCollegeClient::new();

    let response = pwn_college_client.get_challenges_for_module("web-security").await.unwrap();
    dbg!(response);
    // let mut announced_solves: HashMap<i64, Vec<ChallengeSolver>> = HashMap::new();

    // let db_conn = Connection::open("ctfd_discord.sqlite3").unwrap();

    // db_conn
    // .execute("CREATE TABLE IF NOT EXISTS announced_solves (id INTEGER PRIMARY KEY AUTOINCREMENT, challenge_id INTEGER, solver_id INTEGER);", ())
    // .unwrap();

    // db_conn
    // .execute("CREATE TABLE IF NOT EXISTS top_10_teams (id INTEGER PRIMARY KEY AUTOINCREMENT, position INTEGER);", ())
    // .unwrap();

    // // Populate the announced solves hashmap with the existing solves
    // let mut statement = db_conn
    //     .prepare("SELECT challenge_id, solver_id FROM announced_solves;")
    //     .unwrap();

    // let announced_iter = statement
    //     .query_map([], |row| {
    //         Ok((
    //             row.get::<_, i64>(0).unwrap(),
    //             ChallengeSolver {
    //                 account_id: row.get::<_, i64>(1).unwrap(),
    //                 name: "".to_string(),
    //             },
    //         ))
    //     })
    //     .unwrap();

    // for announced in announced_iter {
    //     let (challenge_id, solver) = announced.unwrap();

    //     announced_solves
    //         .entry(challenge_id)
    //         .or_insert_with(Vec::new)
    //         .push(solver);
    // }

    // // Skips announcing existing solves by default
    // if args.skip_announcing_existing_solves {
    //     populate_announced_solves(&ctfd_client, &mut announced_solves).await;
    // }

    // loop {
    //     announce_solves(&http, &webhook, &ctfd_client, &mut announced_solves, &db_conn, args.announce_first_blood_only).await;
    //     //announce_top_10_overtakes(&http, &webhook, &ctfd_client, &db_conn).await;

    //     tokio::time::sleep(std::time::Duration::from_secs(args.refresh_interval_seconds)).await;
    // }
}
