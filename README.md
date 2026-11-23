# UATester
## What is this?
UATester (User Agent Tester) is a lightweight commandline utility for testing how a webserver might respond
differently to a user claiming to be connecting from various browsers or web tools. It currently does this by
comparing the status codes that the server sends back, but this will be expanded to body content in the future.

## How do I use it?
```
$ uatester --help
UATester -- Brute force some user agents
(c) 2026 Jesse Gonyou

Usage:
      uatester <url> <agents-file> [success-status ...]
```

UATest requires an `agents-file`, which is a text file with a different browser's user agent on each line. While
no `agents-file` is distributed with this repository, you can find other lists in this format very easily
elsewhere online, such as [https://gist.github.com/pzb/b4b6f57144aea7827ae4](this one).

The `success-status` field of the command line options is a space-separated list of various HTTP status codes
that the program will display as successful status codes. User agents that result in a successful status code
will be displayed in stdout.

## Why does it exist?
This tool was created primarily for my personal use in CTF (capture the flag) challenges. Most webservers do
not actually respond differently to different user agents, or the differences are not visible in the way that
this tool can detect in its current form, so its current primary use is in CTF challenges.

## How do I build it?
Make sure that you have the Rust toolchain installed.

CD into the project's base directory and run the following command:

```
$ cargo build --release
```

That's it! The new binary will be installed in the `target/release` directory.

## What's next?
This project is actually quite old (I made it back in 2024), which is part of why some of the coding practices
aren't exactly up to snuff. If I revisit it in the future, it will likely consist of cleaning up main.rs,
making the program more modular, including a proper command line argument parser, and implementing a more
complete comparison of the different responses that different user agents produce in the webserver, as opposed
to just their response codes.
