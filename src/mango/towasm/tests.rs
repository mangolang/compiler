use mango::towasm::collect::datatype::Value;
use mango::towasm::collect::typ::Wasm;
use mango::towasm::collect::Statement;
use mango::towasm::collect::Type;
use mango::towasm::control::Block;
use mango::towasm::scope::Function;
use mango::towasm::scope::Module;
use mango::towasm::scope::Output;
use mango::towasm::scope::Parameter;
use mango::towasm::util::Name;
use mango::towasm::values::DeclareLocal;
use mango::towasm::values::Local;
use mango::util::strtype::StrType;

#[test]
fn test_example_1() {
    let loop_condition_decl = DeclareLocal::new(
        Name::new("$loop_condition".to_owned()).unwrap(),
        Type::Bool,
    );
    let loop_condition = loop_condition_decl.local();
    let fac_result_decl = DeclareLocal::new(
        Name::new("fac_result".to_owned()).unwrap(),
        Type::Int32,
    );
    let fac_result = fac_result_decl.local();
    let wasm = Module::new(vec![Function::new(
        Name::new("fac".to_owned()).unwrap(),
        vec![Parameter::new(
            Name::new("n".to_owned()).unwrap(),
            Type::Int32,
        )],
        vec![Output::new(Type::Int32)],
        Block::new(vec![
            // Function body
            Statement::Local(loop_condition_decl),
            Statement::Local(fac_result_decl),
        ]),
    )]);

    println!("WAT:\n{}\n", wasm.as_wat());
}

//;; calculate faculty (n!)
//(module
//  (func $app (export "fac") (param $p0 i32) (result i32)
//    (local $fac_result i32) (local $loop_condition i32)
//    i32.const 1
//    set_local $fac_result
//    block $B0
//      get_local $p0
//      i32.const 2
//      i32.lt_s
//      br_if $B0
//      i32.const 1
//      set_local $fac_result
//      loop $L1
//        get_local $fac_result
//        get_local $p0
//        i32.mul
//        set_local $fac_result
//        get_local $p0
//        i32.const 2
//        i32.gt_s
//        set_local $loop_condition
//        get_local $p0
//        i32.const -1
//        i32.add
//        set_local $p0
//        get_local $loop_condition
//        br_if $L1
//      end
//    end
//    get_local $fac_result)
//)
