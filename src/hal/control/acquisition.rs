//! Placeholder
use crate::hal::config::*;

use crate::raw::gx_enum::GX_FEATURE_ID;
use crate::raw::gx_interface::Result;


// - acquisition
// - [ ] gxi_acquisition_mode()
// - [ ] gxi_acquisition_start()
// - [ ] gxi_acquisition_stop()
// -
// - [ ] gxi_exposure_time_mode()
// - 
// - [ ] gxi_get_exposure_time()
// - [ ] gxi_set_exposure_time()
// - 
// - [ ] gxi_exposure_time_auto()
// - [ ] gxi_exposure_time_auto_off()
// - [ ] gxi_exposure_time_auto_continuous()
// - [ ] gxi_exposure_time_auto_once()
// - 
// - [ ] gxi_set_auto_gain_min()
// - [ ] gxi_set_auto_gain_max() 
// - todo!()

#[cfg(feature = "solo")]
pub fn gxi_get_exposure_time() -> Result<f64> {
    let exp_time:f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_EXPOSURE_TIME)?;
    println!("Now, exposure time is {}",exp_time);
    Ok(exp_time)
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_time(exp_time:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_EXPOSURE_TIME, &exp_time)?;    
    println!("Successfully set exposure time to {}",exp_time);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_exposure_time_min(min:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_EXPOSURE_TIME_MIN, &min)?;    
    println!("Successfully set auto exposure time min to {}",min);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_exposure_time_max(max:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_EXPOSURE_TIME_MAX, &max)?;    
    println!("Successfully set auto exposure time max to {}",max);
    Ok(())
}