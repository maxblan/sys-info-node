use neon::prelude::*;
use sysinfo::Disks;

pub fn kind(mut cx: FunctionContext) -> JsResult<JsArray> {
    let disks = Disks::new_with_refreshed_list();
    let disk_kinds: Vec<Handle<JsString>> = disks
        .iter()
        .map(|disk| cx.string(disk.kind().to_string()))
        .collect();

    let js_array_length = disk_kinds.len() as u32;
    let disk_kinds_array = JsArray::new(&mut cx, js_array_length.try_into().unwrap());

    for (index, disk_kind) in disk_kinds.iter().enumerate() {
        disk_kinds_array.set(&mut cx, index as u32, disk_kind.clone())?;
    }

    Ok(disk_kinds_array)
}

pub fn name(mut cx: FunctionContext) -> JsResult<JsArray> {
    let disks = Disks::new_with_refreshed_list();
    let disk_names: Vec<Handle<JsString>> = disks
        .iter()
        .map(|disk| cx.string(disk.name().to_str().unwrap()))
        .collect();

    let js_array_length = disk_names.len() as u32;
    let disk_names_array = JsArray::new(&mut cx, js_array_length.try_into().unwrap());

    for (index, disk_name) in disk_names.iter().enumerate() {
        disk_names_array.set(&mut cx, index as u32, disk_name.clone())?;
    }

    Ok(disk_names_array)
}

pub fn file_system(mut cx: FunctionContext) -> JsResult<JsArray> {
    let disks = Disks::new_with_refreshed_list();
    let arr = JsArray::new(&mut cx, disks.len().try_into().unwrap());

    for (idx, disk) in disks.iter().enumerate() {
        let file_system = cx.string(disk.file_system().to_str().unwrap());
        let disk_name = disk.name().to_str().unwrap();
        let obj = JsObject::new(&mut cx);
        obj.set(&mut cx, disk_name, file_system)?;
        arr.set(&mut cx, idx as u32, obj)?;
    }

    Ok(arr)
}

pub fn mount_point(mut cx: FunctionContext) -> JsResult<JsArray> {
    let disks = Disks::new_with_refreshed_list();
    let arr = JsArray::new(&mut cx, disks.len().try_into().unwrap());

    for (idx, disk) in disks.iter().enumerate() {
        let mount_point = cx.string(disk.mount_point().to_str().unwrap());
        let disk_name = disk.name().to_str().unwrap();
        let obj = JsObject::new(&mut cx);
        obj.set(&mut cx, disk_name, mount_point)?;
        arr.set(&mut cx, idx as u32, obj)?;
    }

    Ok(arr)
}

pub fn total_space(mut cx: FunctionContext) -> JsResult<JsArray> {
    let disks = Disks::new_with_refreshed_list();
    let arr = JsArray::new(&mut cx, disks.len().try_into().unwrap());

    for (idx, disk) in disks.iter().enumerate() {
        let total_space = cx.number(disk.total_space() as f64);
        let disk_name = disk.name().to_str().unwrap();
        let obj = JsObject::new(&mut cx);
        obj.set(&mut cx, disk_name, total_space)?;
        arr.set(&mut cx, idx as u32, obj)?;
    }

    Ok(arr)
}

pub fn available_space(mut cx: FunctionContext) -> JsResult<JsArray> {
    let disks = Disks::new_with_refreshed_list();
    let arr = JsArray::new(&mut cx, disks.len().try_into().unwrap());

    for (idx, disk) in disks.iter().enumerate() {
        let available_space = cx.number(disk.available_space() as f64);
        let disk_name = disk.name().to_str().unwrap();
        let obj = JsObject::new(&mut cx);
        obj.set(&mut cx, disk_name, available_space)?;
        arr.set(&mut cx, idx as u32, obj)?;
    }

    Ok(arr)
}

pub fn is_removable(mut cx: FunctionContext) -> JsResult<JsArray> {
    let disks = Disks::new_with_refreshed_list();
    let arr = JsArray::new(&mut cx, disks.len().try_into().unwrap());

    for (idx, disk) in disks.iter().enumerate() {
        let is_removable = cx.boolean(disk.is_removable());
        let disk_name = disk.name().to_str().unwrap();
        let obj = JsObject::new(&mut cx);
        obj.set(&mut cx, disk_name, is_removable)?;
        arr.set(&mut cx, idx as u32, obj)?;
    }

    Ok(arr)
}
