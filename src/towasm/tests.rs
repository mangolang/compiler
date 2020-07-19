use crate::towasm::arithmetic::Add;
use crate::towasm::collect::datatype::Value;
use crate::towasm::collect::Type;
use crate::towasm::control::BranchIf;
use crate::towasm::control::Label;
use crate::towasm::control::Loop;
use crate::towasm::control::Return;
use crate::towasm::numeric::Gt;
use crate::towasm::numeric::Mul;
use crate::towasm::scope::Function;
use crate::towasm::scope::Module;
use crate::towasm::scope::Output;
use crate::towasm::scope::Parameter;
use crate::towasm::values::Assign;
use crate::towasm::values::Const;
use crate::towasm::values::DeclareLocal;
use crate::util::strtype::Name;
use crate::util::strtype::typ::StrType;

#[test]
#[allow(unused_variables)]
fn test_example_1() {
    let param_n = Parameter::new(Name::new("n").unwrap(), Type::Int32);
    let var_n = param_n.local();
    let module = Module::new(vec![Function::new(
        Name::from_valid("fac"),
        vec![param_n],
        vec![Output::new(Type::Int32)],
        |func_label: Label| {
            let fac_result_decl = DeclareLocal::new(Name::new("fac_result").unwrap(), Type::Int32);
            let fac_result = fac_result_decl.local();
            let loop_condition_decl = DeclareLocal::new(Name::new("loop_condition").unwrap(), Type::Bool);
            let loop_condition = loop_condition_decl.local();
            vec![
                // Function body
                fac_result_decl,
                loop_condition_decl,
                Assign::new(fac_result.clone(), Const::new(Type::Int32, Value::Int(1))),
                Loop::new_named(Name::new("fac_loop").unwrap(), |loop_label: Label| {
                    vec![
                        Assign::new(fac_result.clone(), Mul::new(fac_result.get(), var_n.get())),
                        Assign::new(loop_condition.clone(), Gt::new(var_n.get(), Const::new(Type::Int32, Value::Int(2)))),
                        Assign::new(var_n.clone(), Add::new(var_n.get(), Const::new(Type::Int32, Value::Int(-1)))),
                        BranchIf::new(loop_condition.get(), loop_label),
                    ]
                }),
                Return::new(func_label, fac_result.get()),
            ]
        },
    )]);

    //    println!("WAT:\n{}\n", module.as_wat());
}
