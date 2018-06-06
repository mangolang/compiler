use mango::towasm::arithmetic::Add;
use mango::towasm::collect::datatype::Value;
use mango::towasm::collect::Type;
use mango::towasm::control::BranchIf;
use mango::towasm::control::Label;
use mango::towasm::control::Loop;
use mango::towasm::control::Return;
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

#[test]
#[allow(unused_variables)]
fn test_example_1() {
    let param_n = Parameter::new(Name::new("n".to_owned()).unwrap(), Type::Int32);
    let var_n = param_n.local();
    let module = Module::new(vec![Function::new(
        Name::new("fac".to_owned()).unwrap(),
        vec![param_n],
        vec![Output::new(Type::Int32)],
        |func_label: Label| {
            let fac_result_decl =
                DeclareLocal::new(Name::new("fac_result".to_owned()).unwrap(), Type::Int32);
            let fac_result = fac_result_decl.local();
            let loop_condition_decl =
                DeclareLocal::new(Name::new("loop_condition".to_owned()).unwrap(), Type::Bool);
            let loop_condition = loop_condition_decl.local();
            vec![
                // Function body
                fac_result_decl,
                loop_condition_decl,
                Assign::new(fac_result.clone(), Const::new(Type::Int32, Value::Int(1))),
                Loop::new_named(
                    Name::new("fac_loop".to_owned()).unwrap(),
                    |loop_label: Label| {
                        vec![
                            Assign::new(
                                fac_result.clone(),
                                Mul::new(fac_result.get(), var_n.get()),
                            ),
                            Assign::new(
                                loop_condition.clone(),
                                Gt::new(var_n.get(), Const::new(Type::Int32, Value::Int(2))),
                            ),
                            Assign::new(
                                var_n.clone(),
                                Add::new(var_n.get(), Const::new(Type::Int32, Value::Int(-1))),
                            ),
                            BranchIf::new(loop_condition.get(), loop_label),
                        ]
                    },
                ),
                Return::new(func_label, fac_result.get()),
            ]
        },
    )]);

    //    println!("WAT:\n{}\n", module.as_wat());
}
