实际上这个问题的解决，一是GPT说可以加个packed，然后错位减少了一位

我就想，那么我减小一下Rust这边数组的长度不就行了，-1，出现了256，那看来还是按照4为单位减去会好一些

-8，buffer_size报错说小了，那就-4吧，一下就成了！

如下

```bash
Successfully get enum description.
GX_ENUM_DESCRIPTION { nValue: 0, szSymbolic: [79, 102, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], reserved: [0, 0, 0, 0, 0, 0, 0, 0] }
"Off"
GX_ENUM_DESCRIPTION { nValue: 1, szSymbolic: [67, 111, 110, 116, 105, 110, 117, 111, 117, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], reserved: [0, 0, 0, 0, 0, 0, 0, 0] }
"Continuous"
GX_ENUM_DESCRIPTION { nValue: 2, szSymbolic: [79, 110, 99, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], reserved: [0, 0, 0, 0, 0, 0, 0, 0] }
"Once"
Successfully closed device
PS C:\Users\Chest\Documents\GitHub\crate_zone\gxci> 
```


下面是最开始问题的描述

```bash
我正在用rust写一个C接口的绑定，但是结构体解析方法上出现了错位，如下
Successfully get enum description.
GX_ENUM_DESCRIPTION { nValue: 0, szSymbolic: [79, 102, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], reserved: [0, 0, 0, 0, 0, 0, 0, 1] }
"Off"
GX_ENUM_DESCRIPTION { nValue: 8031446909689163587, szSymbolic: [117, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], reserved: [0, 0, 0, 0, 0, 2, 0, 1701015119] }
"us"
GX_ENUM_DESCRIPTION { nValue: 0, szSymbolic: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], reserved: [0, 0, 0, 0, 0, 0, 0, 0] }
""
Successfully closed device，这里的nValue本应该是0，1，2，但是都错位到后面去了，以及后面的szSymbolic应该一次是Off, Continuous, Once, 但是后两个的前八个char都错位到前面外面去了，C接口文档如下7.3.9. GX_ENUM_DESCRIPTION
Related interface:GXGetEnumDescription
The interface describes the value and description information of all enumerated items.
{ typedef struct GX_ENUM_DESCRIPTION
int64_t nValue;
char szSymbolic[GX_INFO_LENGTH_64_BYTE];
int32_t reserved[8];
}GX_ENUM_DESCRIPTION;
Name
Description
nValue
The value of the enumeration item
szSymbolic
64 bytes, the character description information of the enumeration item
reserved
32 bytes, reserved
7.3.10. GX_REGISTER_STACK_ENTRY7.4.25. GXGetEnumDescription
Declarations:
GX_API GXGetEnumDescription (GX_DEV_HANDLE hDevice,
GX_FEATURE_ID featureID,
GX_ENUM_DESCRIPTION* pEnumDescription,
size_t* pnBufferSize)
Descriptions:
To get the description information of the enumerated type values: the number of enumerated items
and the value and descriptions of each item, please reference GX_ENUM_DESCRIPTION.
Parameters:
[in]hDevice The handle of the device.
[in]featureID The feature code ID.
[out]pEnumDescription The array pointer, used for the enumeration description information
returned.
[in,out]pnBufferSize The size of the GX_ENUM_DESCRIPTION array that the user
introduces, unit: byte.
If pEnumDescription is NULL:
[out]pnBufferSize The actual size of the buffer needed.
If pEnumDescription is not NULL:
[in]pnBufferSize The size of the buffer that the user allocated.
[out]pnBufferSize
Return the actual filled buffer size.，我的Rust结构体是这样定义的#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_ENUM_DESCRIPTION {
    pub nValue: i64,
    pub szSymbolic: [c_char; GX_INFO_LENGTH_64_BYTE],
    pub reserved: [i32; 8],
}，我的方法如下let mut enum_descriptions: Vec<GX_ENUM_DESCRIPTION> =
        vec![GX_ENUM_DESCRIPTION::new(); enum_entry_nums as usize];
    // Obtain a mutable pointer to the array's data
    let enum_descriptions_ptr: *mut GX_ENUM_DESCRIPTION = enum_descriptions.as_mut_ptr();
    let mut buffer_size :usize = enum_entry_nums as usize * core::mem::size_of::<GX_ENUM_DESCRIPTION>() as usize + 1usize;

    let status = gxi_check(|gxi| gxi.gx_get_enum_description(gxi_device, feature_id, enum_descriptions_ptr, &mut buffer_size))?;

```