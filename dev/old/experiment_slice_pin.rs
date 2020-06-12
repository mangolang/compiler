#![allow(unused)]
use std::mem;
use std::pin::Pin;
use std::marker::PhantomPinned;

// Inspired by https://stackoverflow.com/a/49916546/

#[derive(Debug)]
struct File {
    text: String,
}

struct Slice {
    // This seems super unsafe for Slices or Files not inside CompileUnit
    file: *const Pin<Box<File>>,
    start: usize,
    end: usize,
}

struct CompileUnit {
    file: Pin<Box<File>>,
    slices: Vec<Slice>,
    _pin: PhantomPinned,
}

fn main() {
    let file = File { text: "hello world!".to_owned() };
    let mut units = vec![
        CompileUnit {
            file: Box::new(file).into(),
            slices: vec![],
            _pin: PhantomPinned,
        }
    ];
    let file1 = &units[0].file as *const Pin<Box<File>>;
    units[0].slices.push(Slice { file: file1, start: 1, end: 3 });
    let file2 = &units[0].file as *const Pin<Box<File>>;
    units[0].slices.push(Slice { file: file2, start: 2, end: 7 });
    dbg!(unsafe { &*units[0].slices[0].file });
    go(units);

    let mut s: Slice;
    {
        let mut f: Pin<Box<File>> = Box::new(File { text: "let's break things".to_owned() }).into();
        s = Slice { file: &f as *const Pin<Box<File>>, start: 3, end: 9, };
    }
    dbg!(unsafe { &*s.file });
}

fn go(units: Vec<CompileUnit>) {}
