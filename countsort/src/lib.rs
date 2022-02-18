/* countsort.rs

Countsort is a linear time sorting algorithm, in O(n + k),
where n = the number of integers to sort
and k = the total size of the 'alphabet' of integers.

Countsort works for any range, but we use [0, k) for simplicity.


./countsort -n 10000 -w 6 -t 10:
    Sort n randomly-generated integers,
    in the range [0, 2^w),
    and report how long it takes, on average, over t trials.
*/

use std::time::SystemTime;
use rand::{distributions::Uniform, Rng};
use pyo3::prelude::*;

// countsort has to print its counts somewhere
// but i don't want to actually do that anywhere
// so a noop, per https://stackoverflow.com/questions/42891039/
#[no_mangle]
fn noop() -> () {()}

// Sort n integers in range [0, 2^w), and report how long it took.
pub fn countsort_timing(n: u32, w: u8) -> i64 {
    // generate the random values
    // https://stackoverflow.com/questions/48218459/
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 2 << w);
    let vals: Vec<u32> = (0..n).map(|_| rng.sample(&range)).collect();

    // Count each value in vector 'counts'
    // e.g. counts[11] = 31 means '11' showed up 31 times in the values.
    let mut counts = vec![0; 2 << w];

    // Start timer  https://stackoverflow.com/questions/13322479/
    let now = SystemTime::now();
    // Sort!
    for val in vals.into_iter() {
        counts[val as usize] += 1;
    }
    // todo: Push the values out somewhere
    // i don't actually want to print these out!
    // but i need something that won't be compiled away either.
    for count in counts.into_iter() {
        for _ in 0..count {
            // nested for loop, but this inner loop sums to a total of n
            // so, no quadratic runtime here
            noop();
        }
    }
    let total_ns = now.elapsed().unwrap().as_nanos();
    //println!("... ... Sorted in {:?} ns", total_ns);
    total_ns as i64
}

pub fn countsort_trials(n: u32, w: u8, t: u128) -> f64 {
    let mut trial_times = vec![0; t as usize];
    for i in 0..t {
        //println!("... Trial {} of {}", i + 1,t);
        trial_times[i as usize] = countsort_timing(n, w);
    }

    // https://codereview.stackexchange.com/questions/173338/
    let total: i64 = trial_times.iter().sum();
    total as f64 / trial_times.len() as f64
}


#[pyfunction]
pub fn countsort_timing_python(n: u32, w: u8) -> PyResult<i64> {
    Ok(countsort_timing(n, w))
}

#[pymodule]
pub fn countsortmodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(countsort_timing_python, m)?)?;
    Ok(())
}
