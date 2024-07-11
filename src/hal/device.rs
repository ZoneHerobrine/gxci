use crate::hal::basement::{gxi_check, GXI};
use crate::raw::{gx_const::*, gx_enum::*, gx_handle::*, gx_interface::*, gx_struct::*};
use crate::utils::builder::GXDeviceBaseInfoBuilder;
use crate::utils::facade::convert_to_frame_data;
use crate::utils::facade::*;
use opencv::{
    highgui,
    imgcodecs,
    core,
};
use std::ffi::c_void;
use std::sync::LazyLock;
use std::slice;



//---------------Static Mut V-------------------------------

#[cfg(feature = "solo")]
pub static mut GXI_DEVICE: Option<GX_DEV_HANDLE> = Some(std::ptr::null_mut());
#[cfg(feature = "solo")]
pub static mut GXI_FRAME_DATA: Option<GX_FRAME_DATA> =  Some(GX_FRAME_DATA {
    nStatus: 0,
    pImgBuf: std::ptr::null_mut(),
    nWidth: 0,
    nHeight: 0,
    nPixelFormat: 0,
    nImgSize: 0,
    nFrameID: 0,
    nTimestamp: 0,
    reserved: [0; 1],
});
#[cfg(feature = "solo")]
pub static mut GXI_IMAGE_BUFFER: Option<Vec<u8>> = Some(vec![]);
// pub static mut GXI_IMAGE_BUFFER: LazyLock<Option<Vec<u8>>> = LazyLock::new(|| Some(vec![1; 8]));
// pub static mut GXI_IMAGE_BUFFER: Option<Vec<u8>> = Some(vec![1;8]);




//---------------Static Mut Fn-------------------------------

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



//---------------HAL Functions-------------------------------


pub fn gxi_count_devices(timeout: u32) -> Result<u32, GxciError> {
    let mut device_num = 0;

    unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_update_device_list(&mut device_num, timeout)?;
    }

    Ok(device_num)
}

pub fn gxi_list_devices() -> Result<Vec<GX_DEVICE_BASE_INFO>, GxciError> {
    let mut device_num = 0;
    unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_update_device_list(&mut device_num, 1000)?;
    }

    let mut base_info: Vec<GX_DEVICE_BASE_INFO> = (0..device_num)
        .map(|_| GXDeviceBaseInfoBuilder::new().build())
        .collect();

    let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
    let status = unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size)?
    };

    if status == 0 {
        Ok(base_info)
    } else {
        Err(GxciError::GalaxyError(status))
    }
}

#[cfg(feature = "solo")]
pub fn gxi_open_device() -> Result<(), GxciError> {
    let mut device_num = 0;
    unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_update_device_list(&mut device_num, 1000)?;
    }

    // 在这里调GXI_DEVICE的类型的时候卡了半天，麻了，最后用我最想不到的神秘方式解决了空指针问题
    // 这是个全局变量，静态可变，Option内部空指针初始化，传参的时候要&mut *GXI_DEVICE.as_mut().unwrap()，先可变，再解包，然后取值再可变借用
    let status = unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_open_device_by_index(1, &mut *GXI_DEVICE.as_mut().unwrap())?
    };

    unsafe {
        println!("Device handle: {:?}", GXI_DEVICE);
    }

    if status == 0 {
        println!("Successfully opened device index 1");
        Ok(())
    } else {
        Err(GxciError::GalaxyError(status))
    }
}

#[cfg(feature = "solo")]
pub fn gxi_close_device() -> Result<(), GxciError> {
    let status = unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_close_device(*GXI_DEVICE.as_ref().unwrap())?
    };

    if status == 0 {
        unsafe {
            GXI_DEVICE = Some(std::ptr::null_mut());
            println!("Device handle: {:?}", GXI_DEVICE);
        }
        println!("Successfully closed device");
        Ok(())
    } else {
        Err(GxciError::GalaxyError(status))
    }
}

#[cfg(feature = "solo")]
pub fn gxi_send_command(command: GX_FEATURE_ID) -> Result<(), GxciError> {
    let status = unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_send_command(*GXI_DEVICE.as_mut().unwrap(), command)?
    };

    if status == 0 {
        println!("Successfully sent command");
        Ok(())
    } else {
        Err(GxciError::GalaxyError(status))
    }
}

#[cfg(feature = "solo")]
pub fn gxi_get_image() -> Result<(), GxciError> {

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START);

    unsafe {

        let (frame_data_facade, image_buffer) = fetch_frame_data(&GXI.as_ref().unwrap(), *GXI_DEVICE.as_ref().unwrap()).unwrap();
        let mut frame_data = convert_to_frame_data(&frame_data_facade);

        let status = unsafe {
            GXI.as_ref()
                .ok_or_else(|| {
                    GxciError::InitializationError(
                        "GXI is None. Please check your gxci_init situation.".to_string(),
                    )
                })?
                .gx_get_image(*GXI_DEVICE.as_mut().unwrap(), &mut frame_data, 1000)?
        };

        println!("Frame data: {:?}", frame_data);

        GXI_FRAME_DATA = Some(frame_data);
        GXI_IMAGE_BUFFER = Some(image_buffer);

        // println!("{:?}", GXI_FRAME_DATA.as_ref().unwrap());

        gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP);

        if status == 0 {
            println!("Successfully got image");
            Ok(())
        } else {
            Err(GxciError::GalaxyError(status))
        }

    }
    
    
}

#[cfg(feature = "solo")]
pub fn gxi_save_image_as_png(filename:&str) -> Result<(), GxciError> {
    // 最开始一直报错这个
    // [ERROR:0@3.466] global loadsave.cpp:775 cv::imwrite_ imwrite_('filename.png'): can't write data: unknown exception
    // 最后想起来是和自己之前遇到的同一个问题——指针还还活着，但是image_buffer已经死了，所以存不进去
    // 最后加上GXI_IMAGE_BUFFER = Some(image_buffer);就解决了
    unsafe {
        let frame_data = *GXI_FRAME_DATA.as_ref().unwrap();
        if frame_data.nStatus == 0 {
            let data = slice::from_raw_parts(frame_data.pImgBuf as *const u8, (frame_data.nWidth * frame_data.nHeight) as usize);
            let mat = core::Mat::new_rows_cols_with_data(
                frame_data.nHeight, 
                frame_data.nWidth, 
                data
            ).unwrap();
            if imgcodecs::imwrite(filename, &mat, &opencv::types::VectorOfi32::new()).unwrap() {
                println!("Image saved successfully.");
            } else {
                println!("Failed to save the image.");
            }
        }
    }
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_open_stream() -> Result<(), GxciError> {

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START);

    
    let status = unsafe {
        GXI.as_ref()
            .ok_or_else(|| {
                GxciError::InitializationError(
                    "GXI is None. Please check your gxci_init situation.".to_string(),
                )
            })?
            .gx_stream_on(*GXI_DEVICE.as_mut().unwrap())?
    };

    if status == 0 {
        println!("Successfully opened stream");
        Ok(())
    } else {
        Err(GxciError::GalaxyError(status))
    }
}