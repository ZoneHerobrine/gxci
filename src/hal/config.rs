//! Set of Get some values

use crate::hal::base::gxi_check;
use crate::hal::device::gxi_get_device_handle;
use crate::raw::gx_enum::GX_FEATURE_ID;
use crate::raw::gx_interface::{Result,Error,ErrorKind,GxciError,GXInterface};

#[cfg(feature = "solo")]
pub fn gxi_set_bool(feature_id: GX_FEATURE_ID, bool_value:bool) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_set_bool(gxi_device, feature_id, bool_value))?;

    if status == 0 {
        println!("Successfully set bool value.");
        Ok(())
    } else {
        Err(Error::new(ErrorKind::GxciError(GxciError::GalaxyError(status))))
    }
}