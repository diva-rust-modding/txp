use anyhow::*;
use structopt::StructOpt;
use txp::*;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    ext: Option<String>,
}

use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut file = File::open(&opt.input)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    let (_, atlas) = TextureAtlas::parse(&data).unwrap();
    let path = opt
        .input
        .parent()
        .unwrap()
        .join(opt.input.file_stem().unwrap());
    std::fs::create_dir(&path);
    let ext = opt.ext.unwrap_or("png".into());
    for (i, map) in atlas.0.into_iter().enumerate() {
        match map {
            Map::Texture(t) => {
                let name = format!("tex{}.{}", i, ext);
                let path = path.join(name);
                if ext == "dds" {
                    let mut save = File::create(path)?;
                    let dds = t.to_dds()?;
                    dds.write(&mut save)?;
                } else {
                    if t.is_yuv() {
                        let image = t.yuv_to_image()?;
                        // image.flipv().save(path);
                        image.save(path);
                    } else {
                        image_extract(t.mipmaps[0].clone(), path);
                    }
                }
            }
            Map::Array(m) => {
                for (j, side) in m.sides.iter().enumerate() {
                    let name = format!("tex{}_side{}.{}", i, j, ext);
                    let path = path.join(name);
                    if ext == "dds" {
                        let mut save = File::create(path)?;
                        let dds = side[0].to_dds()?;
                        dds.write(&mut save)?;
                    } else {
                        image_extract(side[0].clone(), path);
                    }
                }
            }
        }
    }
    Ok(())
}

use std::path::Path;
fn image_extract<Q: AsRef<Path>>(subtex: Mipmap<'_>, path: Q) -> Option<()> {
    let image = subtex.to_dynamic_image()?;
    image.flipv().save(path);
    Some(())
}
