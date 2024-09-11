use crate::raw::gx_enum::*;

// 我觉得可以写个这样的函数，你传入一个GX_FEATURE_ID，然后返回一个对应的GX_FEATURE_TYPE
// 这个TYPE可以通过解析字符串来得到，例如，你传入的是GX_FEATURE_ID::GX_STRING_DEVICE_VENDOR_NAME
// 那么显然，你拿到的GX_FEATURE_ID::GX_后面的第一个STRING就是你要的TYPE，然后到下一个_为止
// 这样就能避免那个一一对应match导致体积膨涨一倍的唐氏实现了
// 最后用example试了一下，其实直接第一个GX_后面的就是，因为format之后只有子enum的名字
pub fn match_feature_type(feature_id: GX_FEATURE_ID) -> GX_FEATURE_TYPE {
    let feature_id_str = format!("{:?}", feature_id);
    println!("{:?}", feature_id_str);
    let feature_type_str = feature_id_str.split("_").collect::<Vec<&str>>()[1];
    println!("{:?}", feature_type_str);
    match feature_type_str {
        "INT" => GX_FEATURE_TYPE::GX_FEATURE_INT,
        "FLOAT" => GX_FEATURE_TYPE::GX_FEATURE_FLOAT,
        "ENUM" => GX_FEATURE_TYPE::GX_FEATURE_ENUM,
        "BOOL" => GX_FEATURE_TYPE::GX_FEATURE_BOOL,
        "STRING" => GX_FEATURE_TYPE::GX_FEATURE_STRING,
        "BUFFER" => GX_FEATURE_TYPE::GX_FEATURE_BUFFER,
        "COMMAND" => GX_FEATURE_TYPE::GX_FEATURE_COMMAND,
        _ => panic!("Invalid feature type"),
    }
}
