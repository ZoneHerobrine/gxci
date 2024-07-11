use crate::raw::{gx_interface::*, gx_const::*, gx_struct::*, gx_enum::*};

pub static mut GXI: Option<GXInstance> = None;

pub fn gxi_check<T, F>(func: F) -> Result<T, GxciError>
where
    F: FnOnce(&GXInstance) -> Result<T, GxciError>,
{
    unsafe {
        if let Some(gxi) = GXI.as_ref() {
            func(gxi)
        } else {
            Err(GxciError::InitializationError("GXI is None. Please check your gxci_init situation.".to_string()))
        }
    }
}

pub fn gxci_init(dll_path: &str) -> Result<(), GxciError> {
    unsafe {
        if GXI.is_some() {
            println!("Warning: GXI is already initialized. Reinitializing.");
        }

        GXI = Some(GXInstance::new(dll_path)?);
        GXI.as_ref().unwrap().gx_init_lib()?;
    }
    Ok(())
}

pub fn gxci_close() -> Result<(), GxciError> {
    unsafe {
        gxi_check(|gxi| gxi.gx_close_lib())?;

        GXI = None;
    }
    Ok(())
}