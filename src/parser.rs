//! iCal timetable parser.

use super::event::Event;

use anyhow::Context;
use chrono::{DateTime, Duration, Local, NaiveDateTime, Offset, TimeZone};

/// Parses a raw iCal file.
pub fn parse_ical<B>(buf: B) -> Result<Vec<Event>, anyhow::Error>
where
    B: std::io::BufRead,
{
    // load a raw buffer into an iCal parser
    let ical_events = ical::IcalParser::new(buf)
        .next()
        .context("ical buffer is empty")?
        .context("ical parsing failed")?
        .events;

    // create an empty vector of events
    let mut events = Vec::new();

    // populate the vector of events
    for event in ical_events {
        // create empty bindings of event details
        let mut from = None;
        let mut to = None;
        let mut summary = None;

        // populate the bindings
        for prop in event.properties {
            match prop.name.as_str() {
                "DTSTART" => from = prop.value,
                "DTEND" => to = prop.value,
                "SUMMARY" => summary = prop.value,
                _ => (),
            }
        }

        // unwrap the bindings
        let from = parse_date(from.context("DTSTART value is null")?)?;
        let to = parse_date(to.context("DTEND value is null")?)?;

        // split the summary into event name and location
        let mut summary = summary
            .context("SUMMARY value is null")?
            .splitn(2, " s. ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        // if the summary string is malformed, bail
        if summary.len() != 2 {
            anyhow::bail!("summary has length != 2");
        }

        // split the resulting event name into name and type
        let event_str = summary.remove(0);
        let mut event_split = event_str
            .splitn(2, ' ')
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        // if the event split string is malformed, bail
        if event_split.len() != 2 {
            anyhow::bail!("event_split has length != 2");
        }

        // populate the event vector with a new entry
        events.push(Event {
            event_name: event_split.remove(0),
            event_type: event_split.remove(0),
            location: summary.remove(0),
            from,
            to,
        });
    }

    // return the event vector
    Ok(events)
}

/// Parses a date from an iCal date string into a chrono DateTime with a local timezone.
fn parse_date(s: String) -> Result<DateTime<Local>, anyhow::Error> {
    // if the date string is malformed, bail
    if !s.contains('T') {
        anyhow::bail!("invalid datetime string: {}", s);
    }

    // remove redundant string flags
    let date_str = s.replace('T', "").replace('Z', "");

    // create a naive datetime from the date string
    let naive_date = NaiveDateTime::parse_from_str(&date_str, "%Y%m%d%H%M%S")
        .context("failed to parse naive datetime")?;

    // create a local datetime from the naive datetime
    let offset = Duration::seconds(Local.timestamp(0, 0).offset().fix().local_minus_utc() as i64);
    let adjusted_date = Local.from_local_datetime(&naive_date).unwrap() + offset;

    // return the adjusted datetime
    Ok(adjusted_date)
}
