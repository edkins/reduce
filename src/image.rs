use std::fs::File;
use std::mem::size_of;

use image::{ColorType,ImageDecoder};
use image::jpeg::JPEGDecoder;
use image::png::PNGDecoder;
use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::HDC;
use winapi::um::wingdi::{SetDIBitsToDevice,BITMAPINFO,BITMAPINFOHEADER,RGBQUAD,BI_RGB,DIB_RGB_COLORS};
use winapi::um::winnt::LONG;
use winapi::ctypes::c_void;

use crate::error::Er;

pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[derive(Debug)]
pub struct Image {
    width: u64,
    height: u64,
    stride: u64,
    data: Vec<u8>
}

impl Image {
    pub fn new(width: u64, height: u64) -> Self {
        let stride = 3 * width;
        let data = vec![128;(stride*height) as usize];
        Image{width,height,stride,data}
    }
    fn new_from_data(width: u64, height: u64, color_type: ColorType, data: Vec<u8>) -> Result<Self,Er> {
        println!("{:?}", color_type);
        let stride = 3 * width as usize;
        match color_type {
            ColorType::RGBA(8) => {
                let dstride = 4 * width as usize;
                let mut rgb = vec![0; stride * height as usize];
                for y in 0..height as usize {
                    for x in 0..width as usize {
                        rgb[stride*(height as usize-y-1)+3*x+2] = data[dstride*y+4*x];
                        rgb[stride*(height as usize-y-1)+3*x+1] = data[dstride*y+4*x+1];
                        rgb[stride*(height as usize-y-1)+3*x+0] = data[dstride*y+4*x+2];
                    }
                }
                Ok(Image{
                    width,
                    height,
                    stride: stride as u64,
                    data: rgb
                })
            }
            _ => {
                Err(Er::UnknownColourFormat(color_type))
            }
        }
    }
    pub fn load(filename: &str) -> Result<Self,Er> {
        let file = File::open(filename)?;
        let dims;
        let data;
        let color_type;
        if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
            let decoder = JPEGDecoder::new(file)?;
            dims = decoder.dimensions();
            color_type = decoder.colortype();
            data = decoder.read_image()?;
        } else if filename.ends_with(".png") {
            let decoder = PNGDecoder::new(file)?;
            dims = decoder.dimensions();
            color_type = decoder.colortype();
            data = decoder.read_image()?;
        } else {
            return Err(Er::UnknownImageFormat(filename.to_string()))
        }
        Image::new_from_data(dims.0, dims.1, color_type, data)
    }
    pub fn paint_to_dc(&self, hdc: HDC) {
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER{
                biSize: size_of::<BITMAPINFOHEADER>() as DWORD,
                biWidth: self.width as LONG,
                biHeight: self.height as LONG,
                biPlanes: 1,
                biBitCount: 24,
                biCompression: BI_RGB,
                biSizeImage: self.data.len() as DWORD,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0
            },
            bmiColors: [RGBQUAD{
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0
            }]
        };
        unsafe {
            SetDIBitsToDevice(
                hdc,
                0,
                0,
                self.width as DWORD,
                self.height as DWORD,
                0,
                0,
                0,
                self.height as DWORD,
                self.data.as_ptr() as *const c_void,
                &bmi,
                DIB_RGB_COLORS);
        }
    }
}
