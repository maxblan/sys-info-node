use neon::prelude::*;
use sysinfo::{ System, RefreshKind, CpuRefreshKind };

pub fn cpu_usage(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let (deferred, promise) = cx.promise();

    let mut s = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    s.refresh_cpu();

    let obj: Handle<JsObject> = cx.empty_object();
    for cpu in s.cpus() {
        let usage = cx.number(cpu.cpu_usage());
        obj.set(&mut cx, cpu.name(), usage)?;
    }

    deferred.resolve(&mut cx, obj);

    Ok(promise)
}

pub fn name(mut cx: FunctionContext) -> JsResult<JsArray> {
    let system_info = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );

    let cpu_names: Vec<Handle<JsString>> = system_info
        .cpus()
        .iter()
        .map(|cpu| cx.string(cpu.name()))
        .collect();

    let js_array_length = cpu_names.len() as u32;
    let cpu_names_array = JsArray::new(&mut cx, js_array_length.try_into().unwrap());

    for (index, cpu_name) in cpu_names.iter().enumerate() {
        cpu_names_array.set(&mut cx, index as u32, cpu_name.clone())?;
    }

    Ok(cpu_names_array)
}

pub fn vendor_id(mut cx: FunctionContext) -> JsResult<JsArray> {
    let system_info = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );

    let cpu_vendor_ids: Vec<Handle<JsString>> = system_info
        .cpus()
        .iter()
        .map(|cpu| cx.string(cpu.vendor_id()))
        .collect();

    let js_array_length = cpu_vendor_ids.len() as u32;
    let cpu_vendor_ids_array = JsArray::new(&mut cx, js_array_length.try_into().unwrap());

    for (index, cpu_vendor_id) in cpu_vendor_ids.iter().enumerate() {
        cpu_vendor_ids_array.set(&mut cx, index as u32, cpu_vendor_id.clone())?;
    }

    Ok(cpu_vendor_ids_array)
}

pub fn brand(mut cx: FunctionContext) -> JsResult<JsArray> {
    let system_info = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );

    let cpu_brands: Vec<Handle<JsString>> = system_info
        .cpus()
        .iter()
        .map(|cpu| cx.string(cpu.brand()))
        .collect();

    let js_array_length = cpu_brands.len() as u32;
    let cpu_brands_array = JsArray::new(&mut cx, js_array_length.try_into().unwrap());

    for (index, cpu_brand) in cpu_brands.iter().enumerate() {
        cpu_brands_array.set(&mut cx, index as u32, cpu_brand.clone())?;
    }

    Ok(cpu_brands_array)
}

pub fn frequency(mut cx: FunctionContext) -> JsResult<JsArray> {
    let system_info = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );

    let cpu_frequencies: Vec<Handle<JsNumber>> = system_info
        .cpus()
        .iter()
        .map(|cpu| cx.number(cpu.frequency() as f64))
        .collect();

    let js_array_length = cpu_frequencies.len() as u32;
    let cpu_frequencies_array = JsArray::new(&mut cx, js_array_length.try_into().unwrap());

    for (index, cpu_frequency) in cpu_frequencies.iter().enumerate() {
        cpu_frequencies_array.set(&mut cx, index as u32, cpu_frequency.clone())?;
    }

    Ok(cpu_frequencies_array)
}
