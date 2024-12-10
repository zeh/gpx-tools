use structopt::StructOpt;

use gpx_tools::utils::files::get_files_from_masks;

mod gpx_tools;

#[derive(Debug, StructOpt)]
#[structopt(name = "default", about = "The default use of GPX Tools")]
struct Opt {
	/// The input file paths or file paths mask.
	#[structopt(short, long)]
	input: Vec<String>,
}

fn main() {
	let args = Opt::from_args();
	println!("Input: {:?}", args.input);
	let files = get_files_from_masks(&args.input).unwrap();
	println!("Files ({:?}): {:?}", files.len(), files);
}
