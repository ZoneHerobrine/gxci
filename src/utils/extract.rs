use crate::raw::gx_struct::GX_ENUM_DESCRIPTION;

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