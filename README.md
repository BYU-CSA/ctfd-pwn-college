# CTFd PWN College

So I completely changed this. I wanted a tool that would fetch information from pwn college and use that information to auto-complete challenges for users. I was working on a bot to do so when I realized that I couldn't get a bot to submit challenges onto CTFd. At that point, I switched to writing a plugin, which ended up being significanly simpler. To write it, I had to just reference the source code for CTFd for types and such because the docs are quite poor.

## Plugin

The plugin should be placed in the CTFd in `CTFd/plugins/external-solve-sync`. This will ensure that CTFd automatically sources and runs it. It exposes a `/sync` endpoint on the CTFd that can only be run by an admin account that syncs the solves with the remote instance, pwn.college.

The program expects both `CTFD_URL` and `CTFD_API_KEY` to be valid environmental variables. You can set these manually or include them in a `.env` file.

## Other utilities

`run_sync.py` automatically run a sync command using the same sourced environmental variables as above. This can be used in a cron job to automatically sync every once in a while (for example, once per minute).

`add_module.py` expects one command line argument, a module name from Intro to Cybersecurity in pwn.college. This function will gather all of the challenges from that module and upload them to CTFd in a separate category. These options include:
- `web-security`
- `intercepting-communication`
- `cryptography`
- `access-control`
- `reverse-engineering`
- `binary-exploitation`
- `integrated-security`
