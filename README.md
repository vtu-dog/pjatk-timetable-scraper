# pjatk-timetable-scraper
PJATK lecture timetable scraper

- [Overview](#overview)
- [Installation](#installation)
- [Running the project](#running-the-project)
- [Additional info](#additional-info)

## Overview
I quickly grew weary of having to constantly log into my university's website to peek at the lecture timetable, since it logs you out every few minutes. `pjatk-timetable-scraper` aims to alleviate this problem by scraping the timetable off the servers and pretty-printing it to terminal output.

## Installation
You'll need to [install Rust](https://www.rust-lang.org/tools/install) and build the project yourself, since the website credentials are added to the binary at compile time.

Once you've installed the Rust toolchain, you'll need to rename `.env_example` to `.env`, populate it with your credentials, and run `sudo cargo install --path . --root /usr/local/` in the project directory.

You're all set!

## Running the project

Simply type `pjatk-timetable-scraper` into your favourite terminal. For more info, check out `pjatk-timetable-scraper --help`.

If I were you, I'd probably set up a shorthand by adding `alias plan='pjatk-timetable-scraper'` to your shell profile.

Enjoy!

## Additional info
The project was tested using Rust 1.56.0 (Stable) on macOS 12.0.1 Monterey (arm64).
