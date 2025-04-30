use _2015_day04::KEY;
use md5::compute;
use rayon::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let perf_count = Instant::now();

    fn md5_has_six_leading_zeros(n: &u32) -> bool {
        let md5: [u8; 16] = compute(format!("{}{}", KEY, n)).0;
        md5[0] == 0 && md5[1] == 0 && md5[2] == 0
    }

    const SLICE_SIZE: u32 = 500_000;
    let mut start: u32 = 0;
    let mut end: u32 = start + SLICE_SIZE;
    loop {
        if let Some(result) = (start..end)
            .into_par_iter()
            .find_first(md5_has_six_leading_zeros)
        {
            println!("{result:?}");
            break;
        } else {
            start = end;
            end += SLICE_SIZE;
        }
    }

    let elapsed: Duration = perf_count.elapsed();
    println!("{elapsed:?}")
}
