use ddsfile;
use ddsfile::AlphaMode;
use ddsfile::D3D10ResourceDimension;
use ddsfile::Dds;
use ddsfile::{D3DFormat, DxgiFormat};

use std::convert::TryInto;

use super::*;

impl Texture<'_> {
    fn caps2() -> ddsfile::Caps2 {
        use ddsfile::Caps2;
        let mut caps = Caps2::all();
        caps.remove(Caps2::VOLUME);
        caps
    }
    fn d3d(&self) -> Result<Dds, ddsfile::Error> {
        let def = Default::default();
        let first = self
            .subtextures
            .get(0)
            .and_then(|x| x.get(0))
            .unwrap_or(&def);
        let format = first
            .format
            .to_d3d()
            .ok_or(ddsfile::Error::UnsupportedFormat)?;
        let mips = self
            .subtextures
            .get(0)
            .and_then(|x| x.len().try_into().ok());
        let sides = self.subtextures.len().try_into().ok().filter(|&x| x > 1);
        let caps2 = Some(Self::caps2()).filter(|_| sides == Some(6));
        dbg!(
            first.height,
            first.width,
            sides,
            first.format,
            format,
            mips,
            caps2
        );
        Dds::new_d3d(first.height, first.width, sides, format, mips, caps2)
    }
    pub fn dxgi(&self) -> Result<Dds, ddsfile::Error> {
        use TextureFormat::*;
        let def = Default::default();
        let first = self
            .subtextures
            .get(0)
            .and_then(|x| x.get(0))
            .unwrap_or(&def);
        let format = first.format.to_dxgi();
        let alpha = match first.format {
            DXT1 | DXT1a => AlphaMode::PreMultiplied,
            _ => AlphaMode::Straight,
        };
        let mips = self.subtextures.get(0).map(|x| x.len() as u32);
        let sides = self.subtextures.len().try_into().ok().filter(|&x| x > 1);
        let caps2 = Some(Self::caps2()).filter(|_| sides == Some(6));
        dbg!(
            first.height,
            first.width,
            first.format,
            format,
            mips,
            sides,
            caps2,
            self.subtextures.len() == 6,
            alpha,
        );
        Dds::new_dxgi(
            first.height,
            first.width,
            None,
            format,
            mips,
            sides,
            caps2,
            self.subtextures.len() == 6,
            D3D10ResourceDimension::Texture2D,
            alpha,
        )
    }
    pub fn to_dds(&self) -> Result<Dds, ddsfile::Error> {
        let dds = self.d3d().or_else(|_| self.dxgi());
        dds.map(|mut x| {
            x.data = self
                .subtextures
                .iter()
                .flat_map(|x| x.iter().flat_map(|x| &x.data[..]))
                .cloned()
                .collect();
            x
        })
    }
}

impl TextureFormat {
    pub fn to_d3d(&self) -> Option<D3DFormat> {
        use TextureFormat::*;
        match self {
            A8 => Some(D3DFormat::A8),
            RGB8 => Some(D3DFormat::R8G8B8),
            RGBA8 => Some(D3DFormat::A8R8G8B8),
            RGB5 => Some(D3DFormat::R5G6B5),
            RGB5A1 => Some(D3DFormat::A1R5G5B5),
            RGBA4 => Some(D3DFormat::A4R4G4B4),
            DXT1 => Some(D3DFormat::DXT1),
            DXT1a => Some(D3DFormat::DXT1),
            DXT3 => Some(D3DFormat::DXT3),
            DXT5 => Some(D3DFormat::DXT5),
            L8 => Some(D3DFormat::L8),
            L8A8 => Some(D3DFormat::A8L8),
            _ => None,
        }
    }

    pub fn to_dxgi(&self) -> DxgiFormat {
        use TextureFormat::*;
        match self {
            A8 => DxgiFormat::A8_UNorm,
            RGB8 | RGBA8 => DxgiFormat::R8G8B8A8_UNorm,
            RGB5 => DxgiFormat::B5G6R5_UNorm,
            RGB5A1 => DxgiFormat::B5G5R5A1_UNorm,
            RGBA4 => DxgiFormat::B4G4R4A4_UNorm,
            DXT1 | DXT1a => DxgiFormat::BC1_UNorm,
            DXT3 => DxgiFormat::BC2_UNorm,
            DXT5 => DxgiFormat::BC3_UNorm,
            ATI1 => DxgiFormat::BC4_UNorm,
            ATI2 => DxgiFormat::BC5_UNorm,
            L8 => DxgiFormat::A8_UNorm,
            L8A8 => DxgiFormat::A8P8,
        }
    }
}
