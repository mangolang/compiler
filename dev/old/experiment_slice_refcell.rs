#![allow(unused)]
use core::cell::RefCell;

struct File {
    text: String,
}

struct Slice<'a> {
    file: &'a File,
    start: usize,
    end: usize,
}

struct CompileUnit<'a> {
    file: File,
    slices: RefCell<Vec<Slice<'a>>>,
}

fn main() {
    let mut units = vec![
        CompileUnit {
            file: File { text: "hello world!".to_owned() },
            slices: RefCell::new(vec![]),
        }
    ];
    units[0].slices.borrow_mut().push(Slice { file: &units[0].file, start: 1, end: 3 });
    units[0].slices.borrow_mut().push(Slice { file: &units[0].file, start: 2, end: 7 });
}

