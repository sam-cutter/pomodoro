use clap::Parser;
use std::io::{stdout, Write};
use std::{thread, time};

#[derive(Parser)]
struct Args {
    work: u16,
    rest: u16,
    sessions: u8,
}

// This function sleeps for a specified amount of seconds
fn sleep(seconds: u8) {
    thread::sleep(time::Duration::from_secs(seconds.into()));
}

// This function converts an amount of seconds into hours, minutes and secons for user display
fn get_time_left_string(seconds: u16) -> String {
    let hours: u16 = seconds / 3600;
    let remaining_minutes: u16 = (seconds % 3600) / 60;
    let remaining_seconds: u16 = seconds % 60;

    let time_left_string: String = format!(
        "{} hours, {} minutes, and {} seconds",
        hours, remaining_minutes, remaining_seconds
    );

    return time_left_string;
}

// This function takes 2 integers and returns the rounded percentage
fn get_percentage(a: u16, b: u16) -> u16 {
    let percentage: u16 = (a as f32 / b as f32 * 100.0).round() as u16;

    return percentage;
}

// This function displays a countdown snapshot
fn display_countdown_snapshot(seconds_left: u16, seconds_total: u16, subsession: &str) {
    print!("\r{}", " ".repeat(100));

    let time_left_string: String = get_time_left_string(seconds_left);

    let seconds_done: u16 = seconds_total - seconds_left;
    let percentage_done = get_percentage(seconds_done, seconds_total);

    print!("\r{time_left_string} of {subsession} remaining. You are {percentage_done}% done.");
    stdout().flush().unwrap();

    sleep(1);
}

fn main() {
    // Parse arguments
    let args: Args = Args::parse();

    let work_minutes: &u16 = &args.work;
    let rest_minutes: &u16 = &args.rest;
    let sessions: &u8 = &args.sessions;

    // Convert minute time values into seconds to make counting down easier
    let work_seconds: u16 = work_minutes * 60;
    let rest_seconds: u16 = rest_minutes * 60;

    // For each session, run through work and rest subsessions
    for session in 1..=*sessions {
        println!("Session {session}");

        for work_seconds_left in (0..=work_seconds).rev() {
            display_countdown_snapshot(work_seconds_left, work_seconds, "work")
        }

        for rest_seconds_left in (0..=rest_seconds).rev() {
            display_countdown_snapshot(rest_seconds_left, rest_seconds, "rest")
        }

        println!();
    }
}
