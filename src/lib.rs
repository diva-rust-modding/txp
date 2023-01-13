use std::borrow::Cow;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "ddsfile")]
mod dds;
#[cfg(feature = "image")]
mod image;
mod r#impl;
#[cfg(feature = "pyo3")]
pub mod py_ffi;
mod read;
#[cfg(feature = "dcv-color-primitives")]
mod yuv;

#[derive(Debug, PartialEq, Clone)]
pub struct TextureAtlas<'a>(pub Vec<Texture<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct Texture<'a> {
    pub subtextures: Vec<Vec<Mipmap<'a>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mipmap<'a> {
    id: u32,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Cow<'a, [u8]>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u32)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub enum TextureFormat {
    RGB = 1,
    RGBA,
    RGBA4, //Theoretical Formats, (Only observed in EBOOT)
    L8,    //Theoretical Formats, (Only observed in EBOOT)
    L8A8,  //Theoretical Formats, (Only observed in EBOOT)
    DXT1,
    DXT1a, //Theoretical Formats, (Only observed in EBOOT)
    DXT3,
    DXT5,
    ATI1, //Theoretical Formats, (Only observed in EBOOT)
    ATI2,
}
