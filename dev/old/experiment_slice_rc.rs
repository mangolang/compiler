#![allow(unused)]
use std::rc::Rc;

struct File {
    text: String,
}

struct Slice {
    file: Rc<File>,
    start: usize,
    end: usize,
}

struct CompileUnit {
    file: Rc<File>,
    slices: Vec<Slice>,
}

fn main() {
    let mut units = vec![
        CompileUnit {
            file: Rc::new(File { text: "hello world!".to_owned() }),
            slices: vec![],
        }
    ];
    let file_ref1 = units[0].file.clone();
    let file_ref2 = units[0].file.clone();
    units[0].slices.push(Slice { file: file_ref1, start: 1, end: 3 });
    units[0].slices.push(Slice { file: file_ref2, start: 2, end: 7 });
}
