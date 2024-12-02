use clap::Parser;

// TODO: cleanup this?
mod day1;
mod day2;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.day {
        1 => {
            println!("Day 1");
            day1::solve_both();
        }
        2 => {
            println!("Day 2");
            let r = day2::day_2(args.part);
        }
        _ => {
            println!("Invalid day number");
        }
    };

    Ok(())
}