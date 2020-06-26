#![allow(non_snake_case)]

mod cli;

use threadpool::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::{Instant};
use byte_unit::Byte;

fn main() {
    let args = cli::cli().get_matches();
    let path = PathBuf::from(args.value_of("output_folder").unwrap());
    let n_jobs = args.value_of("threads").unwrap().parse::<usize>().expect("Wrong blocksize argument value");
    let blocksize = args.value_of("blocksize").unwrap().parse::<u32>().expect("Wrong blocksize argument value");
    let blocknum = args.value_of("blocknum").unwrap().parse::<u32>().expect("Wrong blocknum argument value");
    let n_workers = n_jobs * 2;

    let pool = ThreadPool::new(n_workers);

    for i in 0..n_jobs {
        let path = path.clone();
        pool.execute(move || {
            let filename = format!("{}/foo{}.txt", path.to_str().unwrap(), i);
            println!("Spawned thread {}, writing to {}", i, filename);
            let now = Instant::now();
            let mut file = File::create(filename).expect("Cannot create file");
            let random_bytes: Vec<u8> = (0..blocksize).map(|_| {rand::random::<u8>()}).collect();
            for _ in 0..blocknum {
                file.write_all(&random_bytes).expect("Cannot write to file");
            }
            file.sync_all().expect("Cannot sync the file");
            let elapsed = now.elapsed();
            println!("Finished thread {} in {}.{}s, wrote {}",
                        i,
                        elapsed.as_secs(),
                        elapsed.as_millis()%1000,
                        Byte::from_bytes((blocksize*blocknum).into()).get_appropriate_unit(false).to_string()
                    );
        });
    }
    pool.join();
}
