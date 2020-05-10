use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_opt;
use nom::multi::length_data;
        use nom::combinator::map;
use nom_ext::*;

use super::*;

fn parse_magic(id: u8) -> impl Fn(&[u8]) -> IResult<&[u8], nom::number::Endianness> {
    use nom::number::Endianness::*;
    move |i: &[u8]| {
        let (i, res) = alt((tag(&[0x54, 0x58, 0x50, id]), tag(&[id, 0x50, 0x58, 0x54])))(i)?;
        Ok((i, if res[3] == id { Little } else { Big }))
    }
}

impl<'a> TextureAtlas<'a> {
    pub fn parse(i0: &'a [u8]) -> IResult<&'a [u8], TextureAtlas<'a>> { 
        let (i, endian) = parse_magic(3)(i0)?;
        let (i, map_count) = u32_usize(endian)(i)?;
        let (i, _) = u32(endian)(i)?;
        let parse =  alt((map(Texture::parse, Map::Texture), map(TextureArray::parse, Map::Array)));
        let (_, maps) = offset_table(i0, parse, map_count, endian)(i)?;
        Ok((i, Self(maps)))
    }
}

impl<'a> TextureArray<'a> {
    pub fn parse(i0: &'a [u8]) -> IResult<&'a [u8], TextureArray<'a>> { 
        use nom::multi::count;
        let (i, endian) = parse_magic(5)(i0)?;
        let (i, total_mip_count) = u32_usize(endian)(i)?;
        let (i, mipdata) = u32_usize(endian)(i)?;
        let depth = (mipdata & 0xFF00) >> 8;
        let mip_count = total_mip_count / depth;
        let (_, sides) = count(offset_table(i0, SubTexture::parse, mip_count, endian), depth)(i)?;
        //let sides = sides.into_iter().map(|mipmaps| Side { mipmaps }).collect();
        Ok((i, Self { sides, name: None }))
    }
}

impl<'a> Texture<'a> {
    pub fn parse(i0: &'a [u8]) -> IResult<&'a [u8], Texture<'a>> { 
        let (i, endian) = parse_magic(4)(i0)?;
        let (i, mip_count) = u32_usize(endian)(i)?;
        let (i, _) = u32(endian)(i)?;
        let (_, mipmaps) = offset_table(i0, SubTexture::parse, mip_count, endian)(i)?;
        Ok((i, Self{mipmaps, name: None}))
    }
}

impl<'a> SubTexture<'a> {
    pub fn parse(i: &'a [u8]) -> IResult<&'a [u8], SubTexture<'a>> {
        let (i, endian) = parse_magic(2)(i)?;
        let (i, width) = u32(endian)(i)?;
        let (i, height) = u32(endian)(i)?;
        let (i, format) = map_opt(u32(endian), TextureFormat::from_id)(i)?;
        let (i, id) = u32_usize(endian)(i)?;
        let (i, data) = length_data(u32(endian))(i)?;
        let data = data.into();
        Ok((
            i,
            Self {
                width,
                height,
                format,
                id,
                data,
            },
        ))
    }
}

impl TextureFormat {
    fn from_id(id: u32) -> Option<Self> {
        use super::TextureFormat::*;
        match id {
            1 => Some(RGB),
            2 => Some(RGBA),
            3 => Some(RGBA4),
            4 => Some(L8),
            5 => Some(L8A8),
            6 => Some(DXT1),
            7 => Some(DXT1a),
            8 => Some(DXT3),
            9 => Some(DXT5),
            10 => Some(ATI1),
            11 => Some(ATI2),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = include_bytes!("../assets/mikitm001_tex.txp");
    const TEX_OFF: usize = 84;
    const MIP_OFF: usize = 100;

    #[test]
    fn read_subtexture() {
        let input = &INPUT[MIP_OFF..];
        let (_, mip) = SubTexture::parse(input).unwrap();
        assert_eq!(mip.id, 0);
        assert_eq!(mip.width, 256);
        assert_eq!(mip.height, 8);
        assert_eq!(mip.format, TextureFormat::RGB);
    }

    #[test]
    fn read_texture() {
        let input = &INPUT[TEX_OFF..];
        let (_, tex) = Texture::parse(input).unwrap();
        println!("{:?}",tex);
        assert_eq!(tex.mipmaps.len(), 1);
    }

    #[test]
    fn read_atlas() {
        let (_, atlas) = TextureAtlas::parse(INPUT).unwrap();
    }
}
