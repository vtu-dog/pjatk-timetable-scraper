//! PJATK timetable scraper.

mod event;
mod parser;
mod scraper;

#[macro_use] // needed for the `row!` macro
extern crate prettytable;

#[macro_use] // needed for the `app_from_crate!` macro
extern crate clap;

use anyhow::Context;
use chrono::{DateTime, Duration, Local};
use load_dotenv::load_dotenv;
use prettytable::{row, Cell, Row, Table};

/// Application entrypoint.
fn main() {
    // load environment variables from .env during compilation
    load_dotenv!();

    // parse command line arguments
    let args = app_from_crate!()
        .arg(
            clap::Arg::with_name("all")
                .short("a")
                .long("all")
                .multiple(false)
                .help("Shows all timetable entries without filtering"),
        )
        .get_matches();

    // start the program and capture errors
    if let Err(e) = main_wrapper(args.is_present("all")) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

/// Main program logic.
fn main_wrapper(disable_filtering: bool) -> Result<(), anyhow::Error> {
    // bind environment variables
    let username = env!("USERNAME");
    let passwd = env!("PASSWORD");

    // scrape the timetable
    let ical = scraper::scrape_ical(username, passwd)
        .context("failed to invoke scraper::scrape_ical()")?;

    // parse the timetable
    let mut events = parser::parse_ical(ical).context("failed to invoke parser::parse_ical()")?;

    // if filtering is enabled...
    if !disable_filtering {
        // ...filter irrelevant events
        events = filter_events(events);
    }

    // if there are no events to be shown, exit the program
    // will only happen during holidays when filtering is enabled
    if events.is_empty() {
        anyhow::bail!("no events to be shown");
    }

    // create a pretty table...
    let mut table = Table::new();

    // ...name its headers...
    table.set_titles(Row::new(vec![
        Cell::new("#"),
        Cell::new("przedmiot"),
        Cell::new("rodzaj"),
        Cell::new("sala"),
        Cell::new("data"),
    ]));

    // ...and populate it with events
    for (i, event) in events.into_iter().enumerate() {
        table.add_row(row![
            i + 1,
            event.event_name,
            event.event_type,
            event.location,
            format_date(event.from, event.to),
        ]);
    }

    // print the table
    table.printstd();

    // no error to return
    Ok(())
}

/// Filters irrelevant events from the given list of events.
fn filter_events(events: Vec<event::Event>) -> Vec<event::Event> {
    events
        .into_iter()
        .filter(|event| event.in_day_range(Duration::zero(), Duration::days(14)))
        .collect::<Vec<_>>()
}

/// Formats the given date range into a human-readable string.
fn format_date(from: DateTime<Local>, to: DateTime<Local>) -> String {
    format!("{}-{}", from.format("%Y-%m-%d, %H:%M"), to.format("%H:%M"))
}
