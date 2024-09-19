use gxci::hal::config::gxi_get_enum_description;
// use gxci::hal::control::analog::*;
use gxci::hal::base::*;
use gxci::hal::device::*;
use gxci::utils::debug::print_device_info;
use gxci::raw::gx_enum::GX_FEATURE_ID;

fn main()->Result<()> {
    gxci_init_default()?;

    let device_num = gxi_count_devices( 1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }
    
    gxi_open_device()?;

    let enum_descriptions = gxi_get_enum_description(GX_FEATURE_ID::GX_ENUM_DEVICE_LINK_THROUGHPUT_LIMIT_MODE)?;
    for enum_description in enum_descriptions {
        println!("{:?}",enum_description);
    }

    gxi_close_device()?;

    gxci_close()?;
    Ok(())
}