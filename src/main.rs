#![allow(dead_code)]

mod ctfd;
mod pwncollege;

use ctfd::{CTFdClient, ChallengeSolver, TeamId, TeamPosition};
use pwncollege::PWNCollegeClient;

use clap::Parser;
use rand::Rng;
use rusqlite::{Connection, params};

/// A Discord webhook bot to announce CTFd solves
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Toggle for challenge import
    #[arg(short, long, default_value = "false")]
    import_challenges: bool,

    /// Challenge module from which to import - only required with import
    #[arg(long, short = 'm')]
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

async fn import_challenges_from_module(
    ctfd_client: &CTFdClient,
    pwn_college_client: &PWNCollegeClient,
    db_conn: &Connection,
    module: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match db_conn.query_row(
        "SELECT 1 FROM challenge_categories WHERE module_id = ?1 LIMIT 1",
        params![&module],
        |_row| Ok(true),
    ) {
        Ok(_) => {
            eprintln!(
                "{} already exists in the database. Please manually remove it before re-adding its challenges",
                &module
            );
            return Ok(());
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {}
        Err(e) => return Err(e.into()),
    };

    // I'll want to first gather all of the challenge names, then iterate through and add them
    let challenges = pwn_college_client
        .get_challenges_for_module(&module)
        .await
        .unwrap();
    let module_name = pwn_college_client
        .pretty_print_module(&module)
        .await
        .unwrap();

    let rand_flag: [u8; 32] = rand::rng().random();
    let flag = hex::encode(rand_flag);

    for challenge in &challenges {
        ctfd_client
            .new_challenge(&challenge, &module_name, &flag)
            .await
            .unwrap()
    }

    db_conn
        .execute(
            "INSERT INTO challenge_categories (module_id, flag) VALUES (?1, ?2);",
            (&module, &flag),
        )
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let pwn_college_client = PWNCollegeClient::new();
    let ctfd_client = CTFdClient::new(args.ctfd_url, args.ctfd_api_key);

    let db_conn = Connection::open("ctfd_pwn_college.sqlite3").unwrap();

    db_conn
        .execute("CREATE TABLE IF NOT EXISTS challenge_categories (id INTEGER PRIMARY KEY AUTOINCREMENT, module_id VARCHAR(25), flag VARCHAR(80));", ())
        .unwrap();

    // import all the challenges in the module
    if args.import_challenges {
        println!("Starting import of challenges");
        match args.challenges_module {
            // ideally, challenge import will be set up such that
            Some(module) => {
                import_challenges_from_module(&ctfd_client, &pwn_college_client, &db_conn, &module)
                    .await
                    .unwrap();
            }
            None => {
                eprintln!("error: --import-challenges requires --challenges-module");
                std::process::exit(1);
            }
        }
        return;
    }

    println!("Starting watcher");

    // let ctfd_client = CTFdClient::new(args.ctfd_url, args.ctfd_api_key);

    // rather than starting an announced solves iter here, we want a way to track what
    //      challenges have already been solved by which users... a little more complicated
    // let newchall = ctfd_client.new_challenge("test_chall_create", 5, "test", "hype new flag").await.unwrap();
    // dbg!(newchall);

    // let response = ctfd_client.get_challenges_of_category("test").await.unwrap();
    // dbg!(response);

    let pwn_college_client = PWNCollegeClient::new();

    let response = pwn_college_client
        .get_challenges_for_module("web-security")
        .await
        .unwrap();
    dbg!(response);

    // transform this into a capture of existing challenges / flags
    //  eventually, we'll have two items. The flag vector and then a solves one with a students object or something
    // let mut announced_solves: HashMap<i64, Vec<ChallengeSolver>> = HashMap::new();

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
