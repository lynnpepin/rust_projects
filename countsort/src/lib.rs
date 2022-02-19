/* countsort.rs

Countsort is a linear time sorting algorithm, in O(n + k),
where n = the number of integers to sort
and k = the total size of the 'alphabet' of integers.

Countsort works for any range, but we use [0, k) for simplicity.

*/

use std::time::SystemTime;
use rand::{distributions::Uniform, Rng};
use pyo3::prelude::*;

pub fn countsort(vals: Vec<u32>, w: u8) -> Vec<u32> {
    // 1. Count all the values
    //     e.g. counts[11] = 31 means '11' showed up 31 times in the values.
    let mut counts = vec![0; 2 << w];
    let mut sorted: Vec<u32> = vec![0; vals.len()];

    for val in vals.into_iter() {
        counts[val as usize] += 1;
    }

    // 2. Sort into 'sorted'
    let mut idx = 0;
    for (ci, cc) in counts.into_iter().enumerate() {
        // ci = the key, cc = the number of times (counts) it appears
        // e.g. for ci, cc = 4, 3, it means 4 appears 3 times in the unsorted list.
        for _ in 0..cc {
            sorted[idx] = ci as u32;
            idx += 1;
        }
    }

    sorted
}

// Sort n integers in range [0, 2^w), and report how long it took.
pub fn countsort_timing(n: u32, w: u8) -> i64 {
    // generate the random values
    // https://stackoverflow.com/questions/48218459/
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 2 << w);
    let random_vals: Vec<u32> = (0..n).map(|_| rng.sample(&range)).collect();

    // Start timer  https://stackoverflow.com/questions/13322479/
    let now = SystemTime::now();
    // Sort!
    let sorted = countsort(random_vals, w);
    // todo: put call in here
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

#[pyfunction]
pub fn countsort_trials_python(n: u32, w: u8, t: u128) -> PyResult<f64> {
    Ok(countsort_trials(n, w, t))
}

#[pyfunction]
pub fn countsort_python(vals: Vec<u32>, w: u8) -> PyResult<Vec<u32>> {
    Ok(countsort(vals, w))
}

#[pymodule]
pub fn countsortmodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(countsort_timing_python, m)?)?;
    m.add_function(wrap_pyfunction!(countsort_trials_python, m)?)?; 
    m.add_function(wrap_pyfunction!(countsort_python, m)?)?; 
    Ok(())
}
