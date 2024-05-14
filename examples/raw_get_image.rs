
use std::ffi::CString;
use std::mem::size_of;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

use opencv::{
    highgui,
    imgcodecs,
    core,
};

use gxci::{
    raw::{
        gx_interface::*, 
        gx_enum::*,
        gx_struct::*,
        gx_handle::*,
    },
    utils::{
        debug::print_device_info,
        builder::GXDeviceBaseInfoBuilder,
        facade::*,
        image_process::*,
    },
};

extern "C" fn frame_callback(p_frame_callback_data: *mut GX_FRAME_CALLBACK_PARAM) {
    // 避免刷屏
    // println!("Frame callback triggered.");
    // println!("Frame status: {:?}", unsafe { (*p_frame_callback_data).status });
    // println!("Frame All: {:?}", unsafe { *p_frame_callback_data });

    unsafe {
        let frame_callback_data = &*p_frame_callback_data;
        if frame_callback_data.status == 0 {
            let data = slice::from_raw_parts(frame_callback_data.pImgBuf as *const u8, (frame_callback_data.nWidth * frame_callback_data.nHeight) as usize);
            let mat = core::Mat::new_rows_cols_with_data(
                frame_callback_data.nHeight, 
                frame_callback_data.nWidth, 
                // core::CV_8UC1, 
                data
            ).unwrap();
            highgui::imshow("Camera Frame", &mat).unwrap();
            if highgui::wait_key(10).unwrap() > 0 {
                highgui::destroy_window("Camera Frame").unwrap();
            }
        }
    }
}

fn main() {
    unsafe {
        // You can change the library path as you need
        let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
        gx.gx_init_lib().expect("Failed to initialize library");

        // Update the device list
        let mut device_num = 0;
        gx.gx_update_device_list(&mut device_num, 1000)
            .expect("Failed to update device list");

        if device_num > 0 {

            let mut base_info: Vec<GX_DEVICE_BASE_INFO> = (0..device_num).map(|_| {
                GXDeviceBaseInfoBuilder::new().build()
            }).collect();
            let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
            let status = gx
                .gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size)
                .expect("Failed to get all device base info");

            if status == 0 {
                // Assuming 0 is GX_STATUS_SUCCESS
                println!(
                    "Device base info retrieved successfully. Number of devices: {}",
                    device_num
                );
                
                for device in &base_info {
                    print_device_info(&device);
                }

                // Attempt to open the first device using its SN
                let first_device_sn = std::str::from_utf8(&base_info[0].szSN).unwrap_or("");
                let mut device_handle: GX_DEV_HANDLE = std::ptr::null_mut();

                let open_status = gx
                    .gx_open_device_by_index(1, &mut device_handle)
                    .expect("Failed to open device with index");

                if open_status == 0 {
                    println!(
                        "Successfully opened device index 1 with SN: {}",
                        first_device_sn.trim_end_matches(char::from(0))
                    );

                    // let reg_result = gx.gx_register_capture_callback(device_handle, frame_callback);
                    // match reg_result {
                    //     Ok(_) => println!("Capture callback registered successfully."),
                    //     Err(e) => eprintln!("Failed to register capture callback: {:?}", e),
                    // }

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)
                        .expect("Failed to send command");

                    // 这种写法在所有权机制下是错误的，因为image_buffer在返回的时候就已经被释放了
                    // let frame_data_facade = fetch_frame_data(&gx, device_handle);
                    // let mut frame_data = convert_to_frame_data(&frame_data_facade.unwrap());
                    
                    // 这种写法是正确的，因为image_buffer被返回到了当前作用域
                    let (frame_data_facade, image_buffer) = fetch_frame_data(&gx, device_handle).unwrap();
                    let mut frame_data = convert_to_frame_data(&frame_data_facade);

                    // or you can use the following code to initialize the GX_FRAME_DATA struct without using the facade
                    // the following config is about Mono8, 2048x1536 camera

                    // let pixel_size = 1; // BayerRg8、Mono8 格式下每像素1字节
                    // let image_size = 2048 * 1536 * pixel_size; // 图像宽*高*每像素字节数
                    // let mut image_buffer = vec![1u8; image_size]; // 分配图像缓冲区
                    // let mut frame_data = GX_FRAME_DATA {
                    //     nStatus: 0,
                    //     pImgBuf: image_buffer.as_mut_ptr() as *mut c_void, // 设置图像数据指针
                    //     nWidth: 2048,
                    //     nHeight: 1536,
                    //     // nPixelFormat: PixelFormatEntry::Mono8 as i32,
                    //     nPixelFormat: 17301505,
                    //     nImgSize: image_size as i32,
                    //     nFrameID: 0,
                    //     nTimestamp: 0,
                    //     reserved: [17301505],
                    // };

                        let result = gx.gx_get_image(device_handle, &mut frame_data, 100);
                        match result {
                            Ok(_) => {
                                println!("Image captured successfully.");

                                // 熬不住了，先睡了，有时间再去封装
                            // if frame_data.nStatus == 0 {
                            //         let mat = mono8_to_mat(&frame_data);
                            //         highgui::imshow("Camera Frame", &mat).unwrap();
                            //     if highgui::wait_key(10).unwrap() > 0 {
                            //         highgui::destroy_window("Camera Frame").unwrap();
                            //     }
                            //     highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE).unwrap();
                                
                            // }
                                if frame_data.nStatus == 0 {
                                    let data = slice::from_raw_parts(frame_data.pImgBuf as *const u8, (frame_data.nWidth * frame_data.nHeight) as usize);
                                    
                                    // 使用正确的函数签名创建Mat对象
                                    let mat = core::Mat::new_rows_cols_with_data(
                                        frame_data.nHeight, 
                                        frame_data.nWidth, 
                                        // core::CV_8UC1, 
                                        data
                                    ).unwrap();
                                    // 保存图像
                        
                                    // 尝试保存图像，检查是否成功
                                    if imgcodecs::imwrite("right.png", &mat, &opencv::types::VectorOfi32::new()).unwrap() {
                                        println!("Image saved successfully.");
                                    } else {
                                        println!("Failed to save the image.");
                                    }
                                    
                                }
                            }
                            Err(e) => eprintln!("Failed to capture image: {:?}", e),
                        }

                        
                    //     highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE).unwrap();
                    // loop {
                    //     sleep(Duration::from_secs(1));
                    //     break;
                    // }

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)
                        .expect("Failed to send command");


                    

                    // let unregeister_result = gx.gx_unregister_capture_callback(device_handle);
                    // match unregeister_result {
                    //     Ok(_) => println!("Capture callback unregistered successfully."),
                    //     Err(e) => eprintln!("Failed to unregister capture callback: {:?}", e),
                    // }

                    // Close the device
                    gx.gx_close_device(device_handle)
                        .expect("Failed to close device");
                    println!("Device closed.")
                } else {
                    println!(
                        "Failed to open device with SN: {}",
                        first_device_sn.trim_end_matches(char::from(0))
                    );
                }
            } else {
                println!("Failed to retrieve device base info, status: {}", status);
            }
        } else {
            println!("No Devices found.");
        }

        gx.gx_close_lib().expect("Failed to close library");
        println!("Library closed.")
    }
}
