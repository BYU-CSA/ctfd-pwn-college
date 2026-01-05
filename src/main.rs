#![allow(dead_code)]

mod ctfd;
mod db;
mod pwncollege;

use ctfd::CTFdClient;
use db::DB;
use pwncollege::PWNCollegeClient;

use clap::Parser;
use rand::Rng;

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
    db: &mut DB,
    module: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if db.flag_exists(&module).await {
        eprintln!("Flag already exists for {module}. Please delete it before resubmitting");
        return Ok(());
    }
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
        // make the new challenge on CTFd
        let id = ctfd_client
            .new_challenge(&challenge, &module_name, &flag)
            .await
            .unwrap();

        // also add the challenge to the database
        db.insert_challenge(id, &challenge, &module).await.unwrap();
    }

    db.insert_flag(&module, &flag).await.unwrap();

    todo!(
        r#"When you first add challenges, also parse for all time to catch old solves for the modules.
        "On each user add, you will need to do the same thing."#
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let pwn_college_client = PWNCollegeClient::new();
    let ctfd_client = CTFdClient::new(args.ctfd_url, args.ctfd_api_key);
    let mut db = DB::new("ctfd_pwn_college.sqlite3").unwrap();

    // init_db(&db_conn).await;

    // import all the challenges in the module
    if args.import_challenges {
        println!("Starting import of challenges");
        match args.challenges_module {
            // ideally, challenge import will be set up such that
            Some(module) => {
                import_challenges_from_module(&ctfd_client, &pwn_college_client, &mut db, &module)
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
        .get_recent_solves_by_user_for_module("intro-to-cybersecurity", "web-security", "overllama")
        .await
        .unwrap();
    dbg!(&response);

    r#"
    Each tick of the sm should take the following steps:
        - Gather existing users from CTFd and update as necessary
        - For each user
            - Gather solves
            - Check whether there are solves from the last 2 minutes
            - Update CTFd accordingly
    "#;

    // transform this into a capture of existing challenges / flags
    //  eventually, we'll have two items. The flag vector and then a solves one with a students object or something
    // let mut username_to_id: HashMap<String, UserId> = HashMap::new();
    // let mut challenge_name_to_id: HashMap<String, ChallengeId> = HashMap::new();

    // // Main relationships
    // let mut user_solves: HashMap<UserId, HashSet<ChallengeId>> = HashMap::new();

    // // I *really* should have mapped this out...
    // // Probably two tables: challenges + users. Then a third to map the many to many relationship of solves
    // let mut gather_users = db_conn.prepare("SELECT id, username FROM users").unwrap();
    // let usernames: Vec<(UserId, String)> = gather_users
    //     .query_map([], |row| {
    //         Ok((
    //             row.get::<_, UserId>(0)?,
    //             row.get::<_, String>(1)?,
    //         ))
    //     })
    //     .unwrap()
    //     .collect::<Result<Vec<_>, _>>()
    //     .unwrap();

    // for user_data in usernames {
    //     username_to_id.insert(user_data.1.to_string(), user_data.0);
    // }

    // let mut gather_challenges = db_conn.prepare("SELECT id, challenge_name FROM challenges").unwrap();
    // let challenges: Vec<(ChallengeId, String)> = gather_challenges
    //     .query_map([], |row| {
    //         Ok((
    //             row.get::<_, ChallengeId>(0)?, // id
    //             row.get::<_, String>(1)?, // challenge_name
    //         ))
    //     })
    //     .unwrap()
    //     .collect::<Result<Vec<_>, _>>()
    //     .unwrap();

    // for challenge_data in challenges {
    //     // both make sure the module exists and insert the inner data
    //     challenge_name_to_id.insert(challenge_data.1.to_string(), challenge_data.0);
    // }

    // dbg!(challenge_name_to_id);

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
