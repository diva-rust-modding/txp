use pyo3::exceptions::PyException;
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
    id: u32,
    #[pyo3(get, set)]
    pub width: u32,
    #[pyo3(get, set)]
    pub height: u32,
    #[pyo3(get, set)]
    pub format: TextureFormat,
    #[pyo3(get, set)]
    pub data: Vec<u8>,
}

#[pymethods]
impl PyMipmap {
    #[cfg(feature = "image")]
    fn to_rgb(&self) -> Option<Vec<(u8, u8, u8)>> {
        let sub: Mipmap<'_> = self.clone().into();
        sub.to_dynamic_image().map(|x| {
            x.to_rgb()
                .pixels()
                .map(|x| (x.0[0], x.0[1], x.0[2]))
                .collect()
        })
    }
    #[cfg(feature = "image")]
    fn to_rgba(&self) -> Option<Vec<(u8, u8, u8, u8)>> {
        let sub: Mipmap<'_> = self.clone().into();
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

impl<'a> From<PyMap> for Map<'a> {
    fn from(map: PyMap) -> Self {
        if map.sides.len() == 1 {
            let fst = map.sides.get(0).unwrap().clone();
            Map::Texture(fst.into())
        } else {
            Map::Array(TextureArray {
                name: None,
                sides: map
                    .sides
                    .into_iter()
                    .map(|x| x.mipmaps.into_iter().map(Into::into).collect())
                    .collect(),
            })
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
impl<'a> From<PyTexture> for Texture<'a> {
    fn from(tex: PyTexture) -> Self {
        let mipmaps = tex.mipmaps.into_iter().map(Into::into).collect();
        Self {
            name: None,
            mipmaps,
        }
    }
}
impl<'a> From<Mipmap<'a>> for PyMipmap {
    fn from(sub: Mipmap<'a>) -> Self {
        let Mipmap {
            id,
            width,
            height,
            format,
            data,
        } = sub;
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
impl<'a> From<PyMipmap> for Mipmap<'a> {
    fn from(mip: PyMipmap) -> Self {
        let PyMipmap {
            id,
            width,
            height,
            format,
            data,
        } = mip;
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

struct ExternalError<E>(E);

impl<E: std::error::Error> From<ExternalError<E>> for PyErr {
    fn from(err: ExternalError<E>) -> Self {
        PyException::new_err(err.0.to_string())
    }
}

#[pymethods]
impl PyTexture {
    #[cfg(feature = "ddsfile")]
    pub fn to_dds_bytes(&self) -> PyResult<Vec<u8>> {
        let txp: Texture = self.clone().into();
        let dds = txp.to_dds().map_err(ExternalError)?;
        let mut vec = vec![];
        dds.write(&mut vec).map_err(ExternalError)?;
        Ok(vec)
    }

    fn __repr__(&self) -> PyResult<String> {
        let mip = match self.mipmaps.get(0) {
            Some(m) => format!(" {:?} {}x{}", m.format, m.width, m.height),
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
    m.add_class::<TextureFormat>()?;

    Ok(())
}
