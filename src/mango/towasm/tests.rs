use mango::towasm::arithmetic::Add;
use mango::towasm::collect::datatype::Value;
use mango::towasm::collect::typ::Wasm;
use mango::towasm::collect::Statement;
use mango::towasm::collect::Type;
use mango::towasm::control::Block;
use mango::towasm::control::Group;
use mango::towasm::control::Loop;
use mango::towasm::numeric::Gt;
use mango::towasm::numeric::Mul;
use mango::towasm::scope::Function;
use mango::towasm::scope::Module;
use mango::towasm::scope::Output;
use mango::towasm::scope::Parameter;
use mango::towasm::util::Name;
use mango::towasm::values::Assign;
use mango::towasm::values::Const;
use mango::towasm::values::DeclareLocal;
use mango::towasm::values::Expression;
use mango::towasm::values::Local;

#[test]
fn test_example_1() {
    let param_n = Parameter::new(Name::new("n".to_owned()).unwrap(), Type::Int32);
    let var_n = param_n.local();
    let loop_condition_decl =
        DeclareLocal::new(Name::new("loop_condition".to_owned()).unwrap(), Type::Bool);
    let loop_condition = loop_condition_decl.local();
    let fac_result_decl =
        DeclareLocal::new(Name::new("fac_result".to_owned()).unwrap(), Type::Int32);
    let fac_result = fac_result_decl.local();
    let loop_name = Name::new("fac_loop".to_owned()).unwrap();
    let wasm = Module::new(vec![Function::new(
        Name::new("fac".to_owned()).unwrap(),
        vec![param_n],
        vec![Output::new(Type::Int32)],
        Group::new(vec![
            // Function body
            Statement::Local(loop_condition_decl),
            Statement::Local(fac_result_decl),
            Statement::Assign(Assign::new(
                fac_result.clone(),
                Expression::Const(Const::new(Type::Int32, Value::Int(1))),
            )),
            //            Statement::Block(Block::new_named("".to_owned(), vec![])),
            Statement::Loop(Loop::new_named(
                loop_name.clone(),
                vec![
                    Statement::Assign(Assign::new(
                        fac_result.clone(),
                        Expression::Mul(Mul::new(
                            Expression::Local(fac_result.get()),
                            Expression::Local(var_n.get()),
                        )),
                    )),
                    //                    Statement::Assign(Assign::new(
                    //                        loop_condition,
                    //                        Expression::Gt(Gt::new(
                    //                            Expression::Local(var_n.get()),
                    //                            Expression::Const(Const::new(Type::Int32, Value::Int(2))),
                    //                        )),
                    //                    )),
                    Statement::Assign(Assign::new(
                        var_n.clone(),
                        Expression::Add(Add::new(
                            Expression::Local(var_n.get()),
                            Expression::Const(Const::new(Type::Int32, Value::Int(-1))),
                        )),
                    )),
                ],
            )),
        ]),
    )]);

    println!("WAT:\n{}\n", wasm.as_wat());
}

//;; calculate faculty (n!)
//(module
//  (func $app (export "fac") (param $n i32) (result i32)
//    (local $fac_result i32) (local $loop_condition i32)
//    i32.const 1
//    set_local $fac_result
//    i32.const 1
//    set_local $fac_result
//    loop $L1
//      get_local $fac_result
//      get_local $n
//      i32.mul
//      set_local $fac_result
//      get_local $n
//      i32.const 2
//      i32.gt_s
//      set_local $loop_condition
//      get_local $n
//      i32.const -1
//      i32.add
//      set_local $n
//      get_local $loop_condition
//      br_if $L1
//    end
//    get_local $fac_result
//  )
//)
