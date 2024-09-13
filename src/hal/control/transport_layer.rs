//! Placeholder
use crate::hal::config::*;

use crate::raw::gx_enum::GX_FEATURE_ID;
use crate::raw::gx_interface::Result;

#[cfg(feature = "solo")]
pub fn gxi_get_payload_size() -> Result<u32> {
    let payload_size:u32 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_PAYLOAD_SIZE)?;
    println!("Now, payload size is {}",payload_size);
    Ok(payload_size)
}

// #[cfg(feature = "solo")]
// pub fn gxi_set_payload_size(payload_size:u32) -> Result<()> {
//     gxi_set_feature_value(GX_FEATURE_ID::GX_INT_PAYLOAD_SIZE, &payload_size)?;    
//     println!("Successfully set payload size to {}",payload_size);
//     Ok(())
// }

