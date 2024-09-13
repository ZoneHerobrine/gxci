//! Placeholder
use crate::hal::config::*;

use crate::raw::gx_enum::GX_FEATURE_ID;
use crate::raw::gx_interface::Result;

// - chunk_data
// - [ ] gxi_get_chunk_mode_active()
// - [ ] gxi_set_chunk_mode_active()
// - 
// - [ ] gxi_get_chunk_selector()
// - [ ] gxi_set_chunk_selector()
// - 
// - [ ] gxi_get_chunk_enable()
// - [ ] gxi_set_chunk_enable()
// - 
// - [ ] gxi_get_chunk_timestamp()
// - [ ] gxi_set_chunk_timestamp()
// - todo!()

#[cfg(feature = "solo")]
pub fn gxi_get_chunk_mode_active() -> Result<bool> {
    let mode_active:bool = gxi_get_feature_value(GX_FEATURE_ID::GX_BOOL_CHUNKMODE_ACTIVE)?;
    println!("Now, chunk mode active is {}",mode_active);
    Ok(mode_active)
}

#[cfg(feature = "solo")]
pub fn gxi_set_chunk_mode_active(mode_active:bool) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_BOOL_CHUNKMODE_ACTIVE, &mode_active)?;    
    println!("Successfully set chunk mode active to {}",mode_active);
    Ok(())
}

