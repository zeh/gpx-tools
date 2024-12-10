use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "default", about = "The default use of GPX Tools")]
struct Opt {
	/// The input file paths or file paths mask.
	#[structopt(short, long, parse(from_os_str))]
	input: PathBuf,
}

fn main() {
    let args = Opt::from_args();
    println!("Input: {:?}", args.input)
}
