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

上面是需要的格式类型，有2个要点
1. 把后面的///< 改成//<
2. 两个as u32 ，一个大括号包括全部，然后最外面是as isize
3. 给中间的那个加上前缀GX_FEATURE_TYPE::或者GX_FEATURE_LEVEL::

现在需要处理的在下面
大概是这样的，等有时间我们在327一起来调吧
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


//----------------AnalogControls Section----------------------------
GX_ENUM_GAIN_AUTO                 = 5000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Sets the automatic gain control (AGC) mode.
GX_ENUM_GAIN_SELECTOR             = 5001 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Selects which Gain is controlled by the various Gain features.
GX_ENUM_BLACKLEVEL_AUTO           = 5003 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Controls the mode for automatic black level adjustment.
GX_ENUM_BLACKLEVEL_SELECTOR       = 5004 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Selects which Black Level is controlled by the various Black Level features.
GX_ENUM_BALANCE_WHITE_AUTO        = 5006 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Controls the mode for automatic white balancing between the color channels.
GX_ENUM_BALANCE_RATIO_SELECTOR    = 5007 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Selects which Balance ratio to control.
GX_FLOAT_BALANCE_RATIO            = 5008 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Controls ratio of the selected color component to a reference color component.
GX_ENUM_COLOR_CORRECT             = 5009 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Color correction, see also GX_COLOR_CORRECT_ENTRY
GX_ENUM_DEAD_PIXEL_CORRECT        = 5010 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< The dead pixel correct function can eliminate dead pixels in the image, see also GX_DEAD_PIXEL_CORRECT_ENTRY
GX_FLOAT_GAIN                     = 5011 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< The value is an float value that sets the selected gain control in units specific to the camera.
GX_FLOAT_BLACKLEVEL               = 5012 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Controls the analog black level as an absolute physical value.
GX_BOOL_GAMMA_ENABLE              = 5013 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Enable bit of Gamma
GX_ENUM_GAMMA_MODE                = 5014 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Gamma select, see also GX_GAMMA_MODE_ENTRY
GX_FLOAT_GAMMA                    = 5015 | GX_FEATURE_FLOAT| GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Gamma
GX_INT_DIGITAL_SHIFT              = 5016 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< bit select
GX_ENUM_LIGHT_SOURCE_PRESET       = 5017 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Ambient Light Presets, refer to GX_LIGHT_SOURCE_PRESET_ENTRY
GX_BOOL_BLACKLEVEL_CALIB_STATUS   = 5018 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV,	///< BlackLevelCalibStatus
GX_INT_BLACKLEVEL_CALIB_VALUE     = 5019 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,	///< BlackLevelCalibValue

//---------------CustomFeature Section-------------------------
GX_INT_ADC_LEVEL                  = 6000 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< When the pixel size is not 8bits, this function can be used to choose 8bits form 10bits or 12bit for show image.
GX_INT_H_BLANKING                 = 6001 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Horizontal blanking
GX_INT_V_BLANKING                 = 6002 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Vertical blanking
GX_STRING_USER_PASSWORD           = 6003 | GX_FEATURE_STRING | GX_FEATURE_LEVEL_REMOTE_DEV, ///< user password
GX_STRING_VERIFY_PASSWORD         = 6004 | GX_FEATURE_STRING | GX_FEATURE_LEVEL_REMOTE_DEV, ///< verify password
GX_BUFFER_USER_DATA               = 6005 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV, ///< user data
GX_INT_GRAY_VALUE                 = 6006 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< ExpectedGrayValue_InqIsImplemented
GX_ENUM_AA_LIGHT_ENVIRONMENT      = 6007 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Automatic function according to the external light conditions better for accommodation, see also GX_AA_LIGHT_ENVIRMENT_ENTRY
GX_INT_AAROI_OFFSETX              = 6008 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the X offset (left offset) for the rect of interest in pixels for 2A, i.e., the distance in pixels between the left side of the image area and the left side of the AAROI.
GX_INT_AAROI_OFFSETY              = 6009 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the Y offset (top offset) for the rect of interest for 2A, i.e., the distance in pixels between the top of the image area and the top of the AAROI.
GX_INT_AAROI_WIDTH                = 6010 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the width of the rect of interest in pixels for 2A.
GX_INT_AAROI_HEIGHT               = 6011 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the height of the rect of interest in pixels for 2A.
GX_FLOAT_AUTO_GAIN_MIN            = 6012 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Setting up automatic gain range of minimum. When the gain is set to auto mode, this function works.
GX_FLOAT_AUTO_GAIN_MAX            = 6013 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Setting up automatic gain range of maximum. When the gain is set to auto mode, this function works.
GX_FLOAT_AUTO_EXPOSURE_TIME_MIN   = 6014 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Setting up automatic shutter range of minimum. When the shutter is set to auto mode, this function works.
GX_FLOAT_AUTO_EXPOSURE_TIME_MAX   = 6015 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Setting up automatic shutter range of maximum. When the shutter is set to auto mode, this function works.
GX_BUFFER_FRAME_INFORMATION       = 6016 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV, ///< FrameInformation
GX_INT_CONTRAST_PARAM             = 6017 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Contrast parameter
GX_FLOAT_GAMMA_PARAM              = 6018 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Gamma parameter
GX_INT_COLOR_CORRECTION_PARAM     = 6019 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Color correction coefficient
GX_ENUM_IMAGE_GRAY_RAISE_SWITCH   = 6020 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Control ImageGrayRaise is valid, see also GX_IMAGE_GRAY_RAISE_SWITCH_ENTRY
GX_ENUM_AWB_LAMP_HOUSE            = 6021 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Refers to the AWB working environment, see also GX_AWB_LAMP_HOUSE_ENTRY
GX_INT_AWBROI_OFFSETX             = 6022 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the X offset (left offset) for the rect of interest in pixels for Auto WhiteBalance
GX_INT_AWBROI_OFFSETY             = 6023 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the Y offset (top offset) for the rect of interest for Auto WhiteBalance
GX_INT_AWBROI_WIDTH               = 6024 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the width of the rect of interest in pixels for Auto WhiteBalance
GX_INT_AWBROI_HEIGHT              = 6025 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This value sets the height of the rect of interest in pixels for Auto WhiteBalance
GX_ENUM_SHARPNESS_MODE            = 6026 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Sharpening mode, see also GX_SHARPNESS_MODE_ENTRY
GX_FLOAT_SHARPNESS                = 6027 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Sharpness
GX_ENUM_USER_DATA_FILED_SELECTOR  = 6028 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< User selects Flash data area refer to GX_USER_DATA_FILED_SELECTOR_ENTRY
GX_BUFFER_USER_DATA_FILED_VALUE   = 6029 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV, ///< User Area Content
GX_ENUM_FLAT_FIELD_CORRECTION     = 6030 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Flat field correction switch, refer to GX_FLAT_FIELD_CORRECTION_ENTRY
GX_ENUM_NOISE_REDUCTION_MODE      = 6031 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Noise reduction switch, refer to GX_NOISE_REDUCTION_MODE_ENTRY
GX_FLOAT_NOISE_REDUCTION          = 6032 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Noise reduction
GX_BUFFER_FFCLOAD				  = 6033 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Obtain flat field correction parameters
GX_BUFFER_FFCSAVE				  = 6034 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Set flat field correction parameters
GX_ENUM_STATIC_DEFECT_CORRECTION  = 6035 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Static bad point correction refer to GX_ENUM_STATIC_DEFECT_CORRECTION_ENTRY
GX_ENUM_2D_NOISE_REDUCTION_MODE   = 6036 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Refer to GX_2D_NOISE_REDUCTION_MODE_ENTRY
GX_ENUM_3D_NOISE_REDUCTION_MODE   = 6037 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Refer to GX_3D_NOISE_REDUCTION_MODE_ENTRY
GX_COMMAND_CLOSE_ISP              = 6038 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV,///< Close ISP
GX_BUFFER_STATIC_DEFECT_CORRECTION_VALUE_ALL    = 6039 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< static defect conrrection value Refer to GX_BUFFER_FFCSAVE
GX_BUFFER_STATIC_DEFECT_CORRECTION_FLASH_VALUE  = 6040 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV,   ///<static defect conrrection flash value Refer to GX_BUFFER_FFCSAVE
GX_INT_STATIC_DEFECT_CORRECTION_FINISH          = 6041 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,      ///< static defect conrrection finish Refer to GX_INT_AWBROI_HEIGHT
GX_BUFFER_STATIC_DEFECT_CORRECTION_INFO         = 6042 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< static defect conrrection Info Refer to GX_BUFFER_FFCSAVE
GX_COMMAND_STRIP_CALIBRATION_START              = 6043 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Starts the strip calibration
GX_COMMAND_STRIP_CALIBRATION_STOP               = 6044 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Ready to stop the strip calibration

//---------------UserSetControl Section-------------------------
GX_ENUM_USER_SET_SELECTOR         = 7000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Selects the feature User Set to load, save or configure.
GX_COMMAND_USER_SET_LOAD          = 7001 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Loads the User Set specified by UserSetSelector to the device and makes it active.
GX_COMMAND_USER_SET_SAVE          = 7002 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Save the User Set specified by UserSetSelector to the non-volatile memory of the device.
GX_ENUM_USER_SET_DEFAULT          = 7003 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Selects the feature User Set to load and make active by default when the device is reset.

//---------------Event Section-------------------------
GX_ENUM_EVENT_SELECTOR             = 8000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Selects which Event to signal to the host application.
GX_ENUM_EVENT_NOTIFICATION         = 8001 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Activate or deactivate the notification to the host application of the occurrence of the selected Event.
GX_INT_EVENT_EXPOSUREEND           = 8002 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Returns the unique identifier of the ExposureEnd type of Event.
GX_INT_EVENT_EXPOSUREEND_TIMESTAMP = 8003 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Returns the Timestamp of the ExposureEnd Event.
GX_INT_EVENT_EXPOSUREEND_FRAMEID   = 8004 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Returns the unique Identifier of the Frame (or image) that generated the ExposureEnd Event.
GX_INT_EVENT_BLOCK_DISCARD         = 8005 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< This enumeration value indicates the BlockDiscard event ID.
GX_INT_EVENT_BLOCK_DISCARD_TIMESTAMP = 8006 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Indicates the time stamp for the BlockDiscard event
GX_INT_EVENT_OVERRUN                 = 8007 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< This enumeration value indicates the EventOverrun event ID.
GX_INT_EVENT_OVERRUN_TIMESTAMP       = 8008 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Indicates the time stamp of the EventOverrun event
GX_INT_EVENT_FRAMESTART_OVERTRIGGER  = 8009 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< This enumeration value indicates the FrameStartOverTrigger event ID.
GX_INT_EVENT_FRAMESTART_OVERTRIGGER_TIMESTAMP = 8010 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Indicates the time stamp of the FrameStartOverTrigger event
GX_INT_EVENT_BLOCK_NOT_EMPTY                  = 8011 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< This enumeration value indicates the BlockNotEmpty event.
GX_INT_EVENT_BLOCK_NOT_EMPTY_TIMESTAMP        = 8012 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Indicates the time stamp of the BlockNotEmpty event
GX_INT_EVENT_INTERNAL_ERROR                   = 8013 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< This enumeration value indicates the InternalError event.
GX_INT_EVENT_INTERNAL_ERROR_TIMESTAMP         = 8014 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Indicates the time stamp of the InternalError event
GX_INT_EVENT_FRAMEBURSTSTART_OVERTRIGGER      = 8015 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Multi frame trigger mask event ID
GX_INT_EVENT_FRAMEBURSTSTART_OVERTRIGGER_FRAMEID      = 8016 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Multi frame trigger mask event frame ID
GX_INT_EVENT_FRAMEBURSTSTART_OVERTRIGGER_TIMESTAMP    = 8017 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Multi frame trigger mask event timestamp
GX_INT_EVENT_FRAMESTART_WAIT                          = 8018 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Frame Wait Event ID
GX_INT_EVENT_FRAMESTART_WAIT_TIMESTAMP                = 8019 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Frame Wait Event Timestamp
GX_INT_EVENT_FRAMEBURSTSTART_WAIT                     = 8020 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Multi frame waiting event ID
GX_INT_EVENT_FRAMEBURSTSTART_WAIT_TIMESTAMP           = 8021 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Multi frame waiting event timestamp
GX_INT_EVENT_BLOCK_DISCARD_FRAMEID                    = 8022 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Block Loss Event Frame ID
GX_INT_EVENT_FRAMESTART_OVERTRIGGER_FRAMEID           = 8023 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Trigger signal masked event frame ID
GX_INT_EVENT_BLOCK_NOT_EMPTY_FRAMEID                  = 8024 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< The frame memory is not empty Event frame ID
GX_INT_EVENT_FRAMESTART_WAIT_FRAMEID                  = 8025 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Frame Wait Event Frame ID
GX_INT_EVENT_FRAMEBURSTSTART_WAIT_FRAMEID             = 8026 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Multi frame waiting event frame ID
GX_ENUM_EVENT_SIMPLE_MODE						= 8027 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,		    ///< event block ID enable,refer to GX_EVENT_SIMPLE_MODE_ENTRY

//---------------LUT Section-------------------------
GX_ENUM_LUT_SELECTOR             = 9000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Selects which LUT to control.
GX_BUFFER_LUT_VALUEALL           = 9001 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Accesses all the LUT coefficients in a single access without using individual LUTIndex.
GX_BOOL_LUT_ENABLE               = 9002 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< Activates the selected LUT.
GX_INT_LUT_INDEX                 = 9003 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Control the index (offset) of the coefficient to access in the selected LUT.
GX_INT_LUT_VALUE                 = 9004 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Returns the Value at entry LUTIndex of the LUT selected by LUTSelector.

//---------------ChunkData Section-------------------------
GX_BOOL_CHUNKMODE_ACTIVE         = 10001 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Activates the inclusion of Chunk data in the payload of the image.
GX_ENUM_CHUNK_SELECTOR           = 10002 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Selects which Chunk to enable or control.
GX_BOOL_CHUNK_ENABLE             = 10003 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Enables the inclusion of the selected Chunk data in the payload of the image.

//---------------Color Transformation Control-------------------------
GX_ENUM_COLOR_TRANSFORMATION_MODE       = 11000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Color conversion selection, see also GX_COLOR_TRANSFORMATION_MODE_ENTRY
GX_BOOL_COLOR_TRANSFORMATION_ENABLE     = 11001 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Activates the selected Color Transformation module.
GX_ENUM_COLOR_TRANSFORMATION_VALUE_SELECTOR = 11002 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Selects the Gain factor or Offset of the Transformation matrix to access in the selected Color Transformation module.
GX_FLOAT_COLOR_TRANSFORMATION_VALUE     = 11003 | GX_FEATURE_FLOAT| GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Represents the value of the selected Gain factor or Offset inside the Transformation matrix.
GX_ENUM_SATURATION_MODE                 = 11004 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Saturation Mode refer to GX_ENUM_SATURATION_MODE_ENTRY
GX_INT_SATURATION                       = 11005 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,      ///< Saturation

//---------------CounterAndTimerControl Section-------------------------
GX_ENUM_TIMER_SELECTOR                  = 12000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Selects which Counter to configure, Refer to GX_TIMER_SELECTOR_ENTRY
GX_FLOAT_TIMER_DURATION                 = 12001 | GX_FEATURE_FLOAT| GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Sets the duration (in microseconds) of the Timer pulse.
GX_FLOAT_TIMER_DELAY                    = 12002 | GX_FEATURE_FLOAT| GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Sets the duration (in microseconds) of the delay to apply at the reception of a trigger before starting the Timer.
GX_ENUM_TIMER_TRIGGER_SOURCE            = 12003 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Selects the source of the trigger to start the Timer, Refer to GX_TIMER_TRIGGER_SOURCE_ENTRY
GX_ENUM_COUNTER_SELECTOR                = 12004 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Selects which Counter to configure, Refer to GX_COUNTER_SELECTOR_ENTRY
GX_ENUM_COUNTER_EVENT_SOURCE            = 12005 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Select the events that will be the source to increment the Counter, Refer to GX_COUNTER_EVENT_SOURCE_ENTRY
GX_ENUM_COUNTER_RESET_SOURCE            = 12006 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Selects the signals that will be the source to reset the Counter, Refer to GX_COUNTER_RESET_SOURCE_ENTRY
GX_ENUM_COUNTER_RESET_ACTIVATION        = 12007 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Selects the Activation mode of the Counter Reset Source signal, Refer to GX_COUNTER_RESET_ACTIVATION_ENTRY
GX_COMMAND_COUNTER_RESET                = 12008 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< Does a software reset of the selected Counter and starts it.
GX_ENUM_COUNTER_TRIGGER_SOURCE          = 12009 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Counter trigger source refer to GX_COUNTER_TRIGGER_SOURCE_ENTRY
GX_INT_COUNTER_DURATION					= 12010 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Counter Duration
GX_ENUM_TIMER_TRIGGER_ACTIVATION		= 12011 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,     ///< Timer Trigger Activation see also GX_TIMER_TRIGGER_ACTIVATION_ENTRY

//---------------RemoveParameterLimitControl Section-------------------------
GX_ENUM_REMOVE_PARAMETER_LIMIT          = 13000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Cancel parameter range restriction, refer to GX_REMOVE_PARAMETER_LIMIT_ENTRY

//---------------HDRControl Section-------------------------
GX_ENUM_HDR_MODE                = 14000 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,            ///< Refer to GX_HDR_MODE_ENTRY
GX_INT_HDR_TARGET_LONG_VALUE    = 14001 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,             ///< 
GX_INT_HDR_TARGET_SHORT_VALUE   = 14002 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,             ///< 
GX_INT_HDR_TARGET_MAIN_VALUE    = 14003 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,             ///< 

//---------------MultiGrayControl Section-------------------------
GX_ENUM_MGC_MODE                = 15001 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,            ///< Refer to GX_MGC_MODE_ENTRY
GX_INT_MGC_SELECTOR             = 15002 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,             ///< 
GX_FLOAT_MGC_EXPOSURE_TIME      = 15003 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,           ///< 
GX_FLOAT_MGC_GAIN               = 15004 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,           ///< 

//---------------ImageQualityControl Section-------------------------
GX_BUFFER_STRIPED_CALIBRATION_INFO                     = 16001 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Fringe calibration information Refer to GX_BUFFER_STATIC_DEFECT_CORRECTION_INFO
GX_FLOAT_CONTRAST                                      = 16002 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,	 ///< Contrast

//---------------GyroControl Section-------------------------
GX_BUFFER_IMU_DATA                                     = 17001 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< IMU data
GX_ENUM_IMU_CONFIG_ACC_RANGE                           = 17002 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< IMU config acc range, refer to GX_IMU_CONFIG_ACC_RANGE_ENTRY
GX_ENUM_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_SWITCH      = 17003 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< IMU config acc odr low pass filter switch, refer to GX_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_SWITCH_ENTRY
GX_ENUM_IMU_CONFIG_ACC_ODR                             = 17004 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< IMU config acc odr, refer to GX_IMU_CONFIG_ACC_ODR_ENTRY
GX_ENUM_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_FREQUENCY   = 17005 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu config acc odr low pass filter frequency, refer to GX_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_FREQUENCY_ENTRY
GX_ENUM_IMU_CONFIG_GYRO_XRANGE                         = 17006 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu config gyro Xrange, refer to GX_IMU_CONFIG_GYRO_RANGE_ENTRY
GX_ENUM_IMU_CONFIG_GYRO_YRANGE                         = 17007 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu config gyro Yrange, refer to GX_IMU_CONFIG_GYRO_RANGE_ENTRY
GX_ENUM_IMU_CONFIG_GYRO_ZRANGE                         = 17008 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu config gyro Zrange, refer to GX_IMU_CONFIG_GYRO_RANGE_ENTRY
GX_ENUM_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_SWITCH     = 17009 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu config gyro odr low pass filter switch, refer to GX_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_SWITCH_ENTRY
GX_ENUM_IMU_CONFIG_GYRO_ODR                            = 17010 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu config gyro odr, refer to GX_IMU_CONFIG_GYRO_ODR_ENTRY
GX_ENUM_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_FREQUENCY  = 17011 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu config gyro odr low pass filter frequency, refer to GX_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_FREQUENCY_ENTRY
GX_FLOAT_IMU_ROOM_TEMPERATURE                          = 17012 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< imu room temperature
GX_ENUM_IMU_TEMPERATURE_ODR                            = 17013 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< imu temperature odr, refer to GX_IMU_TEMPERATURE_ODR_ENTRY

//---------------FrameBufferControl Section-------------------------
GX_INT_FRAME_BUFFER_COUNT         = 18001 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Frame memory depth
GX_COMMAND_FRAME_BUFFER_FLUSH     = 18002 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Empty the frame save  

//----------------SerialPortControl Section----------------------------------
GX_ENUM_SERIALPORT_SELECTOR			= 19001 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Serial port selection
GX_ENUM_SERIALPORT_SOURCE			= 19002 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Serial port input source
GX_ENUM_SERIALPORT_BAUDRATE			= 19003 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Serial baud rate
GX_INT_SERIALPORT_DATA_BITS			= 19004 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,					///< Serial port data bit
GX_ENUM_SERIALPORT_STOP_BITS		= 19005 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Serial port stop bit
GX_ENUM_SERIALPORT_PARITY			= 19006 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Serial port parity
GX_INT_TRANSMIT_QUEUE_MAX_CHARACTER_COUNT		= 19007 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,		///< Maximum number of characters in transmission queue
GX_INT_TRANSMIT_QUEUE_CURRENT_CHARACTER_COUNT	= 19008 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,		///< Current number of characters in the transmission queue
GX_INT_RECEIVE_QUEUE_MAX_CHARACTER_COUNT		= 19009 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,		///< Maximum number of characters in receive queue
GX_INT_RECEIVE_QUEUE_CURRENT_CHARACTER_COUNT	= 19010 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,		///< Current number of characters in the receive queue
GX_INT_RECEIVE_FRAMING_ERROR_COUNT		= 19011 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Received frame error count
GX_INT_RECEIVE_PARITY_ERROR_COUNT		= 19012 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Receive parity error count
GX_COMMAND_RECEIVE_QUEUE_CLEAR			= 19013 | GX_FEATURE_COMMAND | GX_FEATURE_LEVEL_REMOTE_DEV,			///< Queue Clear
GX_BUFFER_SERIALPORT_DATA				= 19014 | GX_FEATURE_BUFFER | GX_FEATURE_LEVEL_REMOTE_DEV,			///< serial data
GX_INT_SERIALPORT_DATA_LENGTH			= 19015 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,				///< Serial port data length 

//--------------EnoderControl Section-------------------------
GX_ENUM_ENCODER_SELECTOR				= 22001 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Encoder selector
GX_ENUM_ENCODER_DIRECTION				= 22002 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Encoder direction
GX_INT_ENCODER_VALUE			        = 22003 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Decoder value
GX_ENUM_ENCODER_SOURCEA					= 22004 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,	///< Encoder phase A input
GX_ENUM_ENCODER_SOURCEB					= 22005 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,	///< Encoder phase B input
GX_ENUM_ENCODER_MODE				    = 22006 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< Encoder Mode

//////////////////////////////////////////////////////////////////////////
/// Local device layer(Device Feature)
//////////////////////////////////////////////////////////////////////////
GX_DEV_INT_COMMAND_TIMEOUT     = 0 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DEV, ///< Indicates the current command timeout of the specific Link.
GX_DEV_INT_COMMAND_RETRY_COUNT = 1 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DEV, ///< Command retry times

//////////////////////////////////////////////////////////////////////////
/// Flow layer(DataStream Feature)
//////////////////////////////////////////////////////////////////////////
GX_DS_INT_ANNOUNCED_BUFFER_COUNT          = 0 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Number of Buffers declared
GX_DS_INT_DELIVERED_FRAME_COUNT           = 1 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Number of received frames (including residual frames)
GX_DS_INT_LOST_FRAME_COUNT                = 2 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Number of frames lost due to insufficient buffers
GX_DS_INT_INCOMPLETE_FRAME_COUNT          = 3 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Number of residual frames received
GX_DS_INT_DELIVERED_PACKET_COUNT          = 4 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Number of packets received
GX_DS_INT_RESEND_PACKET_COUNT             = 5 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Number of retransmission packets
GX_DS_INT_RESCUED_PACKED_COUNT            = 6 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Number of successful retransmitted packets
GX_DS_INT_RESEND_COMMAND_COUNT            = 7 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Repeat command times
GX_DS_INT_UNEXPECTED_PACKED_COUNT         = 8 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Exception packet number
GX_DS_INT_MAX_PACKET_COUNT_IN_ONE_BLOCK   = 9 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,   ///< Maximum number of retransmissions of data blocks
GX_DS_INT_MAX_PACKET_COUNT_IN_ONE_COMMAND = 10 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Maximum number of packets contained in a retransmit command
GX_DS_INT_RESEND_TIMEOUT                  = 11 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Retransmission timeout time
GX_DS_INT_MAX_WAIT_PACKET_COUNT           = 12 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Maximum waiting packet number
GX_DS_ENUM_RESEND_MODE                    = 13 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_DS, ///< Retransmission, see also GX_DS_RESEND_MODE_ENTRY
GX_DS_INT_MISSING_BLOCKID_COUNT           = 14 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Missing number of BlockID
GX_DS_INT_BLOCK_TIMEOUT                   = 15 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Data block timeout
GX_DS_INT_STREAM_TRANSFER_SIZE            = 16 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< size of transfer block
GX_DS_INT_STREAM_TRANSFER_NUMBER_URB      = 17 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Number of data blocks transmitted
GX_DS_INT_PACKET_TIMEOUT                  = 19 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< time of package timeout
GX_DS_INT_SOCKET_BUFFER_SIZE			  = 20 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Socket buffer size in kilobytes
GX_DS_ENUM_STOP_ACQUISITION_MODE		  = 21 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_DS, ///< stop acquisition mode
GX_DS_ENUM_STREAM_BUFFER_HANDLING_MODE    = 22 | GX_FEATURE_ENUM| GX_FEATURE_LEVEL_DS,  ///< Buffer processing mode, refer to GX_DS_STREAM_BUFFER_HANDLING_MODE_ENTRY
GX_DS_INT_ACQUISITION_BUFFER_CACHE_PREC   = 23 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,  ///< Number of buffer caches collected
GX_DS_ENUM_MULTI_RESEND_MODE			  = 24 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_DS, ///< Retransmission, see also GX_DS_MULTI_RESEND_MODE_ENTRY

//////////////////////////////////////////////////////////////////////////
/// Deprecated Section
//////////////////////////////////////////////////////////////////////////
GX_STRING_DEVICE_ID               = 4    | GX_FEATURE_STRING | GX_FEATURE_LEVEL_REMOTE_DEV, ///< switch to GX_STRING_DEVICE_SERIAL_NUMBER	
GX_STRING_DEVICE_HARDWARE_VERSION = 5    | GX_FEATURE_STRING | GX_FEATURE_LEVEL_REMOTE_DEV, ///< Device hardware version
GX_INT_GAIN                       = 5002 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_FLOAT_GAIN
GX_INT_BLACKLEVEL                 = 5005 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_FLOAT_BLACKLEVEL
GX_FLOAT_BALANCE_RATIO_SELECTOR   = 5007 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< switch to GX_ENUM_BALANCE_RATIO_SELECTOR
GX_ENUM_AA_LIGHT_ENVIRMENT        = 6007 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   ///< switch to GX_ENUM_AA_LIGHT_ENVIRONMENT
GX_INT_ROI_X                      = 6008 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_INT_AAROI_OFFSETX
GX_INT_ROI_Y                      = 6009 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_INT_AAROI_OFFSETY
GX_INT_ROI_WIDTH                  = 6010 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_INT_AAROI_WIDTH
GX_INT_ROI_HEIGHT                 = 6011 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_INT_AAROI_HEIGHT
GX_INT_AUTO_GAIN_VALUEMIN         = 6012 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_FLOAT_AUTO_GAIN_MIN
GX_INT_AUTO_GAIN_VALUEMAX         = 6013 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_FLOAT_AUTO_GAIN_MAX
GX_INT_AUTO_SHUTTER_VALUEMIN      = 6014 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_FLOAT_AUTO_EXPOSURE_TIME_MIN
GX_INT_AUTO_SHUTTER_VALUEMAX      = 6015 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_FLOAT_AUTO_EXPOSURE_TIME_MAX
GX_INT_CONTRASTPARAM              = 6017 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_INT_CONTRAST_PARAM
GX_FLOAT_GAMMAPARAM               = 6018 | GX_FEATURE_FLOAT | GX_FEATURE_LEVEL_REMOTE_DEV,  ///< switch to GX_FLOAT_GAMMA_PARAM
GX_INT_COLORCORRECTIONPARAM       = 6019 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,    ///< switch to GX_INT_COLOR_CORRECTION_PARAM
GX_DS_INT_MAX_NUM_QUEUE_BUFFER    = 18 | GX_FEATURE_INT | GX_FEATURE_LEVEL_DS,              ///< the max number queue buffer
