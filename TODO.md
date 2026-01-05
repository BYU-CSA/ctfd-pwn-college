Design goals:
- I want to have a tool (command line tool) to add challenges to CTFd from a pwn college dojo by name
- I want to have a tool to scrape (every minute or two) pwn college for each challenge for updated solves to that challenge. Can I get most recent?
- That tool should then update CTFd solves per user

I'll need type APIs for the major kinds of items I'll be sending, specifically to serialize and deserialize items for the API
Getting started with the existing API from https://github.com/jordanbertasso/ctfd-solve-announcer-discord/blob/main/src/ctfd.rs would be a good idea
Honestly, they're ctfd.rs is great as-is with likely some additional functions in impls to do solve-challenge functionality and whatever else
Then I can add a college.rs to add functionality for interacting with pwn college

OH I can use a db for all of the tracking of solves, that will be pretty chill
The final code will have APIs for all of the interaction with the respective APIs, then functions in the main.rs file will handle the high-level actions required on top of those APIs

Probably make a separate repo (in BYU CSA) but reference the fact that much of the base code is copied from the original repo
Add a license - MIT

Command line to import challs:
- ~~`/<dojo>/modules` can get all modules (including all challenges)~~
- ~~Just parse through challenges, make a category name, and then create all of the challenges matching the point values~~
- ~~CTFd `challenge post-challenge-list`. Make sure to make a standard flag for the specific module and then store that. Make a persistent challenge list of some kind - track flag by category? Vs by challenge?~~

Find solves:
- `/<dojo>/solves?username=` can get you solves for a specific user on a specific dojo
- Gather list of users from CTFd, then go through and get solves for each user. Is it possible to get a list of solves like we do for first blood but per-user? It might take a lot of space. Maybe the reverse? For each challenge a list of non-solves
- Alternatively can use `/users/{uid}/solves` if it would be easier to track per-user 

Update solves:
- Need to do a submission for the user by hitting CTFd submissions post-submission


The flow of the program sucks... what do I need to do and what is the optimal way to do it?
- Start by checking the sqlite database to gather CTFd state
    - The internal data structures to rust will *also* track CTFd state. Gathered PWN college state is used for updating that CTFd state only
- Each iteration will:
    - Gather the CTFd state of *users existing* (user endpoint) only and make necessary updates to the data structures and sqlite
    - Gather pwn college state for each user, then check and make sure state matches from before
        - Make any necessary updates to CTFd for new challenge solves
        - Potentially also just check solves, and since they come with timestamps, you can convert timestamps so you don't even need to track. Just any in the last 60 seconds should be submitted??? That would make it *really* easy...