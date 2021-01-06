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
        // Find the name of nested mock function
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // Find and cut the rest of the path
        let fun_name = match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        };
        // Do actual printing
        print!("{}: ", &fun_name);
        println!($($arg)*);
    }
}

// Note that macro must be imported from the crate root
#[macro_export]
#[cfg(not(test))]
macro_rules! dbg_log {
    ($($arg:tt)*) => {}
}
