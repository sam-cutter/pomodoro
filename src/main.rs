use clap::Parser;

#[derive(Parser)]
struct Args {
    work: u16,
    rest: u16,
    sessions: u8,
}

fn main() {
    let args: Args = Args::parse();

    let work: &u16 = &args.work;
    let rest: &u16 = &args.rest;
    let sessions: &u8 = &args.sessions;

    let work_seconds: u16 = work * 60;
    let rest_seconds: u16 = rest * 60;
    let session_seconds: u16 = work_seconds + rest_seconds;

    println!("You will work {sessions} sessions, each containing {work} minutes of work, and {rest} minutes of rest.");
    println!("You will work {sessions} sessions, each containing {work_seconds} seconds of work, and {rest_seconds} seconds of rest.");
    println!("Each session will be {session_seconds} seconds long.")
}
