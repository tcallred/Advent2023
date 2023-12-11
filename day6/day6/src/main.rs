fn ways_to_beat_record(time: i64, record_dist: i64) -> i64 {
    let mut ways = 0;
    for hold_time in 0..=time {
        let run_time = time - hold_time;
        let dist = hold_time * run_time;
        if dist > record_dist {
            ways += 1;
        }
    }

    ways
}

fn main() {
    let td = vec![(34, 204), (90, 1713), (89, 1210), (86, 1780)];
    let mut prod = 1;
    for (t, d) in td.iter() {
        prod *= ways_to_beat_record(*t, *d);
    }
    println!("{}", prod);

    println!("{}", ways_to_beat_record(71530, 940200));
    println!("{}", ways_to_beat_record(34908986, 204171312101780));
}
