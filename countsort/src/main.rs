use clap::Parser;
pub mod lib;

/// Parse arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 100)]
    n: u32, 
    #[clap(short, long, default_value_t = 12)]
    w: u8,
    #[clap(short, long, default_value_t = 1)]
    t: u128,
}

fn main () {
    let args = Args::parse();
    println!(
        "Sorting {} {}-bit ints over {} trials",
        args.n, args.w, args.t
    );

    let average = lib::countsort_trials(args.n, args.w, args.t);
    println!("{:.0} ns avg", average);

}