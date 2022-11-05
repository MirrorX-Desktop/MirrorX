// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//
// Copyright (c) Microsoft Corporation. All rights reserved
//----------------------------------------------------------------------

Texture2D tx : register(t0);
SamplerState samLinear : register(s0);

struct PS_INPUT
{
	float4 Pos : SV_POSITION;
	float2 Tex : TEXCOORD;
};

// Derived from https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
// Section: Converting 8-bit YUV to RGB888

static const float3x1 RGBtoYCoeffVector =
{
	// for BT.601 SDTV color conversion matrix
	// https://mymusing.co/bt601-yuv-to-rgb-conversion-color/
	// Section: Computer RGB To YCbCr
	// 0.256788f, 
	// 0.504129f, 
	// 0.097906f
	
	// for BT.709 HDTV color conversion matrix
	// https://mymusing.co/bt-709-yuv-to-rgb-conversion-color/
	// Section: Computer RGB To YCbCr
	0.182585f, // 0.2126f * 219 / 255,
	0.614230f, //0.7152f,
	0.062007f, //0.0722f,
};

float CalculateY(float3 rgb)
{
	float y = mul(rgb, RGBtoYCoeffVector);
	y += 0.0625f;
	return saturate(y);
}

//--------------------------------------------------------------------------------------
// Pixel Shader
//--------------------------------------------------------------------------------------
float PS_Y(PS_INPUT input) : SV_TARGET
{
	float4 pixel = tx.Sample(samLinear, input.Tex);
	return CalculateY(pixel.xyz);
}