use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::*;

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyTextureAtlas {
    #[pyo3(get, set)]
    pub maps: Vec<PyMap>,
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyMap {
    #[pyo3(get, set)]
    pub sides: Vec<PyTexture>,
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyTexture {
    #[pyo3(get, set)]
    pub mipmaps: Vec<PyMipmap>,
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyMipmap {
    id: usize,
    #[pyo3(get, set)]
    pub width: u32,
    #[pyo3(get, set)]
    pub height: u32,
    #[pyo3(get, set)]
    pub format: u8,
    #[pyo3(get, set)]
    pub data: Vec<u8>,
}

#[pymethods]
#[cfg(feature = "image")]
impl PyMipmap {
    fn to_rgb(&self) -> Option<Vec<(u8, u8, u8)>> {
        let sub: SubTexture<'_> = self.clone().into();
        sub.to_dynamic_image().map(|x| {
            x.to_rgb()
                .pixels()
                .map(|x| (x.0[0], x.0[1], x.0[2]))
                .collect()
        })
    }
    fn to_rgba(&self) -> Option<Vec<(u8, u8, u8, u8)>> {
        let sub: SubTexture<'_> = self.clone().into();
        sub.to_dynamic_image().map(|x| {
            x.to_rgba()
                .pixels()
                .map(|x| (x.0[0], x.0[1], x.0[2], x.0[3]))
                .collect()
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        let format = TextureFormat::from_id(self.format as u32);
        Ok(format!(
            "PyMipMap: {:?} {}x{} ({} bytes)",
            format,
            self.width,
            self.height,
            self.data.len()
        ))
    }
}

impl<'a> From<TextureAtlas<'a>> for PyTextureAtlas {
    fn from(atlas: TextureAtlas<'a>) -> Self {
        let maps = atlas.0.into_iter().map(Into::into).collect();
        Self { maps }
    }
}

impl<'a> From<Map<'a>> for PyMap {
    fn from(map: Map<'a>) -> Self {
        match map {
            Map::Texture(t) => Self {
                sides: vec![t.into()],
            },
            Map::Array(a) => a.into(),
        }
    }
}

impl<'a> From<TextureArray<'a>> for PyMap {
    fn from(arr: TextureArray<'a>) -> Self {
        let sides = arr
            .sides
            .into_iter()
            .map(|mip| PyTexture {
                mipmaps: mip.into_iter().map(Into::into).collect(),
            })
            .collect();
        Self { sides }
    }
}
impl<'a> From<Texture<'a>> for PyTexture {
    fn from(tex: Texture<'a>) -> Self {
        let mipmaps = tex.mipmaps.into_iter().map(Into::into).collect();
        Self { mipmaps }
    }
}
impl<'a> From<SubTexture<'a>> for PyMipmap {
    fn from(sub: SubTexture<'a>) -> Self {
        let SubTexture {
            id,
            width,
            height,
            format,
            data,
        } = sub;
        let format = format as u8;
        let data = data.into_owned();
        Self {
            id,
            width,
            height,
            format,
            data,
        }
    }
}
impl<'a> From<PyMipmap> for SubTexture<'a> {
    fn from(mip: PyMipmap) -> Self {
        let PyMipmap {
            id,
            width,
            height,
            format,
            data,
        } = mip;
        //TODO: deal with the error variant properly
        let format = TextureFormat::from_id(format as u32).unwrap();
        let data = data.into();
        Self {
            id,
            width,
            height,
            format,
            data,
        }
    }
}

#[pymethods]
impl PyTextureAtlas {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("PyTextureAtlas: {} map(s)", self.maps.len()))
    }
}

#[pymethods]
impl PyMap {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("PyMap: {} side(s)", self.sides.len()))
    }
}

#[pymethods]
impl PyTexture {
    fn __repr__(&self) -> PyResult<String> {
        let mip = match self.mipmaps.get(0) {
            Some(m) => format!(
                " {:?} {}x{}",
                TextureFormat::from_id(m.format as u32),
                m.width,
                m.height
            ),
            None => "".to_string(),
        };
        Ok(format!(
            "PyTexture: {} mipmap(s){}",
            self.mipmaps.len(),
            mip
        ))
    }
}

#[pyfunction]
fn read(path: String) -> PyResult<PyTextureAtlas> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(path)?;
    let mut input = vec![];
    file.read_to_end(&mut input)?;
    let (_, txp) = TextureAtlas::parse(&input).unwrap();
    Ok(txp.into())
}

#[pymodule]
fn txp(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(self::read))?;
    m.add_class::<PyTextureAtlas>()?;
    m.add_class::<PyMap>()?;
    m.add_class::<PyTexture>()?;
    m.add_class::<PyMipmap>()?;

    Ok(())
}
