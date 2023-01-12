use std::borrow::Cow;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "ddsfile")]
pub mod dds;
#[cfg(feature = "image")]
pub mod image;
mod r#impl;
#[cfg(feature = "pyo3")]
pub mod py_ffi;
mod read;
#[cfg(feature = "dcv-color-primitives")]
pub mod yuv;

#[derive(Debug, PartialEq, Clone)]
pub struct TextureAtlas<'a>(pub Vec<Map<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub enum Map<'a> {
    Texture(Texture<'a>),
    Array(TextureArray<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Texture<'a> {
    pub mipmaps: Vec<SubTexture<'a>>,
    pub name: Option<Cow<'a, str>>,
}

type Sides<'a> = Vec<SubTexture<'a>>;
#[derive(Debug, PartialEq, Clone)]
pub struct TextureArray<'a> {
    pub sides: Vec<Sides<'a>>,
    pub name: Option<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SubTexture<'a> {
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
