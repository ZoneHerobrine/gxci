//! Placeholder
use crate::hal::config::*;

use crate::raw::gx_enum::GX_FEATURE_ID;
use crate::raw::gx_interface::Result;

// - analog
// - [x] gxi_get_gain()
// - [x] gxi_set_gain()
// - 
// - [ ] gxi_gain_auto()
// - [ ] gxi_gain_auto_off()
// - [ ] gxi_gain_auto_continuous()
// - [ ] gxi_gain_auto_once()
// - 
// - [ ] gxi_set_auto_gain_min()
// - [ ] gxi_set_auto_gain_max() 
// - 
// - [ ] gxi_set_balance_ratio()
// - 
// - [ ] gxi_balance_ratio_select()
// - [ ] gxi_balance_ratio_select_r()
// - [ ] gxi_balance_ratio_select_g()
// - [ ] gxi_balance_ratio_select_b()
// - 
// - [ ] gxi_balance_white_auto()
// - [ ] gxi_balance_white_auto_off()
// - [ ] gxi_balance_white_auto_continuous()
// - [ ] gxi_balance_white_auto_once()
// - todo!()


#[cfg(feature = "solo")]
pub fn gxi_get_gain() -> Result<f64> {
    let gain:f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_GAIN)?;
    println!("Now, gain is {}",gain);
    Ok(gain)
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain(gain:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_GAIN, &gain)?;    
    println!("Successfully set gain to {}",gain);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_gain_min(min:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_GAIN_MIN, &min)?;    
    println!("Successfully set auto gain min to {}",min);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_gain_max(max:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_GAIN_MAX, &max)?;    
    println!("Successfully set auto gain max to {}",max);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_ratio(ratio:f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_BALANCE_RATIO, &ratio)?;    
    println!("Successfully set balance ratio to {}",ratio);
    Ok(())
}

// #[cfg(feature = "solo")]
// pub fn gxi_balance_ratio_select(ratio:i64) -> Result<()> {
//     gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_RATIO_SELECTOR, &ratio)?;    
//     println!("Successfully set balance ratio selector to {}",ratio);
//     Ok(())
// }

// #[cfg(feature = "solo")]
// pub fn gxi_balance_white_auto() -> Result<()> {
//     gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_WHITE_AUTO, &true)?;    
//     println!("Successfully set balance white auto to true");
//     Ok(())
// }
// it's about GX_GAIN_AUTO_CONTINUOUS... and description.