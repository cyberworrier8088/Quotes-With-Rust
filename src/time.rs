// This is the time module for checking time and date :)
// It will check the time every 60 seconds and print the day if it changes

use chrono::Local; // importing chrono crate for getting current time
use std::{thread, time::Duration}; // importing thread and time::Duration for sleeping. this is standard library in rust

pub fn get_day() -> String { // getting the current day

    Local::now().format("%A").to_string() // formatting the current day, and returning it as a string, %A is for full day name

}
// %Y-%m-%d is for year-month-day format
pub fn time_checker() { // checking the time every 60 second
    println!("Hello! Today is {}", get_day());

    let mut last_day = Local::now().format("%Y-%m-%d").to_string(); // getting the current date, and storing it in last_day variable

    loop {
        let today = Local::now().format("%Y-%m-%d").to_string(); // getting the current date

        if today != last_day { // if the current date is not the same as the last date
            println!("Hello! Today is {}", get_day()); // print the current day

            last_day = today; // update the last day
        }

        thread::sleep(Duration::from_secs(60)); // sleep for 60 seconds
    }
}
