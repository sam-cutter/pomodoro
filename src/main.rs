use clap::Parser;
use std::{thread, time};

#[derive(Parser)]
struct Args {
    work: u16,
    rest: u16,
    sessions: u8,
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

fn main() {
    // Parse arguments
    let args: Args = Args::parse();

    let work_minutes: &u16 = &args.work;
    let rest_minutes: &u16 = &args.rest;
    let sessions: &u8 = &args.sessions;

    // Convert minute time values into seconds to count down
    let work_seconds: u16 = work_minutes * 60;
    let rest_seconds: u16 = rest_minutes * 60;

    // For each session, run through work and rest subsessions
    for session in 1..=*sessions {
        println!("Session {session}");

        for work_seconds_left in (0..=work_seconds).rev() {
            let time_left_string: String = get_time_left_string(work_seconds_left);
            println!("{time_left_string} of work remaining.");
            thread::sleep(time::Duration::from_secs(1));
        }

        for rest_seconds_left in (0..=rest_seconds).rev() {
            let time_left_string: String = get_time_left_string(rest_seconds_left);
            println!("{time_left_string} of work remaining.");
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
