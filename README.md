# CTFd PWN College

This is a tool being built to integrate certain PWN college challenges into our Cyber Kickstart club but also have a private scoreboard through CTFd such that we can track solves relatively easily.

Inspiration and starting code was taken from https://github.com/jordanbertasso/ctfd-solve-announcer-discord

### Usage

Once the program is built, running the challenge importer is pretty simple. Just run:
```sh
ctfd-pwn-college -c $CTFD_URL -a $CTFD_API_KEY -i -m "web-security" 
```

Make sure you have a CTFd URL and API Key already provisioned (I source them from a `.env` file). The `web-security` is an example of a module in the Orange Belt (Intro to Cybersecurity). You can pick any Orange Belt module and use the name as referenced by the URL of that module (https://pwn.college/intro-to-cybersecurity/web-security/).

The `--help` flag is active, so any questions can be directed there or in my dms