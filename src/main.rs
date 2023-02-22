//#![allow(unused_imports)]
//#![allow(dead_code)]

use clap::Parser;
use tracing::*;

pub mod cli;
pub mod logger;

fn main() {
    // Parse the command line arguments
    let args = cli::Args::parse();

    // Initialize tracing/logging
    let _tracing_file_guard = logger::setup_tracing(&args);

    // Create span marker for main
    let _trace_span = info_span!("main");

    // Program Logic

    // // Create a vector of random numbers
    // let vals: Vec<u32> = (0..100)
    //     .map(|_| rand::thread_rng().gen_range(0..100))
    //     .collect();

    // // Log the values in the vector
    // vals.par_iter().for_each(|x| {
    //     let _trace_span = trace_span!("par_for_each");
    //     debug!(x);
    // });
}
