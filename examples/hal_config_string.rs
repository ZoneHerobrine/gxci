use gxci::hal::config::gxi_get_float_range;
use gxci::hal::control::device::*;
use gxci::hal::base::*;
use gxci::hal::device::*;
use gxci::utils::debug::print_device_info;
use gxci::raw::gx_enum::GX_FEATURE_ID;

fn main()->Result<()> {
    gxci_init_default()?;
    
    let device_num = gxi_count_devices(1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }
    
    gxi_open_device()?;

    let gain_range = gxi_get_float_range(GX_FEATURE_ID::GX_FLOAT_GAIN)?;
    println!("{:?}",gain_range);
    // 10 - 16

    let name = gxi_get_device_vendor_name()?;
    println!("Vendor name: {}", name);


    gxi_close_device()?;

    gxci_close()?;
    Ok(())
}