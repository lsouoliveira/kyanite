use crate::errors::Error;
use crate::interpreter::Frame;

pub static OPCODE_HANDLERS: &[fn(&mut Frame) -> Result<(), Error>] = &[
    op_load_const,
    op_store_name,
    op_load_name,
    op_call,
    op_pop_top,
];

fn op_load_const(frame: &mut Frame) -> Result<(), Error> {
    let const_index = frame.next_opcode() as usize;
    let const_value = frame.get_const(const_index).ok_or_else(|| {
        Error::RuntimeError(format!("Constant at index {} not found", const_index))
    })?;

    frame.push_stack(const_value.clone());
    frame.increment_pc(1);

    Ok(())
}

fn op_load_name(frame: &mut Frame) -> Result<(), Error> {
    let name_index = frame.next_opcode() as usize;
    let name = frame
        .get_name(name_index)
        .ok_or_else(|| Error::RuntimeError(format!("Name at index {} not defined", name_index)))?;

    let object = frame.resolve(&name)?;

    frame.push_stack(object);
    frame.increment_pc(1);

    Ok(())
}

fn op_store_name(frame: &mut Frame) -> Result<(), Error> {
    let name_index = frame.next_opcode() as usize;
    let name = frame
        .get_name(name_index)
        .ok_or_else(|| Error::RuntimeError(format!("Name at index {} not defined", name_index)))?;

    let value = frame.pop_stack()?;

    frame.register_local(&name, value.clone());

    frame.increment_pc(1);

    Ok(())
}

fn op_call(frame: &mut Frame) -> Result<(), Error> {
    let args_count = frame.next_opcode() as usize;

    let mut args = Vec::with_capacity(args_count);

    for _ in 0..args_count {
        args.push(frame.pop_stack()?);
    }

    let callable = frame.pop_stack()?;
    let callable_type = callable.lock().unwrap().get_type()?;

    callable_type
        .lock()
        .unwrap()
        .call(callable.clone(), &mut args, None)?;

    frame.increment_pc(1);

    Ok(())
}

fn op_pop_top(frame: &mut Frame) -> Result<(), Error> {
    frame.pop_stack()?;
    frame.increment_pc(1);
    Ok(())
}
