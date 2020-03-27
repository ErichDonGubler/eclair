mod eclipse_binary;
mod eclipse_summary;
mod errors;

use std::{fs::File, io::prelude::*, path::PathBuf};

use anyhow as ah;
use rmp_serde as rmps;
use structopt::StructOpt;

use crate::eclipse_binary::EclBinaryFile;
use crate::eclipse_summary::EclSummary;
use crate::errors::EclError;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "eclair",
    about = "A converter of Eclipse summary files to MessagePack."
)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Optional output file
    #[structopt(parse(from_os_str), short, long)]
    output: Option<PathBuf>,

    /// Debug
    #[structopt(short, long)]
    debug: bool,
}

fn main() -> ah::Result<()> {
    let opt = Opt::from_args();

    let input_path = opt.input;

    // If there is no stem, bail early
    if input_path.file_stem().is_none() {
        return Err(EclError::InvalidFilePath.into());
    }

    // we allow either extension or no extension at all
    if let Some(ext) = input_path.extension() {
        let ext = ext.to_str();
        if ext != Some("SMSPEC") && ext != Some("UNSMRY") {
            return Err(EclError::InvalidFileExt.into());
        }
    }

    let smspec = EclBinaryFile::new(input_path.with_extension("SMSPEC"))?;
    let unsmry = EclBinaryFile::new(input_path.with_extension("UNSMRY"))?;

    let summary = EclSummary::new(smspec, unsmry, opt.debug);

    // serialize summary data in the MessagePack format
    let res = rmps::to_vec(&summary)?;

    let mut out_file = match opt.output {
        Some(p) => File::create(p)?,
        None => File::create(input_path.with_extension("mpk"))?,
    };

    out_file.write_all(&res)?;

    Ok(())
}
