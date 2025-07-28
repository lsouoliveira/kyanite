use crate::bytecode::{ComparisonOperator, Operator};
use crate::errors::Error;
use crate::interpreter::{eval_frame, Frame};
use crate::objects::base::{
    kya_add, kya_call, kya_compare, kya_set_attr, kya_sub, KyaObject, Type, BASE_TYPE,
};
use crate::objects::class_object::class_new;
use crate::objects::function_object::function_new;
use crate::objects::utils::kya_is_false;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub static OPCODE_HANDLERS: &[fn(&mut Frame) -> Result<(), Error>] = &[
    op_load_const,
    op_store_name,
    op_load_name,
    op_call,
    op_pop_top,
    op_make_function,
    op_load_attr,
    op_compare,
    op_jump_back,
    op_pop_and_jump_if_false,
    op_jump,
    op_make_class,
    op_store_attr,
    op_return,
    op_raise,
    op_bin_op,
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

    let mut args = args.into_iter().rev().collect::<Vec<_>>();

    let callable = frame.pop_stack()?;
    let result = kya_call(callable, &mut args, None)?;

    frame.push_stack(result);

    Ok(())
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

pub fn op_load_attr(frame: &mut Frame) -> Result<(), Error> {
    let instance = frame.pop_stack()?;
    let instance_type = instance.lock().unwrap().get_type()?;
    let tp_get_attr = instance_type.lock().unwrap().tp_get_attr;

    if let Some(get_attr_fn) = tp_get_attr {
        let attr_name_index = frame.next_opcode() as usize;
        let attr_name = frame.get_name(attr_name_index).ok_or_else(|| {
            Error::RuntimeError(format!(
                "Attribute at index {} not defined",
                attr_name_index
            ))
        })?;

        let result = get_attr_fn(instance, attr_name)?;

        frame.push_stack(result);
    } else {
        return Err(Error::RuntimeError(format!(
            "Object '{}' does not support attribute access",
            instance_type.lock().unwrap().name
        )));
    }

    Ok(())
}

pub fn op_compare(frame: &mut Frame) -> Result<(), Error> {
    let right = frame.pop_stack()?;
    let left = frame.pop_stack()?;
    let op = frame.next_opcode();
    let operator = ComparisonOperator::from_u8(op)
        .ok_or_else(|| Error::RuntimeError(format!("Invalid comparison operator: {}", op)))?;

    let result = kya_compare(left, right, operator)?;

    frame.push_stack(result);

    Ok(())
}

pub fn op_jump_back(frame: &mut Frame) -> Result<(), Error> {
    let jump_offset = frame.next_opcode() as usize;
    let current_pc = frame.current_pc();

    frame.set_pc(current_pc - jump_offset);

    Ok(())
}

pub fn op_pop_and_jump_if_false(frame: &mut Frame) -> Result<(), Error> {
    let condition = frame.pop_stack()?;
    let jump = frame.next_opcode() as usize;

    if kya_is_false(condition.clone())? {
        frame.set_pc(jump);
    }

    Ok(())
}

pub fn op_jump(frame: &mut Frame) -> Result<(), Error> {
    let target_pc = frame.next_opcode() as usize;

    frame.set_pc(target_pc);

    Ok(())
}

pub fn op_make_class(frame: &mut Frame) -> Result<(), Error> {
    let code_object = frame.pop_stack()?;

    if let KyaObject::CodeObject(c) = &*code_object.lock().unwrap() {
        let locals = HashMap::new();

        let mut frame_ref = Frame {
            locals: Arc::new(Mutex::new(locals)),
            globals: frame.globals.clone(),
            code: c.code.clone(),
            pc: 0,
            stack: vec![],
            return_value: None,
            error: None,
        };

        let _ = eval_frame(&mut frame_ref);

        let class_type = Type::as_ref(Type {
            ob_type: Some(BASE_TYPE.clone()),
            name: c.code.name.clone(),
            dict: frame_ref.locals.clone(),
            ..Default::default()
        });

        frame.register_local(&c.code.name, class_new(class_type));
    } else {
        return Err(Error::RuntimeError(format!(
            "Expected a CodeObject, but got '{}'",
            code_object.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    Ok(())
}

pub fn op_store_attr(frame: &mut Frame) -> Result<(), Error> {
    let instance = frame.pop_stack()?;
    let value = frame.pop_stack()?;
    let name_index = frame.next_opcode() as usize;
    let name = frame
        .get_name(name_index)
        .ok_or_else(|| Error::RuntimeError(format!("Name at index {} not defined", name_index)))?;

    kya_set_attr(instance.clone(), name.clone(), value.clone())?;

    frame.push_stack(value);

    Ok(())
}

pub fn op_return(frame: &mut Frame) -> Result<(), Error> {
    let return_value = frame.pop_stack()?;

    frame.set_return_value(Some(return_value));

    Ok(())
}

pub fn op_raise(frame: &mut Frame) -> Result<(), Error> {
    let exception = frame.pop_stack()?;

    if !matches!(*exception.lock().unwrap(), KyaObject::ExceptionObject(_)) {
        return Err(Error::RuntimeError(format!(
            "Expected an ExceptionObject, but got '{}'",
            exception.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    frame.set_error(Some(exception.clone()));

    Ok(())
}

pub fn op_bin_op(frame: &mut Frame) -> Result<(), Error> {
    let right = frame.pop_stack()?;
    let left = frame.pop_stack()?;
    let op = frame.next_opcode();
    let operator = Operator::from_u8(op)
        .ok_or_else(|| Error::RuntimeError(format!("Invalid binary operator: {}", op)))?;

    let result = match operator {
        Operator::Plus => kya_add(left, right)?,
        Operator::Minus => kya_sub(left, right)?,
    };

    frame.push_stack(result);

    Ok(())
}
