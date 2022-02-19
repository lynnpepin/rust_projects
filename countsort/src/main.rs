use clap::Parser;
pub mod lib;
/*
./countsort -n 10000 -w 6 -t 10:
    Sort n randomly-generated integers,
    in the range [0, 2^w),
    and report how long it takes, on average, over t trials.

*/
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