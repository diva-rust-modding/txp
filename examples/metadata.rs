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

use std::io::Write;

use tabwriter::TabWriter;


fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut file = File::open(opt.input)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    let (_, atlas) = TextureAtlas::parse(&data).unwrap();
    for map in atlas.0 {
        match map {
            Map::Texture(t) => {
                println!("Texture w/ {} mip(s)", t.mipmaps.len());
                print_mips(&t.mipmaps);
            }
            Map::Array(t) => {
                println!("Texture Array w/ {} side(s)", t.sides.len());
                for (i, side) in t.sides.iter().enumerate() {
                    println!("Side {} has {} mip(s)", i+1, side.len());
                    print_mips(&side);
                }
            }
        }
    }
    Ok(())
}

fn print_mips(mips: &[SubTexture<'_>]) -> Result<()> {
    let mut tw = TabWriter::new(vec![]);
    for (i, mip) in mips.iter().enumerate() {
        // println!("\t{}", mip);
        write!(tw, "\t#{}\t{}x{}\t{:?}\n", i+1, mip.width, mip.height, mip.format)?;
    }
    println!("{}", String::from_utf8(tw.into_inner()?)?);
    Ok(())
}