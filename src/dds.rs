use ddsfile;
use ddsfile::AlphaMode;
use ddsfile::D3D10ResourceDimension;
use ddsfile::Dds;
use ddsfile::{D3DFormat, DxgiFormat};

use std::convert::TryInto;

use super::*;

impl Texture<'_> {
    pub fn to_d3d(&self) -> Result<Dds, ddsfile::Error> {
        let mipmaps = &self.subtextures[0];
        let first = &mipmaps[0];
        let format = first
            .format
            .to_d3d()
            .ok_or(ddsfile::Error::UnsupportedFormat)?;
        let mips = mipmaps.len().try_into().ok();
        let mut dds = Dds::new_d3d(first.height, first.width, None, format, mips, None)?;
        let data = mipmaps.iter().flat_map(|m| &m.data[..]).copied().collect();
        dds.data = data;
        Ok(dds)
    }
    pub fn to_dxgi(&self) -> Result<Dds, ddsfile::Error> {
        use TextureFormat::*;
        let mipmaps = &self.subtextures[0];
        let first = &mipmaps[0];
        let format = first.format.to_dxgi();
        let alpha = match first.format {
            DXT1 | DXT1a => AlphaMode::PreMultiplied,
            _ => AlphaMode::Straight,
        };
        let mips = mipmaps.len().try_into().ok();
        let mut dds = Dds::new_dxgi(
            first.height,
            first.width,
            None,
            format,
            mips,
            None,
            None,
            false,
            D3D10ResourceDimension::Texture2D,
            alpha,
        )?;
        let data = mipmaps.iter().flat_map(|m| &m.data[..]).copied().collect();
        dds.data = data;
        Ok(dds)
    }
    pub fn to_dds(&self) -> Result<Dds, ddsfile::Error> {
        self.to_d3d().or_else(|_| self.to_dxgi())
    }
}

impl Mipmap<'_> {
    pub fn to_d3d(&self) -> Result<Dds, ddsfile::Error> {
        let format = self
            .format
            .to_d3d()
            .ok_or(ddsfile::Error::UnsupportedFormat)?;
        let mut dds = Dds::new_d3d(self.height, self.width, None, format, None, None)?;
        dds.data = self.data.clone().into_owned();
        Ok(dds)
    }

    pub fn to_dxgi(&self) -> Result<Dds, ddsfile::Error> {
        use TextureFormat::*;
        let format = self.format.to_dxgi();
        let alpha = match self.format {
            DXT1 | DXT1a => AlphaMode::PreMultiplied,
            _ => AlphaMode::Straight,
        };
        let mut dds = Dds::new_dxgi(
            self.height,
            self.width,
            None,
            format,
            None,
            None,
            None,
            false,
            D3D10ResourceDimension::Texture2D,
            alpha,
        )?;
        dds.data = self.data.clone().into_owned();
        Ok(dds)
    }
    pub fn to_dds(&self) -> Result<Dds, ddsfile::Error> {
        self.to_d3d().or_else(|_| self.to_dxgi())
    }
}

impl TextureFormat {
    pub fn to_d3d(&self) -> Option<D3DFormat> {
        use TextureFormat::*;
        Some(match self {
            RGB => D3DFormat::R8G8B8,
            RGBA => D3DFormat::A8R8G8B8,
            L8 => D3DFormat::L8,
            L8A8 => D3DFormat::A8L8,
            DXT1 | DXT1a => D3DFormat::DXT1,
            DXT3 => D3DFormat::DXT3,
            DXT5 => D3DFormat::DXT5,
            _ => return None,
        })
    }

    pub fn to_dxgi(&self) -> DxgiFormat {
        use TextureFormat::*;
        match self {
            RGB => DxgiFormat::R8G8B8A8_UNorm, //TODO: Test if this actually works
            RGBA => DxgiFormat::R8G8B8A8_UNorm,
            RGBA4 => DxgiFormat::B4G4R4A4_UNorm,
            L8 => DxgiFormat::R8_UNorm,
            L8A8 => DxgiFormat::R8G8_UNorm,
            DXT1 => DxgiFormat::BC1_UNorm,
            DXT1a => DxgiFormat::BC1_UNorm,
            DXT3 => DxgiFormat::BC2_UNorm,
            DXT5 => DxgiFormat::BC3_UNorm,
            ATI1 => DxgiFormat::BC4_UNorm,
            ATI2 => DxgiFormat::BC5_UNorm,
        }
    }
}
