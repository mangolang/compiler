use mango::towasm::values::Expression;
use mango::towasm::values::Local;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;

#[derive(new)]
pub struct Assign {
    assignee: Local, // todo
    value: Expression,
}

impl Wasm for Assign {
    fn as_wat(&self) -> String {
        format!(
            "{}\nset_local {}",
            self.value.as_wat(),
            self.assignee.as_wat()
        )
        //    set_local $fac_result
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
