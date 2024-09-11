//! Placeholder
use crate::hal::config::*;

use crate::raw::gx_enum::GX_FEATURE_ID;
use crate::raw::gx_interface::Result;

#[cfg(feature = "solo")]
pub fn gxi_get_gain() -> Result<f64> {
    let gain:f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_GAIN)?;
    println!("Gain is {}",gain);
    Ok(gain)
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain(gain:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_GAIN, &gain)?;    
    println!("Successfully set gain to {}",gain);
    Ok(())
}


#[cfg(feature = "solo")]
pub fn gxi_set_exposure_time(exp_time:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_EXPOSURE_TIME, &exp_time)?;    
    println!("Successfully set exposure time to {}",exp_time);
    Ok(())
}