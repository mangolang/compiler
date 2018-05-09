use mango::towasm::values::Const;
use mango::towasm::values::Local;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::rc::Rc;
use mango::towasm::collect::datatype::Value;
use mango::towasm::values::Expression;

pub struct Assign {
    assignee: Rc<Local>, // todo
    value: Expression,        // todo
}

impl Assign {
    pub fn new(assignee: Rc<Local>, value: Expression) -> Self {
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
