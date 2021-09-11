extern crate core_affinity;

use std::time::{Instant, Duration};
use std::thread;

fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(core_ids[1]);

    const SAMPLE_COUNT:usize = 200;
    const PI:f64 = 3.1415926535;
    const TOTAL_AMPLITUDE:i64 = 300;

    let mut busy_span:[f64;SAMPLE_COUNT] = [-1.0;SAMPLE_COUNT];
    let amplitude = TOTAL_AMPLITUDE / 2;

    let mut radian:f64 = 0.0;
    const RADIAN_INCREMENT:f64 = 2.0 / (SAMPLE_COUNT as f64);

    //计算时间片
    for i in 0..SAMPLE_COUNT {
        busy_span[i] = (amplitude as f64 + ((PI * radian).sin()) * amplitude as f64) as f64;
        radian = radian + RADIAN_INCREMENT;
    }

    //让CPU工作指定时间
    let mut start_time;
    let mut j = 0;
    loop {
        start_time = Instant::now();
        while start_time.elapsed().as_millis() as f64 <= busy_span[j] { };
        let d = Duration::from_millis((TOTAL_AMPLITUDE as f64 - busy_span[j]) as u64);
        thread::sleep(d);
        j = (j + 1) % SAMPLE_COUNT;
    }
}
