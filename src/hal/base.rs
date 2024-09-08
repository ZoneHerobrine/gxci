use crate::raw::gx_interface::*;
use std::sync::{LazyLock,Arc,Mutex};

pub static GXI: LazyLock<Arc<Mutex<Option<GXInstance>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(None))
});

pub fn gxi_check<T, F>(func: F) -> Result<T>
where
    F: FnOnce(&GXInstance) -> Result<T>,
{
    let gxi = GXI.lock().unwrap();
    if let Some(gxi) = gxi.as_ref() {
        func(gxi)
    } else {
        Err(Error::new(ErrorKind::GxciError(GxciError::InitializationError("GXI is None. Please check your gxci_init situation.".to_string()))))
    }
}

pub fn gxci_init(dll_path: &str) -> Result<()> {
    gxi_check(|gxi| gxi.gx_init_lib())?;
    
    let mut gxi = GXI.lock().unwrap();
    if gxi.is_some() {
        println!("Warning: GXI is already initialized. Reinitializing.");
    }

    // this is unsafe in raw
    *gxi = Some(GXInstance::new(dll_path)?);
    gxi.as_ref().unwrap().gx_init_lib()?;
    
    Ok(())
}

pub fn gxci_close() -> Result<()> {
    gxi_check(|gxi| gxi.gx_close_lib())?;

    let mut gxi = GXI.lock().unwrap();
    *gxi = None;
    Ok(())
}
