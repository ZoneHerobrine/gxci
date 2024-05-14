#![allow(dead_code)]

// Use simple enums to represent the size and color filter entries.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PixelSizeEntry {
    Bpp8 = 8,
    Bpp10 = 10,
    Bpp12 = 12,
    Bpp16 = 16,
    Bpp24 = 24,
    Bpp30 = 30,
    Bpp32 = 32,
    Bpp36 = 36,
    Bpp48 = 48,
    Bpp64 = 64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PixelColorFilterEntry {
    None = 0,
    BayerRG = 1,
    BayerGB = 2,
    BayerGR = 3,
    BayerBG = 4,
}

// Constants can be associated with the module directly.
pub const PIXEL_MONO: u32 = 0x01000000;
pub const PIXEL_COLOR: u32 = 0x02000000;
pub const PIXEL_8BIT: u32 = 0x00080000;
pub const PIXEL_10BIT: u32 = 0x000A0000;
pub const PIXEL_12BIT: u32 = 0x000C0000;
pub const PIXEL_16BIT: u32 = 0x00100000;
pub const PIXEL_24BIT: u32 = 0x00180000;
pub const PIXEL_30BIT: u32 = 0x001E0000;
pub const PIXEL_32BIT: u32 = 0x00200000;
pub const PIXEL_36BIT: u32 = 0x00240000;
pub const PIXEL_48BIT: u32 = 0x00300000;
pub const PIXEL_64BIT: u32 = 0x00400000;

// Enum for pixel formats, including detailed descriptions as associated constants.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PixelFormatEntry {
    Undefined = 0,
    Mono8 = PIXEL_MONO | PIXEL_8BIT | 0x0001,
    Mono8Signed = PIXEL_MONO | PIXEL_8BIT | 0x0002,
    Mono10 = PIXEL_MONO | PIXEL_16BIT | 0x0003,
    Mono12 = PIXEL_MONO | PIXEL_16BIT | 0x0005,
    Mono14 = PIXEL_MONO | PIXEL_16BIT | 0x0025,
    Mono16 = PIXEL_MONO | PIXEL_16BIT | 0x0007,
    BayerGr8 = PIXEL_MONO | PIXEL_8BIT | 0x0008,
    BayerRg8 = PIXEL_MONO | PIXEL_8BIT | 0x0009,
    BayerGr12 = PIXEL_MONO | PIXEL_16BIT | 0x0010,
    BayerRg12 = PIXEL_MONO | PIXEL_16BIT | 0x0011,
    BayerGb12 = PIXEL_MONO | PIXEL_16BIT | 0x0012,
    BayerBg12 = PIXEL_MONO | PIXEL_16BIT | 0x0013,
    BayerGr16 = PIXEL_MONO | PIXEL_16BIT | 0x002E,
    BayerRg16 = PIXEL_MONO | PIXEL_16BIT | 0x002F,
    BayerGb16 = PIXEL_MONO | PIXEL_16BIT | 0x0030,
    BayerBg16 = PIXEL_MONO | PIXEL_16BIT | 0x0031,

    Rgb8 = PIXEL_COLOR | PIXEL_24BIT | 0x0014,
    Rgb10 = PIXEL_COLOR | PIXEL_48BIT | 0x0018,
    Rgb12 = PIXEL_COLOR | PIXEL_48BIT | 0x001A,
    Rgb14 = PIXEL_COLOR | PIXEL_48BIT | 0x005E,
    Rgb16 = PIXEL_COLOR | PIXEL_48BIT | 0x0033,

    Bgr8 = PIXEL_COLOR | PIXEL_24BIT | 0x0015,
    Bgr10 = PIXEL_COLOR | PIXEL_48BIT | 0x0019,
    Bgr12 = PIXEL_COLOR | PIXEL_48BIT | 0x001B,
    Bgr14 = PIXEL_COLOR | PIXEL_48BIT | 0x004A,
    Bgr16 = PIXEL_COLOR | PIXEL_48BIT | 0x004B,

    Rgba8 = PIXEL_COLOR | PIXEL_32BIT | 0x0016,
    Bgra8 = PIXEL_COLOR | PIXEL_32BIT | 0x0017,
    Argb8 = PIXEL_COLOR | PIXEL_32BIT | 0x0018, // Note, not defined in standard protocol
    Abgr8 = PIXEL_COLOR | PIXEL_32BIT | 0x0019, // Note, not defined in standard protocol

    Yuv444_8 = PIXEL_COLOR | PIXEL_24BIT | 0x0020,
    Yuv422_8 = PIXEL_COLOR | PIXEL_16BIT | 0x0032,
    Yuv411_8 = PIXEL_COLOR | PIXEL_12BIT | 0x001E,
    Yuv420_8Planar = PIXEL_COLOR | PIXEL_12BIT | 0x0040, // Note, not defined in standard protocol

    Ycbcr444_8 = PIXEL_COLOR | PIXEL_24BIT | 0x005B,
    Ycbcr422_8 = PIXEL_COLOR | PIXEL_16BIT | 0x003B,
    Ycbcr411_8 = PIXEL_COLOR | PIXEL_12BIT | 0x005A,

    Ycbcr601_444_8 = PIXEL_COLOR | PIXEL_24BIT | 0x003D,
    Ycbcr601_422_8 = PIXEL_COLOR | PIXEL_16BIT | 0x003E,
    Ycbcr601_411_8 = PIXEL_COLOR | PIXEL_12BIT | 0x003F,

    Ycbcr709_444_8 = PIXEL_COLOR | PIXEL_24BIT | 0x0040,
    Ycbcr709_422_8 = PIXEL_COLOR | PIXEL_16BIT | 0x0041,
    Ycbcr709_411_8 = PIXEL_COLOR | PIXEL_12BIT | 0x0042,

    Mono10Packed = PIXEL_MONO | PIXEL_12BIT | 0x0004, // GigE Vision specific format
    Mono12Packed = PIXEL_MONO | PIXEL_12BIT | 0x0006, // GigE Vision specific format

    // ... Include all other formats similarly.
    Rgb8Planar = PIXEL_COLOR | PIXEL_24BIT | 0x0021,
    Rgb10Planar = PIXEL_COLOR | PIXEL_48BIT | 0x0022,
    // ... More formats as needed.
}
