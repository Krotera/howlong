use howlong::{clock::*, Clock, Duration};
use std::thread;

mod utils;

macro_rules! test_clock {
    ($name: ident, $clock: ty) => {
        #[test]
        fn $name() {
            let ten_millis = Duration::from_millis(10);
            let start = <$clock>::now();
            thread::sleep(ten_millis);
            let elapsed = <$clock>::now() - start;
            assert!(elapsed >= ten_millis);
        }
    };
}

test_clock!(test_system_clock, SystemClock);
#[cfg(have_steady_clock)]
test_clock!(test_steady_clock, SteadyClock);
test_clock!(test_high_resolution_clock, HighResolutionClock);
test_clock!(test_process_real_cpu_clock, ProcessRealCPUClock);

#[test]
fn test_process_user_cpu_clock() {
    let start = ProcessUserCPUClock::now();
    utils::black_box(utils::computation_task());
    let elapsed = ProcessUserCPUClock::now() - start;
    assert!(elapsed > Duration::from_nanos(0));
}

#[test]
fn test_process_system_cpu_clock() {
    let start = ProcessSystemCPUClock::now();
    let elapsed = ProcessSystemCPUClock::now() - start;
    assert!(elapsed < Duration::from_nanos(10));
}

#[test]
fn test_process_cpu_clock() {
    let start = ProcessCPUClock::now();
    utils::black_box(utils::multithreading_task());
    let elapsed = ProcessCPUClock::now() - start;
    assert!(elapsed.real > Duration::from_nanos(0));
    assert!(elapsed.user > Duration::from_nanos(0));
    assert!(elapsed.user + elapsed.system >= elapsed.real);
}

#[test]
fn test_thread_clock() {
    let start_outter = ThreadClock::now();
    let elapsed_inner = thread::spawn(|| {
        let start_inner = ThreadClock::now();
        utils::black_box(utils::computation_task());
        ThreadClock::now() - start_inner
    })
    .join()
    .unwrap();
    let elapsed_outter = ThreadClock::now() - start_outter;
    assert!(elapsed_inner > Duration::from_nanos(0));
    assert!(elapsed_inner > elapsed_outter);
}
