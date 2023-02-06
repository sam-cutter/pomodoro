use clap::Parser;
use std::{thread, time};

#[derive(Parser)]
struct Args {
    work: u16,
    rest: u16,
    sessions: u8,
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
            println!("There are {work_seconds_left} seconds left of working.");
            thread::sleep(time::Duration::from_secs(1));
        }

        for rest_seconds_left in (0..=rest_seconds).rev() {
            println!("There are {rest_seconds_left} seconds left of resting.");
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
