use gxci::utils::matching::match_feature_type;

fn main() {
    let feature_id = gxci::raw::gx_enum::GX_FEATURE_ID::GX_STRING_DEVICE_VENDOR_NAME;
    let feature_type = match_feature_type(feature_id);
    println!("{:?}", feature_type);
}