// use crate::hal::basement::{GXI,gxi_check};
// use crate::raw::{gx_interface::*, gx_const::*, gx_struct::*, gx_enum::*,gx_handle::*};
// use crate::utils::builder::GXDeviceBaseInfoBuilder;

// pub fn gxi_set_float(handle: GX_DEV_HANDLE, feature_id: GX_FEATURE_ID, value: f64) -> Result<(), GxciError> {
//     unsafe {
//         let status = GXI.as_ref().ok_or_else(|| GxciError::InitializationError("GXI is None. Please check your gxci_init situation.".to_string()))?
//             .gx_set_float(handle, feature_id, value)?;
//         if status == 0 {
//             Ok(())
//         } else {
//             Err(GxciError::GalaxyError(status))
//         }
//     }
// }