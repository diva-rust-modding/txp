use txp::*;
use structopt::StructOpt;
use anyhow::*;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

use std::fs::File;
use std::io::Read;

use ::image::ImageDecoder;
use ::image::DynamicImage;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut file = File::open(&opt.input)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    let (_, atlas) = TextureAtlas::parse(&data).unwrap();
    let path = opt.input.parent().unwrap().join(opt.input.file_stem().unwrap());
    std::fs::create_dir(&path);
    for (i, map) in atlas.0.into_iter().enumerate() {
        match map {
            Map::Texture(t) => {
                let name = format!("tex{}.dds", i);
                let path = path.join(name);
                let mip = t.mipmaps[0].clone();
                let image = match mip.to_dynamic_image() {
                    Some(i) => i,
                    None => continue
                };
                image.flipv().save(path);
            }
            _ => continue
        }
    }
    Ok(())
}