use std::{
    arch::x86_64::{_mm_lfence, _rdtsc},
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

thread_local! {
    pub static PROFILE_DATA: std::cell::RefCell<HashMap<String, u64>> = std::cell::RefCell::new(HashMap::new());
}

pub fn store_profile_data(key: String, delta: u64) {
    PROFILE_DATA.with(|data| {
        let mut data = data.borrow_mut();
        let entry = data.entry(key).or_insert(0);
        *entry += delta;
    });
}

#[inline]
pub fn read_tsc() -> u64 {
    unsafe {
        _mm_lfence();
        let tsc = _rdtsc();
        _mm_lfence();
        tsc
    }
}

pub fn tsc_frequency_calibrated() -> u64 {
    let start = Instant::now();
    let tsc_start = read_tsc();

    thread::sleep(Duration::from_millis(200));

    let tsc_end = read_tsc();
    let elapsed = start.elapsed();

    let ticks = tsc_end - tsc_start;

    (ticks as f64 / elapsed.as_secs_f64()) as u64
}

pub fn ticks_to_ns(ticks: u64, tsc_hz: u64) -> u64 {
    assert!(tsc_hz > 0);

    ((ticks as u128 * 1_000_000_000) / tsc_hz as u128) as u64
}
