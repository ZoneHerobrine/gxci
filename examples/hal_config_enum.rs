use gxci::hal::config::gxi_get_enum_description;
// use gxci::hal::control::analog::*;
use gxci::hal::base::*;
use gxci::hal::device::*;
use gxci::utils::debug::print_device_info;
use gxci::raw::gx_enum::GX_FEATURE_ID;
use gxci::raw::gx_struct::GX_ENUM_DESCRIPTION;

fn main()->Result<()> {
    gxci_init_default()?;

    let device_num = gxi_count_devices( 1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }
    
    gxi_open_device()?;


    fn extract_sz_symbolic(data: GX_ENUM_DESCRIPTION) -> String {
        let symbolic_bytes: Vec<u8> = data.szSymbolic
        .iter()
        .take_while(|&&x| x != 0)
        .map(|&x| x as u8) // Convert i8 to u8
        .collect();

        String::from_utf8_lossy(&symbolic_bytes).to_string()
    }

    let enum_descriptions = gxi_get_enum_description(GX_FEATURE_ID::GX_ENUM_BALANCE_WHITE_AUTO)?;
    for enum_description in enum_descriptions {
        println!("{:?}",enum_description);
        println!("{:?}",extract_sz_symbolic(enum_description));
    }

    gxi_close_device()?;

    gxci_close()?;
    Ok(())
}