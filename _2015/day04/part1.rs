use _2015_day04::{Md5HexIter, KEY};
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    let mut md5_iter = Md5HexIter::new(KEY.to_string(), 1..);
    while let Some(md5) = md5_iter.next() {
        if md5[0] == 0 && md5[1] == 0 && md5[2] == 0 {
            println!("{}", md5_iter.current);
            break;
        }
    }

    let elapsed: Duration = start.elapsed();
    println!("{elapsed:?}")
}
