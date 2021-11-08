//! Timetable scraper.

use std::io::Cursor;

use anyhow::Context;
use bytes::Bytes;
use scraper::{Html, Selector};

const URL_1: &str = "https://planzajec.pjwstk.edu.pl/Logowanie.aspx";
const URL_2: &str = "https://planzajec.pjwstk.edu.pl/TwojPlan.aspx";

/// Scrapes the timetable using the given credentials.
pub fn scrape_ical(
    username: &'static str,
    passwd: &'static str,
) -> Result<Cursor<Bytes>, anyhow::Error> {
    // create a reqwest client with cookies enabled
    let client = reqwest::blocking::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .context("failed to invoke ClientBuilder::new().build()")?;

    // fetch and parse the login page
    let html = Html::parse_document(
        &client
            .get(URL_1)
            .send()
            .context("failed to invoke get(URL_1).send()")?
            .text()
            .context("failed to invoke get(URL_1).text()")?,
    );

    // create an input selector
    let input_selector = Selector::parse("input").unwrap();

    // create a multipart login form
    let mut multipart = reqwest::blocking::multipart::Form::new();

    // add the username and password to the form, filling hidden fields with default values
    for element in html.select(&input_selector).map(|e| e.value()) {
        // get the `name` and `value` attributes of a given input element
        let name = element
            .attr("name")
            .context("element attr \"name\" not found in multipart 1")?
            .to_owned();

        let value = element.attr("value").unwrap_or("").to_owned();

        let lower_name = name.to_lowercase();

        // set the input fields to correct values
        if lower_name.contains("username") {
            multipart = multipart.text(name, username.clone());
        } else if lower_name.contains("password") {
            multipart = multipart.text(name, passwd.clone());
        } else {
            multipart = multipart.text(name, value);
        }
    }

    // send the login form and parse the response
    let html = Html::parse_document(
        &client
            .post(URL_1)
            .multipart(multipart)
            .send()
            .context("failed to invoke post(URL_1).multipart().send()")?
            .text()
            .context("failed to invoke post(URL_1).text()")?,
    );

    // create a multipart download form
    let mut multipart = reqwest::blocking::multipart::Form::new();

    // simulate a button click, filling hidden fields with default values
    for element in html.select(&input_selector).map(|e| e.value()) {
        // get the `name` and `value` attributes of a given input element
        let name = element
            .attr("name")
            .context("element attr \"name\" not found in multipart 2")?
            .to_owned();

        let value = element.attr("value").unwrap_or("").to_owned();

        // set the input fields to correct values
        if name.contains("PlanStudenta$CalendarICalExportButton_input") {
            multipart = multipart.text(name, "Eksportuj do iCalendar");
        } else if element.attr("type").unwrap_or("") == "hidden" {
            multipart = multipart.text(name, value);
        }
    }

    // send the download form and parse the response
    let bytes = client
        .post(URL_2)
        .multipart(multipart)
        .send()
        .context("failed to invoke post(URL_2).multipart().send()")?
        .bytes()
        .context("failed to invoke bytes()")?;

    // return the in-memory file
    Ok(Cursor::new(bytes))
}
