//! Rust packed GxAPI interface
#![allow(dead_code)]

use libloading::{Library, Symbol};
use std::ffi::{c_char, c_void, CStr};
use std::sync::MutexGuard;
use std::sync::PoisonError;

pub type Result<T> = core::result::Result<T, Error>;

// Error

pub struct Error {
    pub inner: Box<ErrorKind>,
}

impl Error {
    pub fn new(inner: ErrorKind) -> Self {
        Error {
            inner: Box::new(inner),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<GxciError> for Error {
    fn from(err: GxciError) -> Self {
        Error {
            inner: Box::new(ErrorKind::GxciError(err)),
        }
    }
}

impl From<PoisonError<MutexGuard<'static, GXInstance>>> for Error {
    fn from(e: PoisonError<MutexGuard<'static, GXInstance>>) -> Self {
        Error::new(ErrorKind::MutexPoisonError(e))
    }
}

// ErrorKind

pub enum ErrorKind {
    GxciError(GxciError),
    MutexPoisonError(PoisonError<MutexGuard<'static, GXInstance>>),
    MutexPoisonOptionError(PoisonError<MutexGuard<'static, Option<GXInstance>>>),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::GxciError(e) => write!(f, "GxciError: {:?}", e),
            ErrorKind::MutexPoisonError(e) => write!(f, "MutexPoisonError: {:?}", e),
            ErrorKind::MutexPoisonOptionError(e) => write!(f, "MutexPoisonOptionError: {:?}", e),
        }
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::GxciError(e) => write!(f, "GxciError: {:?}", e),
            ErrorKind::MutexPoisonError(e) => write!(f, "MutexPoisonError: {:?}", e),
            ErrorKind::MutexPoisonOptionError(e) => write!(f, "MutexPoisonOptionError: {:?}", e),
        }
    }
}

// Old GxciError

pub enum GxciError {
    InitializationError(String),
    FunctionCallError(String),
    LibLoadingError(libloading::Error),
    GalaxyError(i32),
    CommandError(String),
}

impl std::fmt::Display for GxciError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GxciError::InitializationError(e) => write!(f, "InitializationError: {}", e),
            GxciError::FunctionCallError(e) => write!(f, "FunctionCallError: {}", e),
            GxciError::LibLoadingError(e) => write!(f, "LibLoadingError: {}", e),
            GxciError::GalaxyError(e) => write!(f, "GalaxyError: {}", e),
            GxciError::CommandError(e) => write!(f, "CommandError: {}", e),
        }
    }
}

impl std::fmt::Debug for GxciError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GxciError::InitializationError(e) => write!(f, "InitializationError: {}", e),
            GxciError::FunctionCallError(e) => write!(f, "FunctionCallError: {}", e),
            GxciError::LibLoadingError(e) => write!(f, "LibLoadingError: {}", e),
            GxciError::GalaxyError(e) => write!(f, "GalaxyError: {}", e),
            GxciError::CommandError(e) => write!(f, "CommandError: {}", e),
        }
    }
}

use crate::raw::{gx_callback::*, gx_enum::*, gx_handle::*, gx_struct::*};

fn convert_to_gx_status(status_code: i32) -> GX_STATUS_LIST {
    match status_code {
        0 => GX_STATUS_LIST::GX_STATUS_SUCCESS,
        -1 => GX_STATUS_LIST::GX_STATUS_ERROR,
        -2 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_TL,
        -3 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_DEVICE,
        -4 => GX_STATUS_LIST::GX_STATUS_OFFLINE,
        -5 => GX_STATUS_LIST::GX_STATUS_INVALID_PARAMETER,
        -6 => GX_STATUS_LIST::GX_STATUS_INVALID_HANDLE,
        -7 => GX_STATUS_LIST::GX_STATUS_INVALID_CALL,
        -8 => GX_STATUS_LIST::GX_STATUS_INVALID_ACCESS,
        -9 => GX_STATUS_LIST::GX_STATUS_NEED_MORE_BUFFER,
        -10 => GX_STATUS_LIST::GX_STATUS_ERROR_TYPE,
        -11 => GX_STATUS_LIST::GX_STATUS_OUT_OF_RANGE,
        -12 => GX_STATUS_LIST::GX_STATUS_NOT_IMPLEMENTED,
        -13 => GX_STATUS_LIST::GX_STATUS_NOT_INIT_API,
        -14 => GX_STATUS_LIST::GX_STATUS_TIMEOUT,
        _ => GX_STATUS_LIST::GX_STATUS_ERROR, // Default error if unknown status code
    }
}

// Define a custom error type
#[derive(Debug)]
pub enum CameraError {
    LibraryError(libloading::Error),
    OperationError(String),
}

impl From<libloading::Error> for CameraError {
    fn from(err: libloading::Error) -> Self {
        CameraError::LibraryError(err)
    }
}
pub trait GXInterface {
    fn new(library_path: &str) -> Result<Self>
    where
        Self: Sized;

    // Lib
    fn gx_init_lib(&self) -> Result<i32>;
    fn gx_close_lib(&self) -> Result<()>;

    // Device
    fn gx_update_device_list(&self, device_num: *mut u32, timeout: u32) -> Result<i32>;
    fn gx_update_all_device_list(&self, num_devices: *mut u32, timeout: u32) -> Result<i32>;
    fn gx_get_all_device_base_info(
        &self,
        p_device_info: *mut GX_DEVICE_BASE_INFO,
        p_buffer_size: *mut usize,
    ) -> Result<i32>;
    fn gx_open_device_by_index(&self, index: u32, device: *mut GX_DEV_HANDLE) -> Result<i32>;
    fn gx_open_device(
        &self,
        open_param: *const GX_OPEN_PARAM,
        device_handle: *mut GX_DEV_HANDLE,
    ) -> Result<i32>;

    fn gx_close_device(&self, device: GX_DEV_HANDLE) -> Result<i32>;

    // Config

    fn gx_export_config_file(&self, device: GX_DEV_HANDLE, file_path: *const c_char)
        -> Result<i32>;

    fn gx_import_config_file(&self, device: GX_DEV_HANDLE, file_path: *const c_char)
        -> Result<i32>;

    // Command
    fn gx_send_command(&self, device: GX_DEV_HANDLE, feature_id: GX_FEATURE_ID) -> Result<i32>;

    // Image
    fn gx_get_image(
        &self,
        device: GX_DEV_HANDLE,
        p_frame_data: *mut GX_FRAME_DATA,
        timeout: i32,
    ) -> Result<i32>;

    // Flush

    fn gx_flush_queue(&self, device: GX_DEV_HANDLE) -> Result<i32>;
    fn gx_flush_event(&self, device: GX_DEV_HANDLE) -> Result<i32>;

    // Feature Status

    fn gx_get_feature_name(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        name: *mut c_char,
        size: *mut usize,
    ) -> Result<i32>;

    fn gx_is_implemented(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        is_implemented: *mut bool,
    ) -> Result<i32>;

    fn gx_is_readable(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        is_readable: *mut bool,
    ) -> Result<i32>;

    fn gx_is_writable(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        is_writable: *mut bool,
    ) -> Result<i32>;

    // Getter and Setter
    fn gx_get_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value: *mut i64,
    ) -> Result<i32>;
    fn gx_set_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value: i64,
    ) -> Result<i32>;

    fn gx_get_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: *mut f64,
    ) -> Result<i32>;
    fn gx_set_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: f64,
    ) -> Result<i32>;

    fn gx_get_enum_entry_nums(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        entry_nums: *mut u32,
    ) -> Result<i32>;
    fn gx_get_enum_description(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_description: *mut GX_ENUM_DESCRIPTION,
        buffer_size: *mut usize,
    ) -> Result<i32>;
    fn gx_get_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: *mut i64,
    ) -> Result<i32>;
    fn gx_set_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: i64,
    ) -> Result<i32>;

    fn gx_get_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        value: *mut bool,
    ) -> Result<i32>;

    fn gx_set_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        value: bool,
    ) -> Result<i32>;

    fn gx_get_string_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize,
    ) -> Result<i32>;

    fn gx_get_string_max_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize,
    ) -> Result<i32>;

    fn gx_get_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *mut c_char,
        size: *mut usize,
    ) -> Result<i32>;

    fn gx_set_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *const c_char,
    ) -> Result<i32>;

    fn gx_get_buffer_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize,
    ) -> Result<i32>;
    fn gx_get_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *mut u8,
        size: *mut usize,
    ) -> Result<i32>;
    fn gx_set_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *const u8,
        size: usize,
    ) -> Result<i32>;

    fn gx_get_int_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_range: *mut GX_INT_RANGE,
    ) -> Result<i32>;
    fn gx_get_float_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_range: *mut GX_FLOAT_RANGE,
    ) -> Result<i32>;

    fn gx_get_event_num_in_queue(&self, device: GX_DEV_HANDLE, event_num: *mut u32) -> Result<i32>;
    fn gx_get_last_error(
        &self,
        error_code: *mut GX_STATUS_LIST, // !!!这里文档里面没有_List，暂未检验可行性
        err_text: *mut c_char,
        size: *mut usize,
    ) -> Result<i32>;
    fn gx_set_acquisition_buffer_number(
        &self,
        device: GX_DEV_HANDLE,
        buffer_num: u64,
    ) -> Result<i32>;

    // Callback
    fn gx_register_capture_callback(
        &self,
        device: *mut c_void,
        callback: GXCaptureCallBack,
    ) -> Result<i32>;
    fn gx_unregister_capture_callback(&self, device: *mut c_void) -> Result<i32>;
    fn gx_register_device_offline_callback(
        &self,
        device: GX_DEV_HANDLE,
        user_param: *mut std::os::raw::c_void,
        callback_fun: GXDeviceOfflineCallBack,
        callback_handle: *mut GX_EVENT_CALLBACK_HANDLE,
    ) -> Result<i32>;
    fn gx_unregister_device_offline_callback(
        &self,
        device: GX_DEV_HANDLE,
        callback_handle: GX_EVENT_CALLBACK_HANDLE,
    ) -> Result<i32>;
    fn gx_register_feature_callback(
        &self,
        device: GX_DEV_HANDLE,
        user_param: *mut std::os::raw::c_void,
        callback_fun: GXFeatureCallBack,
        feature_id: GX_FEATURE_ID,
        callback_handle: *mut GX_FEATURE_CALLBACK_HANDLE,
    ) -> Result<i32>;
    fn gx_unregister_feature_callback(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        callback_handle: GX_FEATURE_CALLBACK_HANDLE,
    ) -> Result<i32>;
}

pub struct GXInstance {
    pub lib: Library,
}

impl GXInterface for GXInstance {
    /// Create a new GXInterface instance
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     // Other Operations
    ///
    /// }
    /// ```
    fn new(library_path: &str) -> Result<Self> {
        unsafe {
            let lib = Library::new(library_path).map_err(|e| GxciError::LibLoadingError(e))?;
            Ok(GXInstance { lib })
        }
    }

    /// Initialize the library
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     // Other Operations
    ///
    /// }
    /// ```
    fn gx_init_lib(&self) -> Result<i32> {
        unsafe {
            let gx_init_lib: Symbol<unsafe extern "C" fn() -> i32> =
                self.lib.get(b"GXInitLib").map_err(|e| {
                    GxciError::FunctionCallError(format!("Failed to get GXInitLib function: {}", e))
                })?;
            Ok(gx_init_lib())
        }
    }

    /// Close library
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     // Other Operations
    ///
    ///     gx.gx_close_lib().expect("Failed to close library");
    ///
    /// }
    /// ```
    fn gx_close_lib(&self) -> Result<()> {
        unsafe {
            let gx_close_lib: Symbol<unsafe extern "C" fn() -> i32> = self
                .lib
                .get(b"GXCloseLib")
                .map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            gx_close_lib();
            Ok(())
        }
    }

    /// Update the device list
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     let mut device_num = 0;
    ///     gx.gx_update_device_list(&mut device_num, 1000)
    ///         .expect("Failed to update device list");
    ///     println!("Number of devices: {:?}", device_num);
    ///
    ///     // Other Operations
    ///
    ///     gx.gx_close_lib().expect("Failed to close library");
    ///
    /// }
    /// ```
    fn gx_update_device_list(&self, device_num: *mut u32, timeout: u32) -> Result<i32> {
        unsafe {
            let gx_update_device_list: Symbol<
                unsafe extern "C" fn(device_num: *mut u32, timeout: u32) -> i32,
            > = self.lib.get(b"GXUpdateDeviceList").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;
            Ok(gx_update_device_list(device_num, timeout))
        }
    }

    /// Enumerate all available devices on the network and retrieve the number of devices
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     let mut device_num = 0;
    ///     gx.gx_update_all_device_list(&mut device_num, 1000)
    ///         .expect("Failed to update device list");
    ///     println!("Number of devices: {:?}", device_num);
    ///
    ///     // Other Operations
    ///
    ///     gx.gx_close_lib().expect("Failed to close library");
    ///
    /// }
    /// ```
    fn gx_update_all_device_list(&self, num_devices: *mut u32, timeout: u32) -> Result<i32> {
        unsafe {
            let gx_update_all_device_list: Symbol<
                unsafe extern "C" fn(num_devices: *mut u32, timeout: u32) -> i32,
            > = self.lib.get(b"GXUpdateAllDeviceList").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;
            println!("num_devices: {:?}, timeout: {:?}", num_devices, timeout);
            Ok(gx_update_all_device_list(num_devices, timeout))
        }
    }

    /// Get all device base info
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     let mut device_num = 0;
    ///     gx.gx_update_device_list(&mut device_num, 1000)
    ///         .expect("Failed to update device list");
    ///     println!("Number of devices: {:?}", device_num);
    ///
    ///     if device_num > 0 {
    ///         let mut base_info: Vec<GX_DEVICE_BASE_INFO> = (0..device_num).map(|_| {
    ///            GXDeviceBaseInfoBuilder::new().build()
    ///         }).collect();
    ///         let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
    ///         let status = gx
    ///             .gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size)
    ///             .expect("Failed to get all device base info");
    ///         if status == 0 {
    ///             // Assuming 0 is GX_STATUS_SUCCESS
    ///             for info in base_info {
    ///             println!("Device Info: {:?}", info);
    ///             }
    ///         } else {
    ///             println!("Failed to get device base info.");
    ///         }
    ///     } else {
    ///        println!("No Devices found.");
    ///     }
    ///
    ///     // Other Operations
    ///
    ///     gx.gx_close_lib().expect("Failed to close library");
    ///
    /// }
    /// ```

    fn gx_get_all_device_base_info(
        &self,
        p_device_info: *mut GX_DEVICE_BASE_INFO,
        p_buffer_size: *mut usize,
    ) -> Result<i32> {
        unsafe {
            let gx_get_all_device_base_info: Symbol<
                unsafe extern "C" fn(
                    p_device_info: *mut GX_DEVICE_BASE_INFO,
                    p_buffer_size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetAllDeviceBaseInfo").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;
            println!(
                "p_device_info: {:?}, p_buffer_size: {:?}",
                p_device_info, p_buffer_size
            );
            Ok(gx_get_all_device_base_info(p_device_info, p_buffer_size))
        }
    }

    /// Open device by index
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    ///
    fn gx_open_device_by_index(&self, index: u32, device: *mut GX_DEV_HANDLE) -> Result<i32> {
        unsafe {
            let gx_open_device_by_index: Symbol<
                unsafe extern "C" fn(index: u32, device: *mut GX_DEV_HANDLE) -> i32,
            > = self.lib.get(b"GXOpenDeviceByIndex").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;
            Ok(gx_open_device_by_index(index, device))
        }
    }

    /// Open a device by a unique identifier such as SN, IP, MAC, or Index
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_open_device(
        &self,
        open_param: *const GX_OPEN_PARAM,
        device_handle: *mut GX_DEV_HANDLE,
    ) -> Result<i32> {
        unsafe {
            let gx_open_device: Symbol<
                unsafe extern "C" fn(
                    open_param: *const GX_OPEN_PARAM,
                    device_handle: *mut GX_DEV_HANDLE,
                ) -> i32,
            > = self.lib.get(b"GXOpenDevice").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;

            println!("device_handle: {:?}", device_handle);
            Ok(gx_open_device(open_param, device_handle))
        }
    }

    /// Close device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_close_device(&self, device: GX_DEV_HANDLE) -> Result<i32> {
        unsafe {
            let gx_close_device: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE) -> i32> =
                self.lib.get(b"GXCloseDevice").map_err(|e| {
                    GxciError::FunctionCallError(format!(
                        "Failed to get GXUpdateDeviceList function: {}",
                        e
                    ))
                })?;
            Ok(gx_close_device(device))
        }
    }

    /// Export the current parameters of the camera to a configuration file
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_export_config_file(
        &self,
        device: GX_DEV_HANDLE,
        file_path: *const c_char,
    ) -> Result<i32> {
        unsafe {
            let gx_export_config_file: Symbol<
                unsafe extern "C" fn(device: GX_DEV_HANDLE, file_path: *const c_char) -> i32,
            > = self.lib.get(b"GXExportConfigFile").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;
            println!("Exported config file to: {:?}", CStr::from_ptr(file_path));
            Ok(gx_export_config_file(device, file_path))
        }
    }

    /// Import a configuration file to the camera
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_import_config_file(
        &self,
        device: GX_DEV_HANDLE,
        file_path: *const c_char,
    ) -> Result<i32> {
        unsafe {
            let gx_import_config_file: Symbol<
                unsafe extern "C" fn(device: GX_DEV_HANDLE, file_path: *const c_char) -> i32,
            > = self.lib.get(b"GXImportConfigFile").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;
            println!("Imported config file from: {:?}", CStr::from_ptr(file_path));
            Ok(gx_import_config_file(device, file_path))
        }
    }

    /// Send command to device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_send_command(&self, device: GX_DEV_HANDLE, feature_id: GX_FEATURE_ID) -> Result<i32> {
        unsafe {
            let gx_send_command: Symbol<unsafe extern "C" fn(GX_DEV_HANDLE, GX_FEATURE_ID) -> i32> =
                self.lib.get(b"GXSendCommand").map_err(|e| {
                    GxciError::FunctionCallError(format!(
                        "Failed to get GXUpdateDeviceList function: {}",
                        e
                    ))
                })?;

            let status_code = gx_send_command(device, feature_id);
            let status = convert_to_gx_status(status_code);
            match status {
                GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(0),
                _ => Err(Error::new(ErrorKind::GxciError(GxciError::CommandError(
                    format!("GXSendCommand failed with status: {:?}", status),
                )))),
                // Err(
                //     GxciError::CommandError(format!(
                //     "GXSendCommand failed with status: {:?}",
                //     status
                // ))),
            }
        }
    }

    /// Get image from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_get_image(
        &self,
        device: GX_DEV_HANDLE,
        p_frame_data: *mut GX_FRAME_DATA,
        timeout: i32,
    ) -> Result<i32> {
        unsafe {
            let gx_get_image: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    p_frame_data: *mut GX_FRAME_DATA,
                    timeout: i32,
                ) -> i32,
            > = self.lib.get(b"GXGetImage").map_err(|e| {
                GxciError::FunctionCallError(format!(
                    "Failed to get GXUpdateDeviceList function: {}",
                    e
                ))
            })?;
            println!("p_frame_data: {:?}", p_frame_data);
            println!("frame_data: {:?}", *p_frame_data);
            let status_code = gx_get_image(device, p_frame_data, timeout);
            let status = convert_to_gx_status(status_code);
            match status {
                GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(0),
                _ => Err(Error::new(ErrorKind::GxciError(GxciError::CommandError(
                    format!("Failed to get image with status: {:?}", status),
                )))),
            }
        }
    }

    /// Clear the cache images in the image output queue
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_flush_queue(&self, device: GX_DEV_HANDLE) -> Result<i32> {
        unsafe {
            let gx_flush_queue: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE) -> i32> =
                self.lib.get(b"GXFlushQueue").map_err(|e| {
                    GxciError::FunctionCallError(format!(
                        "Failed to get GXCloseLib function: {}",
                        e
                    ))
                })?;

            // 调用 C 函数清空图像输出队列
            let result = gx_flush_queue(device);
            println!("Flushed image queue for device: {:?}", device);
            Ok(result)
        }
    }

    /// Clear the device event queue, such as the end of frame exposure event data queue
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_flush_event(&self, device: GX_DEV_HANDLE) -> Result<i32> {
        unsafe {
            let gx_flush_event: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE) -> i32> =
                self.lib.get(b"GXFlushEvent").map_err(|e| {
                    GxciError::FunctionCallError(format!(
                        "Failed to get GXCloseLib function: {}",
                        e
                    ))
                })?;

            // 调用 C 函数清空设备事件队列
            let result = gx_flush_event(device);
            println!("Flushed event queue for device: {:?}", device);
            Ok(result)
        }
    }

    /// Get the string description corresponding to the feature ID
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_feature_name(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        name: *mut c_char,
        size: *mut usize,
    ) -> Result<i32> {
        unsafe {
            let gx_get_feature_name: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    name: *mut c_char,
                    size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetFeatureName").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            // 调用 C 函数获取功能码对应的字符串描述
            let result = gx_get_feature_name(device, feature_id, name, size);
            println!("name: {:?}, size: {:?}", name, size);
            Ok(result)
        }
    }

    /// Check if a feature is implemented by the camera
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_is_implemented(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        is_implemented: *mut bool,
    ) -> Result<i32> {
        unsafe {
            let gx_is_implemented: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    is_implemented: *mut bool,
                ) -> i32,
            > = self.lib.get(b"GXIsImplemented").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            // 查询是否支持某功能
            println!("is_implemented: {:?}", is_implemented);
            Ok(gx_is_implemented(device, feature_id, is_implemented))
        }
    }

    /// Check if a feature is readable by the camera
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_is_readable(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        is_readable: *mut bool,
    ) -> Result<i32> {
        unsafe {
            let gx_is_readable: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    is_readable: *mut bool,
                ) -> i32,
            > = self.lib.get(b"GXIsReadable").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            // 查询功能码是否可读
            println!("is_readable: {:?}", is_readable);
            Ok(gx_is_readable(device, feature_id, is_readable))
        }
    }

    /// Check if a feature is writable by the camera
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_is_writable(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        is_writable: *mut bool,
    ) -> Result<i32> {
        unsafe {
            let gx_is_writable: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    is_writable: *mut bool,
                ) -> i32,
            > = self.lib.get(b"GXIsWritable").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            // 查询功能码是否可写
            println!("is_writable: {:?}", is_writable);
            Ok(gx_is_writable(device, feature_id, is_writable))
        }
    }

    /// Get int value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_get_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value: *mut i64,
    ) -> Result<i32> {
        unsafe {
            let gx_get_int: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    int_value: *mut i64,
                ) -> i32,
            > = self.lib.get(b"GXGetInt").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("int_value: {:?}", int_value);
            Ok(gx_get_int(device, feature_id, int_value))
        }
    }

    /// Set int value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_set_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value: i64,
    ) -> Result<i32> {
        unsafe {
            let gx_set_int: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    int_value: i64,
                ) -> i32,
            > = self.lib.get(b"GXSetInt").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("int_value: {:?}", int_value);
            Ok(gx_set_int(device, feature_id, int_value))
        }
    }

    /// Get float value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_get_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: *mut f64,
    ) -> Result<i32> {
        unsafe {
            let gx_get_float: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    float_value: *mut f64,
                ) -> i32,
            > = self.lib.get(b"GXGetFloat").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("int_value: {:?}", float_value);
            Ok(gx_get_float(device, feature_id, float_value))
        }
    }

    /// Set float value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_set_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: f64,
    ) -> Result<i32> {
        unsafe {
            let gx_set_float: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    float_value: f64,
                ) -> i32,
            > = self.lib.get(b"GXSetFloat").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("int_value: {:?}", float_value);
            Ok(gx_set_float(device, feature_id, float_value))
        }
    }

    /// Get the number of selectable options for an enumeration type value
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_enum_entry_nums(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        entry_nums: *mut u32,
    ) -> Result<i32> {
        unsafe {
            let gx_get_enum_entry_nums: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    entry_nums: *mut u32,
                ) -> i32,
            > = self.lib.get(b"GXGetEnumEntryNums").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            // 调用 C 函数获取枚举项的可选项个数
            let result = gx_get_enum_entry_nums(device, feature_id, entry_nums);
            println!(
                "Number of enum entries for feature ID {:?}: {:?}",
                feature_id, entry_nums
            );
            Ok(result)
        }
    }

    /// Get the description of enumeration type values: the number of enum items and each item's value and string description
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_enum_description(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_description: *mut GX_ENUM_DESCRIPTION,
        buffer_size: *mut usize,
    ) -> Result<i32> {
        unsafe {
            let gx_get_enum_description: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    enum_description: *mut GX_ENUM_DESCRIPTION,
                    buffer_size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetEnumDescription").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            // 调用 C 函数获取枚举类型值的描述信息
            let result = gx_get_enum_description(device, feature_id, enum_description, buffer_size);
            println!("Buffer size for enum description: {:?}", buffer_size);
            Ok(result)
        }
    }

    /// Get enum value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_get_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: *mut i64,
    ) -> Result<i32> {
        unsafe {
            let gx_get_enum: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    enum_value: *mut i64,
                ) -> i32,
            > = self.lib.get(b"GXGetEnum").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("enum_value: {:?}", enum_value);
            Ok(gx_get_enum(device, feature_id, enum_value))
        }
    }

    /// Set enum value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_set_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: i64,
    ) -> Result<i32> {
        unsafe {
            let gx_set_enum: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    enum_value: i64,
                ) -> i32,
            > = self.lib.get(b"GXSetEnum").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("enum_value: {:?}", enum_value);
            Ok(gx_set_enum(device, feature_id, enum_value))
        }
    }

    /// Get bool value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_get_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        bool_value: *mut bool,
    ) -> Result<i32> {
        unsafe {
            let gx_get_bool: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    bool_value: *mut bool,
                ) -> i32,
            > = self.lib.get(b"GXGetBool").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("bool_value: {:?}", bool_value);
            Ok(gx_get_bool(device, feature_id, bool_value))
        }
    }

    /// Set bool value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_set_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        bool_value: bool,
    ) -> Result<i32> {
        unsafe {
            let gx_set_bool: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    bool_value: bool,
                ) -> i32,
            > = self.lib.get(b"GXSetBool").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("bool_value: {:?}", bool_value);
            Ok(gx_set_bool(device, feature_id, bool_value))
        }
    }

    /// Get the length of the string value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_string_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize, // Using usize to represent size_t
    ) -> Result<i32> {
        unsafe {
            let gx_get_string_length: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetStringLength").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("size: {:?}", size);
            Ok(gx_get_string_length(device, feature_id, size))
        }
    }
    /// Get the maximum length of the string value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_string_max_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize, // Using usize to represent size_t
    ) -> Result<i32> {
        unsafe {
            let gx_get_string_max_length: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetStringMaxLength").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("size: {:?}", size);
            Ok(gx_get_string_max_length(device, feature_id, size))
        }
    }

    /// Get the string value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *mut c_char,
        size: *mut usize,
    ) -> Result<i32> {
        unsafe {
            let gx_get_string: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    content: *mut c_char,
                    size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetString").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("content: {:?}, size: {:?}", content, size);
            Ok(gx_get_string(device, feature_id, content, size))
        }
    }

    /// Set the string value for the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_set_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *const c_char,
    ) -> Result<i32> {
        unsafe {
            let gx_set_string: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    content: *const c_char,
                ) -> i32,
            > = self.lib.get(b"GXSetString").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("content: {:?}", content);
            Ok(gx_set_string(device, feature_id, content))
        }
    }

    /// Get the length of the block data from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    ///
    fn gx_get_buffer_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize,
    ) -> Result<i32> {
        unsafe {
            let gx_get_buffer_length: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetBufferLength").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("size: {:?}", size);
            Ok(gx_get_buffer_length(device, feature_id, size))
        }
    }
    /// Get the block data from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *mut u8,
        size: *mut usize,
    ) -> Result<i32> {
        unsafe {
            let gx_get_buffer: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    buffer: *mut u8,
                    size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetBuffer").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("buffer: {:?}, size: {:?}", buffer, size);
            Ok(gx_get_buffer(device, feature_id, buffer, size))
        }
    }

    /// Set the block data for the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    ///
    fn gx_set_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *const u8,
        size: usize,
    ) -> Result<i32> {
        unsafe {
            let gx_set_buffer: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    buffer: *const u8,
                    size: usize,
                ) -> i32,
            > = self.lib.get(b"GXSetBuffer").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("buffer: {:?}, size: {:?}", buffer, size);
            Ok(gx_set_buffer(device, feature_id, buffer, size))
        }
    }

    /// Get the range of an integer type value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_int_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_range: *mut GX_INT_RANGE,
    ) -> Result<i32> {
        unsafe {
            let gx_get_int_range: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    int_range: *mut GX_INT_RANGE,
                ) -> i32,
            > = self.lib.get(b"GXGetIntRange").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("int_range: {:?}", int_range);
            Ok(gx_get_int_range(device, feature_id, int_range))
        }
    }

    /// Get the range of a float type value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_float_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_range: *mut GX_FLOAT_RANGE,
    ) -> Result<i32> {
        unsafe {
            let gx_get_float_range: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    float_range: *mut GX_FLOAT_RANGE,
                ) -> i32,
            > = self.lib.get(b"GXGetFloatRange").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            println!("float_range: {:?}", float_range);
            Ok(gx_get_float_range(device, feature_id, float_range))
        }
    }

    /// Get the number of events in the remote device event queue
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_event_num_in_queue(&self, device: GX_DEV_HANDLE, event_num: *mut u32) -> Result<i32> {
        unsafe {
            let gx_get_event_num_in_queue: Symbol<
                unsafe extern "C" fn(device: GX_DEV_HANDLE, event_num: *mut u32) -> i32,
            > = self.lib.get(b"GXGetEventNumInQueue").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            // 调用 C 函数获取事件队列中的事件数量
            let result = gx_get_event_num_in_queue(device, event_num);
            println!(
                "Event number in queue for device: {:?}, count: {:?}",
                device, event_num
            );
            Ok(result)
        }
    }

    /// Get the most recent error description.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_get_last_error(
        &self,
        error_code: *mut GX_STATUS_LIST,
        err_text: *mut c_char,
        size: *mut usize,
    ) -> Result<i32> {
        unsafe {
            let gx_get_last_error: Symbol<
                unsafe extern "C" fn(
                    error_code: *mut GX_STATUS_LIST,
                    err_text: *mut c_char,
                    size: *mut usize,
                ) -> i32,
            > = self.lib.get(b"GXGetLastError").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            let result = gx_get_last_error(error_code, err_text, size);
            if !err_text.is_null() && !size.is_null() {
                println!("Error text: {}", CStr::from_ptr(err_text).to_string_lossy());
            }
            Ok(result)
        }
    }

    /// Set the number of acquisition buffers.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_set_acquisition_buffer_number(
        &self,
        device: GX_DEV_HANDLE,
        buffer_num: u64,
    ) -> Result<i32> {
        unsafe {
            let gx_set_acquisition_buffer_number: Symbol<
                unsafe extern "C" fn(device: GX_DEV_HANDLE, buffer_num: u64) -> i32,
            > = self.lib.get(b"GXSetAcqusitionBufferNumber").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            let result = gx_set_acquisition_buffer_number(device, buffer_num);
            println!("Set acquisition buffer number to: {}", buffer_num);
            Ok(result)
        }
    }

    /// Register capture callback
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_register_capture_callback(
        &self,
        device: *mut c_void,
        callback: GXCaptureCallBack,
    ) -> Result<i32> {
        unsafe {
            let gx_register_capture_callback: Symbol<
                unsafe extern "C" fn(
                    device: *mut c_void,
                    user_param: *mut c_void,
                    callback: GXCaptureCallBack,
                ) -> i32,
            > = self.lib.get(b"GXRegisterCaptureCallback").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            let status_code = gx_register_capture_callback(device, std::ptr::null_mut(), callback);
            let status = convert_to_gx_status(status_code);
            match status {
                GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(0),
                _ => Err(Error::new(ErrorKind::GxciError(GxciError::CommandError(
                    format!("Failed to register_callback with status: {:?}", status),
                )))),
            }
        }
    }
    /// Unregister capture callback
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    fn gx_unregister_capture_callback(&self, device: *mut c_void) -> Result<i32> {
        unsafe {
            let gx_unregister_capture_callback: Symbol<
                unsafe extern "C" fn(device: *mut c_void) -> i32,
            > = self.lib.get(b"GXUnregisterCaptureCallback").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;
            let status_code = gx_unregister_capture_callback(device);
            let status = convert_to_gx_status(status_code);
            match status {
                GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(0),
                _ => Err(Error::new(ErrorKind::GxciError(GxciError::CommandError(
                    format!("Failed to unregister_callback with status: {:?}", status),
                )))),
            }
        }
    }
    /// Register a callback function for device offline notification events
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_register_device_offline_callback(
        &self,
        device: GX_DEV_HANDLE,
        user_param: *mut std::os::raw::c_void,
        callback_fun: GXDeviceOfflineCallBack,
        callback_handle: *mut GX_EVENT_CALLBACK_HANDLE,
    ) -> Result<i32> {
        unsafe {
            let gx_register_device_offline_callback: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    user_param: *mut std::os::raw::c_void,
                    callback_fun: GXDeviceOfflineCallBack,
                    callback_handle: *mut GX_EVENT_CALLBACK_HANDLE,
                ) -> i32,
            > = self
                .lib
                .get(b"GXRegisterDeviceOfflineCallback")
                .map_err(|e| {
                    GxciError::FunctionCallError(format!(
                        "Failed to get GXCloseLib function: {}",
                        e
                    ))
                })?;

            let result = gx_register_device_offline_callback(
                device,
                user_param,
                callback_fun,
                callback_handle,
            );
            Ok(result)
        }
    }
    /// Unregister a callback function for device offline notification events
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_unregister_device_offline_callback(
        &self,
        device: GX_DEV_HANDLE,
        callback_handle: GX_EVENT_CALLBACK_HANDLE,
    ) -> Result<i32> {
        unsafe {
            let gx_unregister_device_offline_callback: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    callback_handle: GX_EVENT_CALLBACK_HANDLE,
                ) -> i32,
            > = self
                .lib
                .get(b"GXUnregisterDeviceOfflineCallback")
                .map_err(|e| {
                    GxciError::FunctionCallError(format!(
                        "Failed to get GXCloseLib function: {}",
                        e
                    ))
                })?;

            let result = gx_unregister_device_offline_callback(device, callback_handle);
            Ok(result)
        }
    }

    /// Register a callback function for device feature updates
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_register_feature_callback(
        &self,
        device: GX_DEV_HANDLE,
        user_param: *mut std::os::raw::c_void,
        callback_fun: GXFeatureCallBack,
        feature_id: GX_FEATURE_ID,
        callback_handle: *mut GX_FEATURE_CALLBACK_HANDLE,
    ) -> Result<i32> {
        unsafe {
            let gx_register_feature_callback: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    user_param: *mut std::os::raw::c_void,
                    callback_fun: GXFeatureCallBack,
                    feature_id: GX_FEATURE_ID,
                    callback_handle: *mut GX_FEATURE_CALLBACK_HANDLE,
                ) -> i32,
            > = self.lib.get(b"GXRegisterFeatureCallback").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            let result = gx_register_feature_callback(
                device,
                user_param,
                callback_fun,
                feature_id,
                callback_handle,
            );
            Ok(result)
        }
    }

    /// Unregister a callback function for device feature updates
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    fn gx_unregister_feature_callback(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        callback_handle: GX_FEATURE_CALLBACK_HANDLE,
    ) -> Result<i32> {
        unsafe {
            let gx_unregister_feature_callback: Symbol<
                unsafe extern "C" fn(
                    device: GX_DEV_HANDLE,
                    feature_id: GX_FEATURE_ID,
                    callback_handle: GX_FEATURE_CALLBACK_HANDLE,
                ) -> i32,
            > = self.lib.get(b"GXUnregisterFeatureCallback").map_err(|e| {
                GxciError::FunctionCallError(format!("Failed to get GXCloseLib function: {}", e))
            })?;

            let result = gx_unregister_feature_callback(device, feature_id, callback_handle);
            Ok(result)
        }
    }
}

// 相关定义如下
// pub type GX_DEV_HANDLE = *mut c_void;
// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct GX_FRAME_DATA {
//     pub status: u32,            // Image acquisition status
//     pub frame_id: u64,          // Frame ID
//     pub p_img_buf: *mut c_void, // Pointer to the image buffer
//     pub img_size: i32,          // Size of the image buffer, adjusted to i32 to match C definition
//     pub width: i32,             // Image width, adjusted to i32 to match C definition
//     pub height: i32,            // Image height, adjusted to i32 to match C definition
//     pub pixel_format: i32,      // Pixel format, adjusted to i32 to match C definition
//     pub timestamp: u64,         // Timestamp of the frame
//     pub offset_x: i32,          // X offset of the image
//     pub offset_y: i32,          // Y offset of the image
//     pub reserved: [i32; 1],     // Reserved, array of 1 i32 to match C definition
// }
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_STATUS_LIST {
//     GX_STATUS_SUCCESS = 0,
//     GX_STATUS_ERROR = -1,
//     GX_STATUS_NOT_FOUND_TL = -2,
//     GX_STATUS_NOT_FOUND_DEVICE = -3,
//     GX_STATUS_OFFLINE = -4,
//     GX_STATUS_INVALID_PARAMETER = -5,
//     GX_STATUS_INVALID_HANDLE = -6,
//     GX_STATUS_INVALID_CALL = -7,
//     GX_STATUS_INVALID_ACCESS = -8,
//     GX_STATUS_NEED_MORE_BUFFER = -9,
//     GX_STATUS_ERROR_TYPE = -10,
//     GX_STATUS_OUT_OF_RANGE = -11,
//     GX_STATUS_NOT_IMPLEMENTED = -12,
//     GX_STATUS_NOT_INIT_API = -13,
//     GX_STATUS_TIMEOUT = -14,
// }
