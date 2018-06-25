//use mango::towasm::Wasm;
//use std::fs::File;
//use std::io;
//use std::rc::Rc;

// TODO: REMOVE

//pub struct GivenName {
//    name: String,
//}
//
//impl GivenName {
//    pub fn new(name: String) -> Option<Rc<Self>> {
//        // todo: filter out illegal names (thread_lcoal!)
//        assert!(!name.starts_with("$"));
//        return Some(Rc::new(GivenName { name }));
//    }
//
//    pub fn pure_name(&self) -> String {
//        return self.name.to_owned();
//    }
//}
//
//impl Wasm for GivenName {
//    fn as_wat(&self) -> String {
//        format!("${}", self.name)
//    }
//
//    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
//        unimplemented!()
//    }
//}
