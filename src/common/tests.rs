use std::panic;
use std::thread;

pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> thread::Result<R> {
    // https://stackoverflow.com/a/59211505
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

pub fn assert_panic_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
    let result = catch_unwind_silent(f);
    assert!(result.is_err());
}

// Note that macro must be imported from the crate root
#[macro_export]
#[cfg(test)]
macro_rules! dbg_log {
    ($($arg:tt)*) => {
        print!("[test] ");
        println!($($arg)*);
    }
}

// Note that macro must be imported from the crate root
#[macro_export]
#[cfg(not(test))]
macro_rules! dbg_log {
    ($($arg:tt)*) => {}
}
