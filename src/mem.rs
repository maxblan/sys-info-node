use neon::prelude::*;
use sysinfo::System;

pub fn available_memory(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let system_info = System::new_all();
    let available_memory = system_info.available_memory() as f64;
    Ok(cx.number(available_memory))
}

pub fn free_memory(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let system_info = System::new_all();
    let free_memory = system_info.free_memory() as f64;
    Ok(cx.number(free_memory))
}

pub fn total_memory(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let system_info = System::new_all();
    let total_memory = system_info.total_memory() as f64;
    Ok(cx.number(total_memory))
}

pub fn used_memory(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let system_info = System::new_all();
    let used_memory = system_info.used_memory() as f64;
    Ok(cx.number(used_memory))
}
