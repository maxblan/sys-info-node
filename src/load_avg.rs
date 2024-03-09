use neon::prelude::*;
use sysinfo::System;

pub fn load_avg(mut cx: FunctionContext) -> JsResult<JsObject> {
    let load_avg = System::load_average();

    let one = cx.number(load_avg.one);
    let five = cx.number(load_avg.five);
    let fifteen = cx.number(load_avg.fifteen);

    let obj: Handle<JsObject> = cx.empty_object();
    obj.set(&mut cx, "one", one)?;
    obj.set(&mut cx, "five", five)?;
    obj.set(&mut cx, "fifteen", fifteen)?;

    Ok(obj)
}
