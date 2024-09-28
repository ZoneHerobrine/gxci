use crate::raw::gx_struct::GX_ENUM_DESCRIPTION;
use crate::raw::gx_struct::GX_FRAME_CALLBACK_PARAM;
use std::slice;

pub fn extract_sz_symbolic(data: GX_ENUM_DESCRIPTION) -> String {
    let symbolic_bytes: Vec<u8> = data.szSymbolic
    .iter()
    .take_while(|&&x| x != 0)
    .map(|&x| x as u8) // Convert i8 to u8
    .collect();

    String::from_utf8_lossy(&symbolic_bytes).to_string()
}

pub fn extract_n_value(data: GX_ENUM_DESCRIPTION) -> i64 {
    data.nValue
}

pub fn extract_img_bug(frame_callback_data: &GX_FRAME_CALLBACK_PARAM) -> &'static [u8] {
    unsafe {
        if frame_callback_data.status == 0 {
            let data = slice::from_raw_parts(frame_callback_data.pImgBuf as *const u8, (frame_callback_data.nWidth * frame_callback_data.nHeight) as usize);
            data
        } else {
            &[]
        }
    }
}

pub fn extract_frame_callback_param(p_frame_callback_data: *mut GX_FRAME_CALLBACK_PARAM) -> &'static GX_FRAME_CALLBACK_PARAM {
    unsafe { &*p_frame_callback_data }
}