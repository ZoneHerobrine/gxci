use gxci::hal::device::*;
use gxci::hal::base::*;
use gxci::utils::debug::print_device_info;

fn main() {
    let dll_path = "C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll"; // 假设这是一个测试用的 DLL 路径
    gxci_init(dll_path);

    let mut device_num = gxi_count_devices( 1000);
    println!("Device number: {}", device_num.unwrap());

    let base_info = gxi_list_devices().unwrap();
    for device in &base_info {
        print_device_info(&device);
    }
    
    gxi_open_device();

    // gxi_open_stream(); // default in 10s,waiting for enhancement

    // gxi_close_stream();

    gxi_open_stream_interval(10);

    gxi_close_device();

    gxci_close();
}