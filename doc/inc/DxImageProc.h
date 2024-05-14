//--------------------------------------------------------------- 
/** 
\file      DxImageProc.h
\brief     Image Processing Library
\version   v1.0.2305.9291 
\date      2023-03-08
\author    Software Department 
<p>Copyright (c) 2022-2023 and all right reserved.</p> 
*/ 
//--------------------------------------------------------------- 

#if !defined (_DXIMAGEPROC_H)         
#define _DXIMAGEPROC_H 			///< pre-compiled macro define

////////////////////////////////////////////////////////////////////////////////////////////
//  Type definitions. The following types are defined in the standard C library header 
//  stdint.h. This file is not included in the previous version of Microsoft's compilation
//  platform VS 2010, so type definitions need to be redefined here.
////////////////////////////////////////////////////////////////////////////////////////////

#if defined(_WIN32)
    #ifndef _STDINT_H 
        #ifdef _MSC_VER // Microsoft compiler
            #if _MSC_VER < 1600
                typedef __int8            int8_t;
                typedef __int16           int16_t;
                typedef __int32           int32_t;
                typedef __int64           int64_t;
                typedef unsigned __int8   uint8_t;
                typedef unsigned __int16  uint16_t;
                typedef unsigned __int32  uint32_t;
                typedef unsigned __int64  uint64_t;
            #else
                // In Visual Studio 2010 is stdint.h already included
                #include <stdint.h>
            #endif
        #else
            // Not a Microsoft compiler
            #include <stdint.h>
        #endif
    #endif 
#else
    // Linux
    #include <stdint.h>
#endif

//------------------------------------------------------------------------------
//  Operating System Platform Definition
//------------------------------------------------------------------------------

#include <stddef.h>

#ifdef WIN32
    #ifndef _WIN32
        #define _WIN32
    #endif
#endif

#ifdef _WIN32
    #include <Windows.h>
    #define GX_DLLIMPORT   __declspec(dllimport)
    #define GX_DLLEXPORT   __declspec(dllexport)

    #define GX_STDC __stdcall
    #define GX_CDEC __cdecl

    #if defined(__cplusplus)
        #define GX_EXTC extern "C"
    #else
        #define GX_EXTC
    #endif
#else
    // remove the None #define conflicting with GenApi
    #undef None
    #if __GNUC__>=4
        #define GX_DLLIMPORT   __attribute__((visibility("default")))
        #define GX_DLLEXPORT   __attribute__((visibility("default")))

        #if defined(__i386__)
            #define GX_STDC __attribute__((stdcall))
            #define GX_CDEC __attribute__((cdecl))
        #else
            #define GX_STDC 
            #define GX_CDEC 
        #endif

        #if defined(__cplusplus)
            #define GX_EXTC extern "C"
        #else
            #define GX_EXTC
        #endif
    #else
        #error Unknown compiler
    #endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32                   
#define DHDECL __stdcall

#else                          
#include <stdlib.h>
#include <string.h>
#define  DHDECL
#endif

#include "GxPixelFormat.h"


#   define IMAGE_CONVERT_DECLARE_HANDLE(name) \
    struct name##_; typedef struct name##_ *name

IMAGE_CONVERT_DECLARE_HANDLE(DX_IMAGE_FORMAT_CONVERT_HANDLE);

typedef char            VxInt8;     
typedef short           VxInt16;     
typedef int             VxInt32;     
#ifdef _WIN32 
typedef __int64         VxInt64;     
#else
typedef long long       VxInt64;     
#endif
typedef unsigned char   VxUint8;     
typedef unsigned short  VxUint16;    
typedef unsigned int    VxUint32;    

/// status  definition
typedef enum  tagDX_STATUS
{
	DX_OK                         = 0,    
	DX_PARAMETER_INVALID          = -101, 
	DX_PARAMETER_OUT_OF_BOUND     = -102, 
	DX_NOT_ENOUGH_SYSTEM_MEMORY   = -103, 
	DX_NOT_FIND_DEVICE            = -104, 
	DX_STATUS_NOT_SUPPORTED       = -105,
    DX_CPU_NOT_SUPPORT_ACCELERATE = -106,
    DX_PARAMETER_NOT_INITIALIZED  = -107,
	DX_FILE_OPEN_FAIL             = -108,
	DX_FILE_READ_FAIL             = -109
} DX_STATUS;

/// Bayer layout
typedef enum  tagDX_PIXEL_COLOR_FILTER
{
	NONE    = 0,   
	BAYERRG = 1,   
	BAYERGB = 2,   
	BAYERGR = 3,   
	BAYERBG = 4    
} DX_PIXEL_COLOR_FILTER;

/// image interpolation method
typedef enum tagDX_BAYER_CONVERT_TYPE
{
	RAW2RGB_NEIGHBOUR  = 0,   
	RAW2RGB_ADAPTIVE   = 1,   
	RAW2RGB_NEIGHBOUR3 = 2    
} DX_BAYER_CONVERT_TYPE;

/// image valid bit
typedef enum tagDX_VALID_BIT
{ 
	DX_BIT_0_7	    = 0,    ///< bit 0~7
	DX_BIT_1_8	    = 1,    ///< bit 1~8
	DX_BIT_2_9	    = 2,    ///< bit 2~9
	DX_BIT_3_10	    = 3,    ///< bit 3~10
	DX_BIT_4_11	    = 4,    ///< bit 4~11
	DX_BIT_5_12	    = 5,    ///< bit 5~12
	DX_BIT_6_13	    = 6,    ///< bit 6~13
	DX_BIT_7_14     = 7,    ///< bit 7~14
	DX_BIT_8_15     = 8     ///< bit 8~15
} DX_VALID_BIT;

/// image actual bits
typedef enum tagDX_ACTUAL_BITS
{
	DX_ACTUAL_BITS_8  = 8,     ///< 8bit
	DX_ACTUAL_BITS_10 = 10,    ///< 10bit
	DX_ACTUAL_BITS_12 = 12,    ///< 12bit
	DX_ACTUAL_BITS_14 = 14,    ///< 14bit
	DX_ACTUAL_BITS_16 = 16     ///< 16bit
} DX_ACTUAL_BITS;

///  image mirror method
typedef enum DX_IMAGE_MIRROR_MODE
{
	HORIZONTAL_MIRROR = 0,     
	VERTICAL_MIRROR   = 1     
}DX_IMAGE_MIRROR_MODE;

/// RGB channel order
typedef enum DX_RGB_CHANNEL_ORDER
{
	DX_ORDER_RGB = 0,
	DX_ORDER_BGR = 1
}DX_RGB_CHANNEL_ORDER;

/// mono8 image process struct 
typedef  struct  MONO_IMG_PROCESS
{
	bool            bDefectivePixelCorrect;   
	bool            bSharpness;               
	bool            bAccelerate;               
	float           fSharpFactor;              
	VxUint8         *pProLut;                  
	VxUint16        nLutLength;                
	VxUint8         arrReserved[32];           
} MONO_IMG_PROCESS;

/// Raw8 Image process struct 
typedef  struct  COLOR_IMG_PROCESS
{
	bool                   bDefectivePixelCorrect; 
	bool                   bDenoise;              
	bool                   bSharpness;             
	bool                   bAccelerate;            
	VxInt16                *parrCC;               
	VxUint8                nCCBufLength;           
	float                  fSharpFactor;          
	VxUint8                *pProLut;              
	VxUint16               nLutLength;            
    DX_BAYER_CONVERT_TYPE  cvType;                 
	DX_PIXEL_COLOR_FILTER  emLayOut;               
	bool                   bFlip;                  
	VxUint8                arrReserved[32];        
} COLOR_IMG_PROCESS;


/// FLAT FIELD CORRECTION PROCESS struct
typedef struct FLAT_FIELD_CORRECTION_PROCESS
{
	void                        *pBrightBuf;         ///< Bright image buffer
	void                        *pDarkBuf;           ///< Dark image buffer
	VxUint32                     nImgWid;            ///< image width
	VxUint32                     nImgHei;            ///< image height
	DX_ACTUAL_BITS               nActualBits;        ///< image actual bits
	DX_PIXEL_COLOR_FILTER        emBayerType;        ///< Bayer Type
}FLAT_FIELD_CORRECTION_PROCESS; 


/// color transform factor
#ifndef COLOR_TRANSFORM_FACTOR_DEFINED
#define COLOR_TRANSFORM_FACTOR_DEFINED
typedef  struct  COLOR_TRANSFORM_FACTOR
{
    float fGain00;          ///< red    contribution to the red   pixel   (multiplicative factor)
    float fGain01;          ///< green  contribution to the red   pixel   (multiplicative factor)
    float fGain02;          ///< blue  contribution to the red   pixel   (multiplicative factor)

    float fGain10;          ///< red   contribution to the green pixel   (multiplicative factor)
    float fGain11;          ///< green contribution to the green pixel   (multiplicative factor)
    float fGain12;          ///< blue  contribution to the green pixel   (multiplicative factor)

    float fGain20;          ///< red   contribution to the blue  pixel   (multiplicative factor)
    float fGain21;          ///< green contribution to the blue  pixel   (multiplicative factor)
    float fGain22;          ///< blue  contribution to the blue  pixel   (multiplicative factor)
} COLOR_TRANSFORM_FACTOR;
#endif

/// static defect pixel correction struct 
typedef  struct  STATIC_DEFECT_CORRECTION
{
	VxUint32				nImgWidth;   
	VxUint32				nImgHeight;               
	VxUint32				nImgOffsetX;               
	VxUint32				nImgOffsetY;
	VxUint32				nImgWidthMax;
	DX_PIXEL_COLOR_FILTER	nBayerType;
	DX_ACTUAL_BITS			emActualBits;

} STATIC_DEFECT_CORRECTION;
//--------------------------------------------------
/**
\brief  Convert Raw8 to Rgb24
\param  pInputBuffer   	[in] input buffer 
\param  pOutputBuffer   [out]output buffer(new buffer)
\param  nWidth  	    [in] image width
\param  nHeight   	    [in] image height
\param  cvtype          [in] Bayer convert type 
\param  nBayerType      [in] pixel color filter 
\param  bFlip           [in] output image flip or not, true:flip false:not flip

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRaw8toRGB24(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, DX_BAYER_CONVERT_TYPE cvtype, DX_PIXEL_COLOR_FILTER nBayerType, bool bFlip);

//--------------------------------------------------
/**
\brief  Convert Raw8 to Rgb24 with chosen RGB channel order
\param  pInputBuffer   	[in] input buffer 
\param  pOutputBuffer   [out]output buffer(new buffer)
\param  nWidth  	    [in] image width
\param  nHeight   	    [in] image height
\param  cvtype          [in] Bayer convert type 
\param  nBayerType      [in] pixel color filter 
\param  bFlip           [in] output image flip or not, true:flip false:not flip
\param  emChannelOrder	[in] RGB channel order of output image

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRaw8toRGB24Ex(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, DX_BAYER_CONVERT_TYPE cvtype, DX_PIXEL_COLOR_FILTER nBayerType, bool bFlip, DX_RGB_CHANNEL_ORDER emChannelOrder);

//--------------------------------------------------
/**
\brief  Convert Raw12Packed to Raw16
\param  pInputBuffer   	[in] input Buffer 
\param  pOutputBuffer   [out]output Buffer(new buffer)
\param  nWidth          [in] image width
\param  nHeight         [in] image height                      

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRaw12PackedToRaw16(void* pInputBuffer, void* pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight);

//-------------------------------------------------
/**
\brief  Convert Raw10Packed to Raw16
\param  pInputBuffer   	[in] input buffer 
\param  pOutputBuffer   [out]output buffer(new buffer)
\param  nWidth          [in] image width
\param  nHeight         [in] image height 

\return emStatus
*/
//-------------------------------------------------
VxInt32 DHDECL DxRaw10PackedToRaw16(void* pInputBuffer, void* pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight);

//------------------------------------------------
/**
\brief  To rotate the 8-bit image clockwise by 90 degrees
\param  pInputBuffer  	[in] input buffer 
\param  pOutputBuffer	[out]output buffer(new buffer)  
\param  nWidth        	[in] image width
\param  nHeight       	[in] image height 

\return emStatus
*/
//------------------------------------------------
VxInt32 DHDECL DxRotate90CW8B(void* pInputBuffer, void* pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight);

//------------------------------------------------
/**
\brief  To rotate the 8-bit image counter clockwise by 90 degrees
\param  pInputBuffer    [in] input buffer 
\param  pOutputBuffer	[out]output buffer(new buffer) 
\param  nWidth          [in] image width
\param  nHeight         [in] image height 

\return emStatus
*/
//------------------------------------------------
VxInt32 DHDECL DxRotate90CCW8B(void* pInputBuffer, void* pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight);

//-----------------------------------------------
/**
\brief  Brightness adjustment (RGB24 or gray image of 8-bit)
\param  pInputBuffer  	[in] input buffer 
\param  pOutputBuffer	[out]output buffer 
\param  nImagesize    	[in] image size,unit:byte(RGB:width * height * 3)
\param  nFactor        	[in] factor,range(-150~150)

\return emStatus
*/
//-----------------------------------------------
VxInt32 DHDECL DxBrightness(void* pInputBuffer, void* pOutputBuffer, VxUint32 nImagesize, VxInt32 nFactor);

//--------------------------------------------------
/**
\brief  Contrast adjustment(RGB24 or gray image of 8-bit)
\param  pInputBuffer	[in] input buffer      
\param  pOutputBuffer	[out]output buffer
\param  nImagesize      [in] image size,unit:byte(RGB:width * height * 3)
\param  nFactor	        [in] factor,range(-50~100)

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxContrast(void* pInputBuffer, void* pOutputBuffer, VxUint32 nImagesize, VxInt32 nFactor);

//--------------------------------------------------
/**
\brief  Sharpen adjustment (RGB24)
\param  pInputBuffer	[in] input buffer      
\param  pOutputBuffer	[out]output buffer
\param  nWidth          [in] image width
\param  nHeight         [in] image height 
\param  fFactor        	[in] factor,range(0.1~5.0)

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxSharpen24B(void* pInputBuffer, void* pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, float fFactor);

//--------------------------------------------------
/**
\brief  Saturation adjustment (RGB24)
\param  pInputBuffer	[in] input buffer      
\param  pOutputBuffer	[out]output buffer
\param  nImageSize     	[in] image size (width * height)
\param  nFactor        	[in] factor,range(0 ~ 128)

\return emStatus   
*/
//--------------------------------------------------
VxInt32 DHDECL DxSaturation(void* pInputBuffer, void* pOutputBuffer, VxUint32 nImagesize, VxInt32 nFactor);

//--------------------------------------------------
/**
\brief  Get white balance ratios(RGB24), In order to calculate accurately, the camera should shoot objective "white" area,or input image
        is white area.
\param  pInputBuffer   	[in] input buffer
\param  nWidth        	[in] image width
\param  nHeight       	[in] image height
\param  dRatioR         [out]R ratio
\param  dRatioG         [out]G ratio
\param  dRatioB         [out]B ratio

\return emStatus  
*/   
//--------------------------------------------------
VxInt32 DHDECL DxGetWhiteBalanceRatio(void *pInputBuffer, VxUint32 nWidth, VxUint32 nHeight, double* dRatioR, double* dRatioG, double* dRatioB);

//-----------------------------------------------------
/**
\brief  Auto raw defective pixel correct,Support image from Raw8 to Raw16, the bit number is actual bit number, when it is more than 8, the actual bit 
        can be every number between 9 to 16. And if image format is packed, you need convert it to Raw16.This function should be used in each frame.

\param  pRawImgBuf      [in,out]Raw image buffer
\param  nWidth        	[in]image width
\param  nHeight       	[in]image height
\param  nBitNum         [in]image bit number (for example:if image 10bit, nBitNum = 10, if image 12bit,nBitNum = 12,range:8 ~ 16)

\return emStatus  
*/
//-----------------------------------------------------
VxInt32 DHDECL DxAutoRawDefectivePixelCorrect(void* pRawImgBuf, VxUint32 nWidth, VxUint32 nHeight, VxInt32 nBitNum);

//--------------------------------------------------
/**
\brief  Convert Raw16 to Raw8
\param  pInputBuffer   	[in] input buffer(size:width * height *2)
\param  pOutputBuffer   [out]output buffer(new buffer,size:width * height)
\param  nWidth          [in] image width
\param  nHeight         [in] image height
\param  nValidBits      [in] valid bits

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRaw16toRaw8(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, DX_VALID_BIT nValidBits);

//--------------------------------------------------
/**
\brief  Convert RGB48 to RGB24
\param  pInputBuffer   	[in] input buffer(size:width * height * 3 *2)
\param  pOutputBuffer   [out]output buffer(new buffer,size:width * height * 3)
\param  nWidth          [in] image width
\param  nHeight         [in] image height
\param  nValidBits      [in] valid bits 

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRGB48toRGB24(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, DX_VALID_BIT nValidBits);

//--------------------------------------------------
/**
\brief  Convert Raw16 to RGB48
\param  pInputBuffer   	[in] input buffer(size:width * height * 2)
\param  pOutputBuffer   [out]output buffer(new buffer,size:width * height * 3 * 2)
\param  nWidth  	    [in] image width
\param  nHeight   	    [in] image height
\param  nActualBits     [in] image actual bits
\param  cvtype          [in] Bayer convert type 
\param  nBayerType      [in] pixel color filter
\param  bFlip           [in] image flip or not, true:flip false:not flip

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRaw16toRGB48(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, DX_ACTUAL_BITS nActualBits, DX_BAYER_CONVERT_TYPE cvtype, DX_PIXEL_COLOR_FILTER nBayerType, bool bFlip);

//--------------------------------------------------
/**
\brief  Convert Raw8 to ARGB32
\param  pInputBuffer   	[in] input buffer(size:width * height)
\param  pOutputBuffer   [out]output buffer(new buffer,size:width * height * 4 * 2)
\param  nWidth  	    [in] image width
\param  nHeight   	    [in] image height
\param  nStride         [in] Android surface stride
\param  nActualBits     [in] image actual bits
\param  cvtype          [in] Bayer convert type 
\param  nBayerType      [in] pixel color filter
\param  bFlip           [in] image flip or not, true:flip false:not flip
\param  nAlpha			[in] value of channel Alpha

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRaw8toARGB32(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, int nStride, DX_BAYER_CONVERT_TYPE cvtype, DX_PIXEL_COLOR_FILTER nBayerType, bool bFlip, VxUint8 nAlpha);

//--------------------------------------------------
/**
\brief  calculating contrast lookup table (RGB24)
\param  nContrastParam  [in] contrast param,range(-50 ~ 100)
\param  pContrastLut    [out]contrast lookup table
\param  pLutLength      [out]contrast lookup table length(unit:byte)

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxGetContrastLut(int nContrastParam, void *pContrastLut, int *pLutLength);

//--------------------------------------------------
/**
\brief  calculating gamma lookup table (RGB24)
\param  dGammaParam   [in] gamma param,range(0.1 ~ 10) 
\param  pGammaLut     [out]gamma lookup table
\param  pLutLength    [out]gamma lookup table length(unit:byte)

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxGetGammatLut(double dGammaParam, void *pGammaLut, int *pLutLength);

//--------------------------------------------------
/**
\brief  image quality improvement (RGB24)
\param  pInputBuffer   	      [in] input buffer
\param  pOutputBuffer         [out]output buffer
\param  nWidth                [in] image width
\param  nHeight               [in] image height
\param  nColorCorrectionParam [in] color correction param address(get from camera)
\param  pContrastLut          [in] contrast lookup table
\param  pGammaLut             [in] gamma lookup table

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageImprovment(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, VxInt64 nColorCorrectionParam, void *pContrastLut, void *pGammaLut);

//--------------------------------------------------
/**
\brief  image quality improvement (ARGB32)
\param  pInputBuffer   	      [in] input buffer
\param  pOutputBuffer         [out]output buffer
\param  nWidth                [in] image width
\param  nHeight               [in] image height
\param  nColorCorrectionParam [in] color correction param address(get from camera)
\param  pContrastLut          [in] contrast lookup table
\param  pGammaLut             [in] gamma lookup table

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxARGBImageImprovment(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, VxInt64 nColorCorrectionParam, void *pContrastLut, void *pGammaLut);

//--------------------------------------------------
/**
\brief  image quality improvement (RGB24 & BGR24)
\param  pInputBuffer   	      [in] input buffer
\param  pOutputBuffer         [out]output buffer
\param  nWidth                [in] image width
\param  nHeight               [in] image height
\param  nColorCorrectionParam [in] color correction param address(get from camera)
\param  pContrastLut          [in] contrast lookup table
\param  pGammaLut             [in] gamma lookup table
\param  emChannelOrder        [in] RGB channel order of output image

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageImprovmentEx(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, VxInt64 nColorCorrectionParam, void *pContrastLut, void *pGammaLut, DX_RGB_CHANNEL_ORDER emChannelOrder);

//-------------------------------------------------------------
/**
\brief  image mirror(Raw8 or 8bit image) 
\param  pInputBuff   	[in] input buffer
\param  pOutputBuf      [out]output buffer
\param  nWidth          [in] image width
\param  nHeight         [in] image height
\param  emMirrorMode    [in] mirror mode

\return emStatus
*/
//-------------------------------------------------------------
VxInt32 DHDECL DxImageMirror(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, DX_IMAGE_MIRROR_MODE emMirrorMode);

//--------------------------------------------------
/**
\brief  calculating lookup table of 8bit image 
\param  nContrastParam  [in] contrast param,range(-50~100)
\param  dGamma          [in] gamma param,range(0.1~10)
\param  nLightness      [in] lightness param,range(-150~150)
\param  pLut            [out]lookup table
\param  pLutLength      [in] lookup table length(unit:byte)

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxGetLut(VxInt32 nContrastParam, double dGamma, VxInt32 nLightness, VxUint8 *pLut, VxUint16 *pLutLength);

//--------------------------------------------------
/**
\brief  calculating array of image processing color adjustment 
\param  nColorCorrectionParam   [in] color correction param address(get from camera)
\param  nSaturation             [in] saturation factor,Range(0~128)
\param  parrCC                  [out]array address
\param  nLength                 [in] length(sizeof(VxInt16)*9)

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxCalcCCParam(VxInt64 nColorCorrectionParam, VxInt16 nSaturation, VxInt16 *parrCC, VxUint8 nLength);

//--------------------------------------------------
/**
\brief  Raw8 image process 
\param  pRaw8Buf    	      [in] input buffer
\param  pRgb24Buf             [out]output buffer(new buffer)
\param  nWidth                [in] image width
\param  nHeight               [in] image height
\param  pstClrImgProc         [in] Raw8 image process struct pointer

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxRaw8ImgProcess(void *pRaw8Buf, void *pRgb24Buf, VxUint32 nWidth, VxUint32 nHeight, COLOR_IMG_PROCESS *pstClrImgProc);

//--------------------------------------------------
/**
\brief  Mono8 image process 
\param  pInputBuf    	      [in] input buffer
\param  pOutputBuf            [out]output buffer(new buffer)
\param  nWidth                [in] image width
\param  nHeight               [in] image height
\param  pstGrayImgProc        [in] mono8 image process struct pointer

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxMono8ImgProcess(void *pInputBuf, void *pOutputBuf, VxUint32 nWidth, VxUint32  nHeight, MONO_IMG_PROCESS *pstGrayImgProc);

//--------------------------------------------------
/**
\brief  Get Flat Field Correction Coefficients
\param  stFlatFieldCorrection         [in] flat field correction process struct
\param  pFFCCoefficients              [out]flat field correction coefficients
\param  pnlength                      [in]&&[out]flat field correction coefficients length(byte)
\param  pnTargetValue                 [in] correction target Value

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxGetFFCCoefficients(FLAT_FIELD_CORRECTION_PROCESS stFlatFieldCorrection, void *pFFCCoefficients, int *pnlength, int *pnTargetValue = NULL);


//--------------------------------------------------
/**
\brief  Flat Field Correction Process
\param  pInputBuffer    	  [in]        Image in
\param  pOutputBuffer    	  [out]       Image out
\param  nActualBits           [in]        Image actual cits
\param  nImgWidth             [in]        Image width
\param  nImgHeight            [in]        Image height 
\param  pFFCCoefficients      [in]        Flat field correction coefficients
\param  pnlength              [in]        Flat field correction coefficients(byte)

\return emStatus  
*/
//--------------------------------------------------
VxInt32 DHDECL DxFlatFieldCorrection(void *pInputBuffer, void *pOutputBuffer, DX_ACTUAL_BITS nActualBits, VxUint32 nImgWidth, VxUint32  nImgHeight, void  *pFFCCoefficients, int *pnLength);


//--------------------------------------------------
/**
\brief  calculating array of image processing color adjustment 
\param  pstColorTransformFactor [in] color correction param address(user set)
\param  nSaturation             [in] saturation factor,Range(0~128)
\param  parrCC                  [out]array address
\param  nLength                 [in] length(sizeof(VxInt16)*9)

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxCalcUserSetCCParam(COLOR_TRANSFORM_FACTOR *pstColorTransformFactor, VxInt16 nSaturation, VxInt16 *parrCC, VxUint8 nLength);

//--------------------------------------------------
/**
\brief  Image Format Convert Process 

        Supported image format conversion include 

        1.Bayer conversion
         a. input image format  GX_PIXEL_FORMAT_BAYER_GR8 GX_PIXEL_FORMAT_BAYER_RG8 GX_PIXEL_FORMAT_BAYER_GB8 GX_PIXEL_FORMAT_BAYER_BG8
            output image format GX_PIXEL_FORMAT_MONO8 GX_PIXEL_FORMAT_RGB8  GX_PIXEL_FORMAT_RGBA8 GX_PIXEL_FORMAT_BGRA8 
                                GX_PIXEL_FORMAT_ARGB8 GX_PIXEL_FORMAT_ABGR8 GX_PIXEL_FORMAT_RGB8_PLANAR

         b. input image format  GX_PIXEL_FORMAT_BAYER_GR10 GX_PIXEL_FORMAT_BAYER_RG10 GX_PIXEL_FORMAT_BAYER_GB10 GX_PIXEL_FORMAT_BAYER_BG10 
                                GX_PIXEL_FORMAT_BAYER_GR12 GX_PIXEL_FORMAT_BAYER_RG12 GX_PIXEL_FORMAT_BAYER_GB12 GX_PIXEL_FORMAT_BAYER_BG12 
                                GX_PIXEL_FORMAT_BAYER_GR16 GX_PIXEL_FORMAT_BAYER_RG16 GX_PIXEL_FORMAT_BAYER_GB16 GX_PIXEL_FORMAT_BAYER_BG16
            output image format GX_PIXEL_FORMAT_MONO16 GX_PIXEL_FORMAT_RGB16  GX_PIXEL_FORMAT_RGBA16 GX_PIXEL_FORMAT_BGRA16 
                                GX_PIXEL_FORMAT_ARGB16 GX_PIXEL_FORMAT_ABGR16 GX_PIXEL_FORMAT_RGB16_PLANAR

        2.RGB conversion
         a. input image format  GX_PIXEL_FORMAT_RGB8 GX_PIXEL_FORMAT_BGR8
            output image format GX_PIXEL_FORMAT_YUV444_8 GX_PIXEL_FORMAT_YUV422_8 GX_PIXEL_FORMAT_YUV411_8 GX_PIXEL_FORMAT_YUV420_8_PLANAR 
                                GX_PIXEL_FORMAT_YCBCR444_8 GX_PIXEL_FORMAT_YCBCR422_8 GX_PIXEL_FORMAT_YCBCR411_8 GX_PIXEL_FORMAT_MONO8

         b. input image format  GX_PIXEL_FORMAT_RGB16 GX_PIXEL_FORMAT_BGR16
            output image format GX_PIXEL_FORMAT_MONO16

        3.Packed conversion(not support flip)
         a. input image format  GX_PIXEL_FORMAT_MONO12_PACKED GX_PIXEL_FORMAT_MONO10_PACKED
            output image format GX_PIXEL_FORMAT_MONO8

\param  handle          [in]     Image Format convert handle
\param  pInputBuffer    [in]     Image in
\param  nInBufferSize   [in]     Input Image buffer size
\param  pOutputBuffer   [in&out] Image out
\param  nOutBufferSize  [in]     Output Image buffer size
\param  emInPixelFormat [in]     Input Image Pixel Type
\param  nImgWidth       [in]     Image width
\param  nImgHeight      [in]     Image height 
\param  bFlip           [in]     Image flip or not, true:flip false:not flip

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvert(DX_IMAGE_FORMAT_CONVERT_HANDLE handle, void *pInputBuffer, int nInBufferSize, void *pOutputBuffer, int nOutBufferSize, GX_PIXEL_FORMAT_ENTRY emInPixelFormat, VxUint32 nImgWidth, VxUint32  nImgHeight, bool bFlip);

//--------------------------------------------------
/**
\brief  Create handle for Image Format Convert 
\param  phandle          [in] Image Format convert handle

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvertCreate(DX_IMAGE_FORMAT_CONVERT_HANDLE *phandle);

//--------------------------------------------------
/**
\brief  Destroy handle for Image Format Convert  
\param  handle          [in] Image Format convert handle

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvertDestroy(DX_IMAGE_FORMAT_CONVERT_HANDLE handle);

//--------------------------------------------------
/**
\brief  Set Output Pixel type 
\param  handle          [in] Image Format convert handle
\param  emPixelFormat   [in] Pixel Format

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvertSetOutputPixelFormat(DX_IMAGE_FORMAT_CONVERT_HANDLE handle, GX_PIXEL_FORMAT_ENTRY emPixelFormat);

//--------------------------------------------------
/**
\brief  Set Alpha channel value 
\param  handle          [in] Image Format convert handle
\param  nAlphaValue     [in] Alpha channel value(range of 0~255)

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvertSetAlphaValue(DX_IMAGE_FORMAT_CONVERT_HANDLE handle, VxUint8 nAlphaValue);

//--------------------------------------------------
/**
\brief  Set Bayer Pixel Format Convert Type
\param  handle          [in] Image Format convert handle
\param  emCvtType       [in] Bayer Pixel Format convert RGB type

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvertSetInterpolationType(DX_IMAGE_FORMAT_CONVERT_HANDLE handle, DX_BAYER_CONVERT_TYPE emCvtType);

//--------------------------------------------------
/**
\brief  Calculating Buffer size for conversion 
\param  handle          [in]   Image Format convert handle
\param  emPixelFormat   [in]   Pixel Format
\param  nImgWidth       [in]   Image Width
\param  nImgHeight      [in]   Image Height
\param  pBufferSize     [out]  Image buffer size

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvertGetBufferSizeForConversion(DX_IMAGE_FORMAT_CONVERT_HANDLE handle, GX_PIXEL_FORMAT_ENTRY emPixelFormat, VxUint32 nImgWidth, VxUint32  nImgHeight, int *pBufferSize);

//--------------------------------------------------
/**
\brief  Set Output Pixel type 
\param  handle           [in]  Image Format convert handle
\param  pemPixelFormat   [out] Pixel Format

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxImageFormatConvertGetOutputPixelFormat(DX_IMAGE_FORMAT_CONVERT_HANDLE handle, GX_PIXEL_FORMAT_ENTRY *pemPixelFormat);


//-------------------------------------------------------------
/**
\brief  Image defect pixel correction  
\param  pInputBuffer		[in]  Image in
\param  pOutputBuffer		[out] Image out
\param  stDefectCorrection  [in]  Image parameter used to do defect correction
\param  pDefectPosBuffer	[in]  Defect Pixel position file buffer
\param  nDefectPosBufferSize[in]  Defect Pixel position file buffer size

\return  emStatus
*/
//-------------------------------------------------------------
VxInt32 DHDECL DxStaticDefectCorrection(void *pInputBuffer, void *pOutputBuffer, STATIC_DEFECT_CORRECTION stDefectCorrection, void* pDefectPosBuffer, VxUint32 nDefectPosBufferSize);


//--------------------------------------------------
/**
\brief  calculating lookup table of camera 
\param  nContrastParam  [in]     contrast param,range(-50~100)
\param  dGamma          [in]     gamma param,range(0.1~10)
\param  nLightness      [in]     lightness param,range(-150~150)
\param  pLut            [in&out] lookup table
\param  pLutLength      [in]     lookup table length(unit:byte)

Lookup table length should be obtained through the interface GXGetBufferLength.

\return emStatus
*/
//--------------------------------------------------
VxInt32 DHDECL DxCalcCameraLutBuffer(VxInt32 nContrastParam, double dGamma, VxInt32 nLightness, void *pLut, VxUint32 *pnLutLength);


//--------------------------------------------------
/**
\brief  read lut file
\param  pchLutFilePath  [in]     Lut file path. Lut file(xxx.lut) can be obtained from Lut 
                                 Create Tool Plugin,which can be get by select Plugin->Lut 
                                 Create Tool Plugin from the menu bar in GalaxyView.
\param  pLut            [in&out] Lookup table. Users need to apply for memory in advance.The 
                                 memory size is also lookup table length(nLutLength),should be 
                                 obtained through the interface GXGetBufferLength,
                                 e.g. GXGetBufferLength(m_hDevice, GX_BUFFER_LUT_VALUEALL,&nLutLength),                               
\param  pnLutLength     [in]     Lookup table length(unit:byte),which should be obtained through
                                 the interface GXGetBufferLength, e.g. 
                                 GXGetBufferLength(m_hDevice, GX_BUFFER_LUT_VALUEALL,&nLutLength), 

\return   DX_OK                       success
          DX_PARAMETER_INVALID        the input parameter is invalid
          DX_PARAMETER_OUT_OF_BOUND   the input parameter is out of bound
          DX_FILE_OPEN_FAIL           file open failed
          DX_FILE_READ_FAIL           file read failed
          DX_NOT_ENOUGH_SYSTEM_MEMORY system memory is not enough
          DX_STATUS_NOT_SUPPORTED     lookup table length is not supported
*/
//--------------------------------------------------
VxInt32 DHDECL DxReadLutFile(const char *pchLutFilePath, void *pLut, VxUint32 *pnLutLength);

//------------------------------------------------
/**
\brief  To rotate the 16-bit image clockwise by 90 degrees
\param  pInputBuffer  	[in] input buffer 
\param  pOutputBuffer	[out]output buffer(new buffer)  
\param  nWidth        	[in] image width
\param  nHeight       	[in] image height 

\return emStatus
*/
//------------------------------------------------
VxInt32 DHDECL DxRotate90CW16B(void* pInputBuffer, void* pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight);

//------------------------------------------------
/**
\brief  To rotate the 16-bit image counter clockwise by 90 degrees
\param  pInputBuffer    [in] input buffer 
\param  pOutputBuffer	[out]output buffer(new buffer) 
\param  nWidth          [in] image width
\param  nHeight         [in] image height 

\return emStatus
*/
//------------------------------------------------
VxInt32 DHDECL DxRotate90CCW16B(void* pInputBuffer, void* pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight);

//-------------------------------------------------------------
/**
\brief  image mirror(Raw16 or 16bit image) 
\param  pInputBuff   	[in] input buffer
\param  pOutputBuf      [out]output buffer
\param  nWidth          [in] image width
\param  nHeight         [in] image height
\param  emMirrorMode    [in] mirror mode

\return emStatus
*/
//-------------------------------------------------------------
VxInt32 DHDECL DxImageMirror16B(void *pInputBuffer, void *pOutputBuffer, VxUint32 nWidth, VxUint32 nHeight, DX_IMAGE_MIRROR_MODE emMirrorMode);





#ifdef __cplusplus
}
#endif

#endif




