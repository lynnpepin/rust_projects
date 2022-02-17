/* 02_countsort.rs

Goal: Time how long it takes to sort N unsigned ints of word size W, over T trials.

E.g. ./countsort 1000000 4 10 would:

Init array of float64s of size T
Over 10 trials:
    Generate zeroes of size  [0 ... 2**4]
    Generate a list of 1000000 uint8s in range [0, ... 2**4 -1]
    Start timer
        Count each element in the lest
    Stop timer
    Add time to timer array

Report mean, std, min, and max sort time
*/

use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;


// Round up to the nearest integer size.
fn real_wordsize(ww: i8) -> i8 {
    if ww <= 8 {
        return 8;
    }
    else if ww <= 16 {
        return 16;
    }
    else if ww <= 24 {
        return 32;
    }
    else {
        println!("!!! WORD SIZE TOO BIG. 24 is the max.");
        // todo: Raise error properly
        return 0;
    }
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main () {
    const NN: usize = 1000000;  // Number of elements to generate
    const WW: i8 = 12;       // Number of bits in the unsigned int
    const TT: usize = 10;       // Number of trials to perform
    
    // todo: How to set type of generated elements programatically? Just assume i16 for now
    let true_WW: i8 = real_wordsize(WW);  // Real wordsize. (E.g. 6-bit uints will be represented using u8s.)

    println!("Sorting {} {}-bit ints (u{}) over {} trials", NN, WW, true_WW, TT);
    let trial_times: [i64; TT] = [0; TT];

    for tt in 0..TT {
        println!("Trial {} of {}", tt, TT);
        // generate random integers
        // sort them
    }
    let counts: [u64; NN] = [0; NN];

}