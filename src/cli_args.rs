extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliArgs{
    /// Name of textfile being loaded and displayed
    #[structopt(short = "t", long="textfile", parse(from_os_str))]
    pub text_file: Option<PathBuf>,

    #[structopt(short = "f", long="fontsize")]
    pub font_size: Option<i32>,
}

impl CliArgs {
    pub fn new() -> CliArgs {
        let mut args = CliArgs::from_args();
        if args.font_size == None {
            args.font_size = Some(64);
        }
        args
    }
}
