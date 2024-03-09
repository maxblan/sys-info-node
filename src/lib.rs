use neon::prelude::*;

mod cpu;
mod load_avg;
mod disk;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("cpuUsage", cpu::cpu_usage)?;
    cx.export_function("cpuName", cpu::name)?;
    cx.export_function("cpuVendorId", cpu::vendor_id)?;
    cx.export_function("cpuFrequency", cpu::frequency)?;
    cx.export_function("cpuFrequency", cpu::brand)?;

    cx.export_function("loadAvg", load_avg::load_avg)?;

    cx.export_function("diskKind", disk::kind)?;
    cx.export_function("diskName", disk::name)?;
    cx.export_function("diskFileSystem", disk::file_system)?;
    cx.export_function("diskMountPoint", disk::mount_point)?;
    cx.export_function("diskAvailableSpace", disk::available_space)?;
    cx.export_function("diskTotalSpace", disk::total_space)?;
    cx.export_function("diskIsRemovable", disk::is_removable)?;

    Ok(())
}
