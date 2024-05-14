#![allow(dead_code)]
use std::ops::BitOr;


// 实现 BitOr for GX_FEATURE_TYPE

impl BitOr for GX_FEATURE_TYPE {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<GX_FEATURE_LEVEL> for GX_FEATURE_TYPE {
    type Output = u32;

    fn bitor(self, rhs: GX_FEATURE_LEVEL) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<GX_FEATURE_TYPE> for u32 {
    type Output = u32;

    fn bitor(self, rhs: GX_FEATURE_TYPE) -> Self::Output {
        self | rhs as u32
    }
}

impl BitOr<GX_FEATURE_LEVEL> for u32 {
    type Output = u32;

    fn bitor(self, rhs: GX_FEATURE_LEVEL) -> Self::Output {
        self | rhs as u32
    }
}




//------------------------------------------------------------------------------
//  Frame Status Definition
//------------------------------------------------------------------------------
// 这种写法的好处暂时搁置
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GXFrameStatusList {
//     GX_FRAME_STATUS_SUCCESS = 0,           // Normal frame
//     GX_FRAME_STATUS_INCOMPLETE = -1,       // Incomplete frame
// }
// pub type GX_FRAME_STATUS = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FRAME_STATUS {
    GX_FRAME_STATUS_SUCCESS = 0,           // Normal frame
    GX_FRAME_STATUS_INCOMPLETE = -1,       // Incomplete frame
}


//------------------------------------------------------------------------------
//  Device Type Definition
//------------------------------------------------------------------------------
// 历史原因保留下面的写法
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS {
    GX_DEVICE_CLASS_UNKNOWN = 0,           // Unknown device type
    GX_DEVICE_CLASS_USB2 = 1,              // USB2.0 Vision device
    GX_DEVICE_CLASS_GEV = 2,               // Gige Vision device
    GX_DEVICE_CLASS_U3V = 3,               // USB3 Vision device
}
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_DEVICE_CLASS {
//     Unknown = 0,
//     Usb2 = 1,
//     Gev = 2,
//     U3v = 3,
// }

//------------------------------------------------------------------------------
//  Feature Type Definition
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_TYPE {
    GX_FEATURE_INT = 0x10000000,           // Integer type
    GX_FEATURE_FLOAT = 0x20000000,         // Floating point type
    GX_FEATURE_ENUM = 0x30000000,          // Enum type
    GX_FEATURE_BOOL = 0x40000000,          // Boolean type
    GX_FEATURE_STRING = 0x50000000,        // String type
    GX_FEATURE_BUFFER = 0x60000000,        // Block data type
    GX_FEATURE_COMMAND = 0x70000000,       // Command type
}
// pub enum GX_FEATURE_TYPE {
//     GX_FEATURE_INT = 268435456,           // Integer type
//     GX_FEATURE_FLOAT = 536870912,         // Floating point type
//     GX_FEATURE_ENUM = 805306368,          // Enum type
//     GX_FEATURE_BOOL = 1073741824,          // Boolean type
//     GX_FEATURE_STRING = 1342177280,        // String type
//     GX_FEATURE_BUFFER = 1610612736,        // Block data type
//     GX_FEATURE_COMMAND = 1879048192,       // Command type
// }

//------------------------------------------------------------------------------
//  Feature Level Definition
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_LEVEL {
    GX_FEATURE_LEVEL_REMOTE_DEV         = 0x00000000,  //< Remote device layer
    GX_FEATURE_LEVEL_TL                 = 0x01000000,  //< TL layer
    GX_FEATURE_LEVEL_IF                 = 0x02000000,  //< Interface layer    
    GX_FEATURE_LEVEL_DEV                = 0x03000000,  //< Device layer
    GX_FEATURE_LEVEL_DS                 = 0x04000000,  //< DataStream layer
}
// pub enum GX_FEATURE_LEVEL {
//     GX_FEATURE_LEVEL_REMOTE_DEV         = 0,  //< Remote device layer
//     GX_FEATURE_LEVEL_TL                 = 16777216,  //< TL layer
//     GX_FEATURE_LEVEL_IF                 = 33554432,  //< Interface layer    
//     GX_FEATURE_LEVEL_DEV                = 50331648,  //< Device layer
//     GX_FEATURE_LEVEL_DS                 = 67108864,  //< DataStream layer
// }

//------------------------------------------------------------------------------
//  Access Mode of Device
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_MODE
{
    GX_ACCESS_READONLY                  = 2,           //< Open the device in read-only mode
    GX_ACCESS_CONTROL                   = 3,           //< Open the device in controlled mode
    GX_ACCESS_EXCLUSIVE                 = 4,           //< Open the device in exclusive mode
}

// 历史原因保留
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_ACCESS_STATUS_CMD {
//     Unknown = 0,
//     ReadWrite = 1,
//     ReadOnly = 2,
//     NoAccess = 3,
// }


// 自己写的
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_ID{

    //////////////////////////////////////////////////////////////////////////
    /// Remote device layer(Remote Device Feature)
    //////////////////////////////////////////////////////////////////////////

    //---------------DeviceInfomation Section--------------------------
    GX_STRING_DEVICE_VENDOR_NAME               = (0   | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Name of the manufacturer of the device.
    GX_STRING_DEVICE_MODEL_NAME                = (1   | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Model of the device.
    GX_STRING_DEVICE_FIRMWARE_VERSION          = (2   | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Version of the firmware in the device.
    GX_STRING_DEVICE_VERSION                   = (3   | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Version of the device.
    GX_STRING_DEVICE_SERIAL_NUMBER             = (4   | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Device serial number.
    GX_STRING_FACTORY_SETTING_VERSION          = (6   | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Factory parameter version
    GX_STRING_DEVICE_USERID                    = (7   | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< User-programmable device identifier.
    GX_INT_DEVICE_LINK_SELECTOR                = (8   | GX_FEATURE_TYPE::GX_FEATURE_INT    as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Selects which Link of the device to control.
    GX_ENUM_DEVICE_LINK_THROUGHPUT_LIMIT_MODE  = (9   | GX_FEATURE_TYPE::GX_FEATURE_ENUM   as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Controls if the DeviceLinkThroughputLimit is active.
    GX_INT_DEVICE_LINK_THROUGHPUT_LIMIT        = (10  | GX_FEATURE_TYPE::GX_FEATURE_INT    as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Limits the maximum bandwidth of the data that will be streamed out by the device on the selected Link.
    GX_INT_DEVICE_LINK_CURRENT_THROUGHPUT      = (11  | GX_FEATURE_TYPE::GX_FEATURE_INT    as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< The bandwidth of current device acquisition
    GX_COMMAND_DEVICE_RESET                    = (12  | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< reset device
    GX_INT_TIMESTAMP_TICK_FREQUENCY            = (13  | GX_FEATURE_TYPE::GX_FEATURE_INT     as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Time stamp clock frequency
    GX_COMMAND_TIMESTAMP_LATCH                 = (14  | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Timestamp latch 
    GX_COMMAND_TIMESTAMP_RESET                 = (15  | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< reset Timestamp
    GX_COMMAND_TIMESTAMP_LATCH_RESET           = (16  | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< reset Timestamp latch
    GX_INT_TIMESTAMP_LATCH_VALUE               = (17  | GX_FEATURE_TYPE::GX_FEATURE_INT     as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Timestamp Latch value
    GX_STRING_DEVICE_PHY_VERSION               = (18  | GX_FEATURE_TYPE::GX_FEATURE_STRING  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Device network chip version
    GX_ENUM_DEVICE_TEMPERATURE_SELECTOR        = (19  | GX_FEATURE_TYPE::GX_FEATURE_ENUM    as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Device temperature selection
    GX_FLOAT_DEVICE_TEMPERATURE                = (20  | GX_FEATURE_TYPE::GX_FEATURE_FLOAT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Device temperature
    GX_STRING_DEVICE_ISP_FIRMWARE_VERSION      = (21  | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Device ISP firmware version
    GX_ENUM_LOWPOWER_MODE                      = (22  | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Low power consumption mode,refer to GX_LOWPOWER_MODE_ENTRY
    GX_ENUM_CLOSE_CCD                          = (23  | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Close CCD, refer to GX_CLOSE_CCD_ENTRY

    //---------------ImageFormat Section--------------------------------
    GX_INT_SENSOR_WIDTH               = (1000 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Effective width of the sensor in pixels.
    GX_INT_SENSOR_HEIGHT              = (1001 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Effective height of the sensor in pixels.
    GX_INT_WIDTH_MAX                  = (1002 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Maximum width of the image (in pixels).
    GX_INT_HEIGHT_MAX                 = (1003 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Maximum height of the image (in pixels).
    GX_INT_OFFSET_X                   = (1004 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Horizontal offset from the origin to the region of interest (in pixels).
    GX_INT_OFFSET_Y                   = (1005 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Vertical offset from the origin to the region of interest (in pixels).
    GX_INT_WIDTH                      = (1006 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Width of the image provided by the device (in pixels).
    GX_INT_HEIGHT                     = (1007 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Height of the image provided by the device (in pixels).
    GX_INT_BINNING_HORIZONTAL         = (1008 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Number of horizontal photo-sensitive cells to combine together.
    GX_INT_BINNING_VERTICAL           = (1009 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Number of vertical photo-sensitive cells to combine together.
    GX_INT_DECIMATION_HORIZONTAL      = (1010 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,     //< Horizontal sub-sampling of the image.
    GX_INT_DECIMATION_VERTICAL        = (1011 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,      //< Vertical sub-sampling of the image.
    GX_ENUM_PIXEL_SIZE                = (1012 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Total size in bits of a pixel of the image.
    GX_ENUM_PIXEL_COLOR_FILTER        = (1013 | GX_FEATURE_TYPE::GX_FEATURE_ENUM  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Type of color filter that is applied to the image.
    GX_ENUM_PIXEL_FORMAT              = (1014 | GX_FEATURE_TYPE::GX_FEATURE_ENUM  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Format of the pixels provided by the device.
    GX_BOOL_REVERSE_X                 = (1015 | GX_FEATURE_TYPE::GX_FEATURE_BOOL  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Flip horizontally the image sent by the device.
    GX_BOOL_REVERSE_Y                 = (1016 | GX_FEATURE_TYPE::GX_FEATURE_BOOL  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Flip vertically the image sent by the device.
    GX_ENUM_TEST_PATTERN              = (1017 | GX_FEATURE_TYPE::GX_FEATURE_ENUM  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Selects the type of test pattern that is generated by the device as image source.
    GX_ENUM_TEST_PATTERN_GENERATOR_SELECTOR = (1018 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< Selects which test pattern generator is controlled by the TestPattern feature.
    GX_ENUM_REGION_SEND_MODE          = (1019 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< ROI output mode, see also GX_REGION_SEND_MODE_ENTRY
    GX_ENUM_REGION_MODE               = (1020 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< zone switch, see also GX_REGION_MODE_ENTRY
    GX_ENUM_RREGION_SELECTOR          = (1021 | GX_FEATURE_TYPE::GX_FEATURE_ENUM  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Selects the Region of interest to control.
    GX_INT_CENTER_WIDTH               = (1022 |GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,      //< width of window
    GX_INT_CENTER_HEIGHT              = (1023 |GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,      //< height of window
    GX_ENUM_BINNING_HORIZONTAL_MODE   = (1024 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Binning Horizontal mode, see also GX_BINNING_HORIZONTAL_MODE_ENTRY
    GX_ENUM_BINNING_VERTICAL_MODE     = (1025 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Binning vertical mode, see also GX_BINNING_VERTICAL_MODE_ENTRY
    GX_ENUM_SENSOR_SHUTTER_MODE       = (1026 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< Sensor shutter mode, refer to GX_SENSOR_SHUTTER_MODE_ENTRY
    GX_INT_DECIMATION_LINENUMBER      = (1027 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32  | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,    //< decimation line number
    GX_INT_SENSOR_DECIMATION_HORIZONTAL = (1028 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32  | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,  //< Sensor horizontal decimation
    GX_INT_SENSOR_DECIMATION_VERTICAL   = (1029 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32  | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,  //< Sensor vertical decimation
    GX_ENUM_SENSOR_SELECTOR             = (1030 | GX_FEATURE_TYPE::GX_FEATURE_ENUM  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,  //< selector current sonsor, refer to GX_SENSOR_SELECTOR_ENTRY
    GX_INT_CURRENT_SENSOR_WIDTH         = (1031 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< current sonsor width
    GX_INT_CURRENT_SENSOR_HEIGHT        = (1032 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< current sonsor height
    GX_INT_CURRENT_SENSOR_OFFSETX       = (1033 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< current sonsor offset X
    GX_INT_CURRENT_SENSOR_OFFSETY       = (1034 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< current sonsor offset Y
    GX_INT_CURRENT_SENSOR_WIDTHMAX      = (1035 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< current sonsor width max
    GX_INT_CURRENT_SENSOR_HEIGHTMAX     = (1036 | GX_FEATURE_TYPE::GX_FEATURE_INT  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,   //< current sonsor height max
    GX_ENUM_SENSOR_BIT_DEPTH            = (1037 | GX_FEATURE_TYPE::GX_FEATURE_ENUM  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,  //< Sensor Bit Depth, refer to GX_SENSOR_BIT_DEPTH_ENTRY
    GX_BOOL_WATERMARK_ENABLE            = (1038 | GX_FEATURE_TYPE::GX_FEATURE_BOOL  as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,  //< Watermark

    //---------------TransportLayer Section-------------------------------
    GX_INT_PAYLOAD_SIZE                              =( 2000 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< Provides the number of bytes transferred for each image or chunk on the stream channel. 
    GX_BOOL_GEV_CURRENT_IPCONFIGURATION_LLA          =( 2001 | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Controls whether the Link Local Address IP configuration scheme is activated on the given logical link.
    GX_BOOL_GEV_CURRENT_IPCONFIGURATION_DHCP         =( 2002 | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Controls whether the DHCP IP configuration scheme is activated on the given logical link.
    GX_BOOL_GEV_CURRENT_IPCONFIGURATION_PERSISTENTIP =( 2003 | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Controls whether the PersistentIP configuration scheme is activated on the given logical link.
    GX_INT_ESTIMATED_BANDWIDTH                       =( 2004 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< EstimatedBandwidth, Unit: Bps(Bytes per second)
    GX_INT_GEV_HEARTBEAT_TIMEOUT                     =( 2005 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< Controls the current heartbeat timeout in milliseconds.
    GX_INT_GEV_PACKETSIZE                            =( 2006 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< Specifies the stream packet size, in bytes, to send on the selected channel for a GVSP transmitter or specifies the maximum packet size supported by a GVSP receiver.
    GX_INT_GEV_PACKETDELAY                           =( 2007 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< Controls the delay (in timestamp counter unit) to insert between each packet for this stream channel.
    GX_INT_GEV_LINK_SPEED                            =( 2008 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< It indicates the connection speed in Mbps for the selected network interface.

    //---------------AcquisitionTrigger Section---------------------------
    GX_ENUM_ACQUISITION_MODE          =( 3000 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Sets the acquisition mode of the device.
    GX_COMMAND_ACQUISITION_START      =( 3001 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Starts the Acquisition of the device.
    GX_COMMAND_ACQUISITION_STOP       =( 3002 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Stops the Acquisition of the device at the end of the current Frame.
    GX_INT_ACQUISITION_SPEED_LEVEL    =( 3003 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,     //< Setting the speed level of acquiring image.
    GX_INT_ACQUISITION_FRAME_COUNT    =( 3004 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,     //< Number of frames to acquire in MultiFrame Acquisition mode.
    GX_ENUM_TRIGGER_MODE              =( 3005 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Controls if the selected trigger is active.
    GX_COMMAND_TRIGGER_SOFTWARE       =( 3006 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Generates an internal trigger.
    GX_ENUM_TRIGGER_ACTIVATION        =( 3007 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Specifies the activation mode of the trigger.
    GX_ENUM_TRIGGER_SWITCH            =( 3008 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Control external trigger signal is valid, see also GX_TRIGGER_SWITCH_ENTRY
    GX_FLOAT_EXPOSURE_TIME            =( 3009 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Sets the Exposure time when ExposureMode is Timed and ExposureAuto is Off.
    GX_ENUM_EXPOSURE_AUTO             =( 3010 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Sets the automatic exposure mode when ExposureMode is Timed.
    GX_FLOAT_TRIGGER_FILTER_RAISING   =( 3011 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Raising edge signal pulse width is smaller than this value is invalid.
    GX_FLOAT_TRIGGER_FILTER_FALLING   =( 3012 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Falling edge signal pulse width is smaller than this value is invalid.
    GX_ENUM_TRIGGER_SOURCE            =( 3013 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Specifies the internal signal or physical input Line to use as the trigger source.
    GX_ENUM_EXPOSURE_MODE             =( 3014 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Sets the operation mode of the Exposure (or shutter).
    GX_ENUM_TRIGGER_SELECTOR          =( 3015 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Selects the type of trigger to configure.
    GX_FLOAT_TRIGGER_DELAY            =( 3016 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Specifies the delay in microseconds (us) to apply after the trigger reception before activating it.
    GX_ENUM_TRANSFER_CONTROL_MODE     =( 3017 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Selects the control method for the transfers.
    GX_ENUM_TRANSFER_OPERATION_MODE   =( 3018 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Selects the operation mode of the transfer.
    GX_COMMAND_TRANSFER_START         =( 3019 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Starts the streaming of data blocks out of the device.
    GX_INT_TRANSFER_BLOCK_COUNT       =( 3020 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,     //< frame number of transmission. when set GX_ENUM_TRANSFER_OPERATION_MODE as GX_ENUM_TRANSFER_OPERATION_MODE_MULTIBLOCK, this function is actived
    GX_BOOL_FRAMESTORE_COVER_ACTIVE   =( 3021 | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< FrameBufferOverwriteActive
    GX_ENUM_ACQUISITION_FRAME_RATE_MODE      =( 3022 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Controls if the acquisitionFrameRate is active, see also GX_ACQUISITION_FRAME_RATE_MODE_ENTRY
    GX_FLOAT_ACQUISITION_FRAME_RATE          =( 3023 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Controls the acquisition rate (in Hertz) at which the frames are captured.
    GX_FLOAT_CURRENT_ACQUISITION_FRAME_RATE  =( 3024 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Indicates the maximum allowed frame acquisition rate.
    GX_ENUM_FIXED_PATTERN_NOISE_CORRECT_MODE =( 3025 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Controls if the FixedPatternNoise is active, see also GX_FIXED_PATTERN_NOISE_CORRECT_MODE  
    GX_INT_ACQUISITION_BURST_FRAME_COUNT     =( 3030 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,     //< frame number of transmission.
    GX_ENUM_ACQUISITION_STATUS_SELECTOR      =( 3031 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Acquisition status selection, see also GX_ACQUISITION_STATUS_SELECTOR_ENTRY
    GX_BOOL_ACQUISITION_STATUS               =( 3032 | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Acquisition status
    GX_FLOAT_EXPOSURE_DELAY                  =( 30300| GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Delay of exposure
    GX_FLOAT_EXPOSURE_OVERLAP_TIME_MAX      =( 30301 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< Maximum overlapping exposure time
    GX_ENUM_EXPOSURE_TIME_MODE              =( 30302 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Exposure time mode, refer to GX_EXPOSURE_TIME_MODE_ENTRY
    GX_ENUM_ACQUISITION_BURST_MODE          =( 30303 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Burst acquisition mode
    GX_ENUM_OVERLAP_MODE                    =( 30304 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< overlap mode,refer to GX_OVERLAP_MODE_ENTRY
    GX_ENUM_MULTISOURCE_SELECTOR            =( 30305 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< MultiSourceSelector to GX_MULTISOURCE_SELECTOR_ENTRY
    GX_BOOL_MULTISOURCE_ENABLE              =( 30306 | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< MultiSource Trigger Enable
    GX_BOOL_TRIGGER_CACHE_ENABLE            =( 30307 | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32| GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< Trigger Cache Enable

    GX_FLOAT_GAIN                     = (5011 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< The value is an float value that sets the selected gain control in units specific to the camera.
    
    // todolist now 1-2-3 is enough~ :D
    // DigitalIO 4000
    // AnalogControl 5000
    // CounterAndTimer 6000
    // UserSetControl 7000
    // Event 8000
    // LUT 9000
    // ChunkData 10000
    // Color Transformation 11000
    // CounterAndTimerControl 12000
    // RemoveParameterLimitControl 13000
    // HDRControl 14000
    // MultiGrayControl 15000
    // ImageQualityControl 16000
    // GyroControl 17000
    // FrameBufferControl 18000
    // SerialPortControl 19000
    // EnoderControl 22000



    // here is the origin debug version's history :D

    // GX_COMMAND_ACQUISITION_START = (3001 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize,
    // GX_COMMAND_ACQUISITION_STOP = (3002| GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,
    // GX_COMMAND_TRIGGER_SOFTWARE = (3006| GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,
    // GX_COMMAND_ACQUISITION_START = 3001 | 1879048192 | 0,
    // GX_COMMAND_ACQUISITION_STOP = 3002| 1879048192 | 0,
    // GX_COMMAND_TRIGGER_SOFTWARE = 3006| 1879048192 | 0,
    // GX_INT_WIDTH                      = 1006 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Width of the image provided by the device (in pixels).
    // GX_INT_HEIGHT                     = 1007 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Height of the image provided by the device (in pixels).
    // GX_INT_WIDTH = 1006 | 268435456 | 0,
    // GX_INT_HEIGHT = 1007 | 268435456 | 0,
    // GX_INT_PAYLOAD_SIZE = 2000 | 268435456 | 0,

    // GX_ENUM_PIXEL_SIZE                = 1012 | 805306368 | 0,    //< Total size in bits of a pixel of the image.
    // GX_ENUM_PIXEL_FORMAT              = 1014 | 805306368 | 0,    //< Format of the pixels provided by the device.
   


}

// 下面是3.5写的，大多数应该是用不到的


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_STATUS_LIST {
    GX_STATUS_SUCCESS = 0,
    GX_STATUS_ERROR = -1,
    GX_STATUS_NOT_FOUND_TL = -2,
    GX_STATUS_NOT_FOUND_DEVICE = -3,
    GX_STATUS_OFFLINE = -4,
    GX_STATUS_INVALID_PARAMETER = -5,
    GX_STATUS_INVALID_HANDLE = -6,
    GX_STATUS_INVALID_CALL = -7,
    GX_STATUS_INVALID_ACCESS = -8,
    GX_STATUS_NEED_MORE_BUFFER = -9,
    GX_STATUS_ERROR_TYPE = -10,
    GX_STATUS_OUT_OF_RANGE = -11,
    GX_STATUS_NOT_IMPLEMENTED = -12,
    GX_STATUS_NOT_INIT_API = -13,
    GX_STATUS_TIMEOUT = -14,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FRAME_STATUS_LIST {
    GX_FRAME_STATUS_SUCCESS = 0,
    GX_FRAME_STATUS_INCOMPLETE = -1,
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_FRAME_STATUS {
//     GX_FRAME_STATUS_SUCCESS = 0,
//     GX_FRAME_STATUS_INCOMPLETE = -1,
// }


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS_LIST {
    GX_DEVICE_CLASS_UNKNOWN = 0,
    GX_DEVICE_CLASS_USB2 = 1,
    GX_DEVICE_CLASS_GEV = 2,
    GX_DEVICE_CLASS_U3V = 3,
}


// #[repr(u32)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_FEATURE_TYPE {
//     GX_FEATURE_INT = 0x10000000,
//     GX_FEATURE_FLOAT = 0x20000000,
//     GX_FEATURE_ENUM = 0x30000000,
//     GX_FEATURE_BOOL = 0x40000000,
//     GX_FEATURE_STRING = 0x50000000,
//     GX_FEATURE_BUFFER = 0x60000000,
//     GX_FEATURE_COMMAND = 0x70000000,
// }

// #[repr(u32)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_FEATURE_LEVEL {
//     GX_FEATURE_LEVEL_REMOTE_DEV = 0x00000000,
//     GX_FEATURE_LEVEL_TL = 0x01000000,
//     GX_FEATURE_LEVEL_IF = 0x02000000,
//     GX_FEATURE_LEVEL_DEV = 0x03000000,
//     GX_FEATURE_LEVEL_DS = 0x04000000,
// }

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_ACCESS_MODE {
//     GX_ACCESS_READONLY = 2,
//     GX_ACCESS_CONTROL = 3,
//     GX_ACCESS_EXCLUSIVE = 4,
// }

pub type GX_ACCESS_MODE_CMD = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_STATUS {
    GX_ACCESS_STATUS_UNKNOWN = 0,
    GX_ACCESS_STATUS_READWRITE = 1,
    GX_ACCESS_STATUS_READONLY = 2,
    GX_ACCESS_STATUS_NOACCESS = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_OPEN_MODE {
    GX_OPEN_SN = 0,
    GX_OPEN_IP = 1,
    GX_OPEN_MAC = 2,
    GX_OPEN_INDEX = 3,
    GX_OPEN_USERID = 4,
}

// pub type GX_OPEN_MODE_CMD = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_IP_CONFIGURE_MODE_LIST {
    GX_IP_CONFIGURE_DHCP,
    GX_IP_CONFIGURE_LLA,
    GX_IP_CONFIGURE_STATIC_IP,
    GX_IP_CONFIGURE_DEFAULT,
}

pub type GX_IP_CONFIGURE_MODE = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_RESET_DEVICE_MODE {
    GX_MANUFACTURER_SPECIFIC_RECONNECT = 0x1,
    GX_MANUFACTURER_SPECIFIC_RESET = 0x2,
}

pub type GX_RESET_DEVICE_MODE_CMD = i32;
