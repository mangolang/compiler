use mango::towasm::collect::datatype::Value;
use mango::towasm::values::Const;
use mango::towasm::values::Expression;
use mango::towasm::values::Local;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::rc::Rc;

pub struct Assign {
    assignee: Local, // todo
    value: Expression,
}

impl Assign {
    pub fn new(assignee: Local, value: Expression) -> Self {
        // todo:
        //        assert!(assignee.typ.can_assign(value.typ));
        return Assign { assignee, value };
    }
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
