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

    println!("You will work {sessions} sessions, each containing {work} minutes of work, and {rest} minutes of rest.")
}
