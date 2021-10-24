extern crate inkwell;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(
        &[i64_type.into(), i64_type.into(), i64_type().into()],
        false,
    );

    // Module
    let module = context.create_module("main");
    let builder = context.create_builder();

    // Function
    let function = module.add_function("sum", fn_type, None);

    // Block
    let basic_block = context.append_basic_block(function, "entry");

    // Instruction(Builder)
    builder.position_at_end(basic_block);
    let x = function.get_nth_param(0).unwrap().into_int_value();
    let y = function.get_nth_param(0).unwrap().into_int_value();
    let z = function.get_nth_param(0).unwrap().into_int_value();
    let sum = builder.build_int_add(x, y, "sum");
    let sum = builder.build_int_add(z, sum, "sum");
    builder.build_return(Some(&sum));

    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    unsafe {
        let x = 1u64;
        let y = 2u64;
        let z = 3u64;
        let r = execution_engine
            .get_function::<unsafe extern "C" fn(u64, u64, u64) -> u64>("sum")?
            .call(x, y, z);
        println!("{:?}", r);
        Ok(())
    }
}
