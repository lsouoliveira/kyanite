use crate::errors::Error;
use crate::interpreter::Frame;
use crate::objects::base::KyaObject;
use crate::objects::function_object::function_new;

pub static OPCODE_HANDLERS: &[fn(&mut Frame) -> Result<(), Error>] = &[
    op_load_const,
    op_store_name,
    op_load_name,
    op_call,
    op_pop_top,
    op_make_function,
];

fn op_load_const(frame: &mut Frame) -> Result<(), Error> {
    let const_index = frame.next_opcode() as usize;
    let const_value = frame.get_const(const_index).ok_or_else(|| {
        Error::RuntimeError(format!("Constant at index {} not found", const_index))
    })?;

    frame.push_stack(const_value.clone());

    Ok(())
}

fn op_load_name(frame: &mut Frame) -> Result<(), Error> {
    let name_index = frame.next_opcode() as usize;
    let name = frame
        .get_name(name_index)
        .ok_or_else(|| Error::RuntimeError(format!("Name at index {} not defined", name_index)))?;

    let object = frame.resolve(&name)?;

    frame.push_stack(object);

    Ok(())
}

fn op_store_name(frame: &mut Frame) -> Result<(), Error> {
    let name_index = frame.next_opcode() as usize;
    let name = frame
        .get_name(name_index)
        .ok_or_else(|| Error::RuntimeError(format!("Name at index {} not defined", name_index)))?;

    let value = frame.pop_stack()?;

    frame.register_local(&name, value.clone());

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
    let tp_call = callable_type.lock().unwrap().tp_call;

    if let Some(call_fn) = tp_call {
        let result = call_fn(callable, &mut args, None)?;

        frame.push_stack(result);

        Ok(())
    } else {
        Err(Error::RuntimeError(format!(
            "Object '{}' is not callable",
            callable.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

fn op_pop_top(frame: &mut Frame) -> Result<(), Error> {
    frame.pop_stack()?;
    Ok(())
}

fn op_make_function(frame: &mut Frame) -> Result<(), Error> {
    let code_object = frame.pop_stack()?;

    if let KyaObject::CodeObject(c) = &*code_object.lock().unwrap() {
        let code = c.code.clone();

        let function_object = function_new(code.name.clone(), code.clone(), frame.globals.clone());

        frame.register_local(&code.name, function_object.clone());
    } else {
        return Err(Error::RuntimeError(format!(
            "Expected a CodeObject, but got '{}'",
            code_object.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    Ok(())
}
