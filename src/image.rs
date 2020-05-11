use ::image::*;
use ::image::dxt::{DxtDecoder, DXTVariant};

use super::*;

impl<'a> SubTexture<'a> {
    fn to_dxt_decoder(&self) -> Option<Result<DxtDecoder<&[u8]>, ImageError>> {
        use TextureFormat::*;
        let format = match self.format {
            DXT1 |
            DXT1a => Some(DXTVariant::DXT1),
            DXT3 => Some(DXTVariant::DXT3),
            DXT5 => Some(DXTVariant::DXT5),
            _ => return None
        }?;
        Some(DxtDecoder::new(&self.data, self.width, self.height, format))
    }
}