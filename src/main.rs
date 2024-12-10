use std::io::BufReader;
use std::fs::File;

use structopt::StructOpt;
use gpx_tools::utils::files::get_files_from_masks;
use gpx::read;
use gpx::{Gpx, Track, TrackSegment};
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
	println!("Files: ({:?})", files.len());

	for file in files {
		println!("  File: {:?}", file);
		let data = File::open(file).unwrap();
		let reader = BufReader::new(data);

		let gpx: Gpx = read(reader).unwrap();

		println!("    Creator: {:?}", gpx.creator.unwrap_or("<None>".to_string()));
		println!("    Version: {:?}", gpx.version);

		println!("    Tracks: {:?}", gpx.tracks.len());

		for track in gpx.tracks {
			println!("      Track: {:?}", track.name);
			println!("        Description: {:?}", track.description);
			println!("        Source: {:?}", track.source);
			println!("        Segments: {:?}", track.segments.len());

			for segment in track.segments {
				println!("          Points: {:?}", segment.points.len());
			}
		}

		println!("    Routes: {:?}", gpx.routes.len());

		for route in gpx.routes {
			println!("      Route: {:?}", route.name);
			println!("        Description: {:?}", route.description);
			println!("        Source: {:?}", route.source);
			println!("        Points: {:?}", route.points.len());
		}


		// Each GPX file has multiple "tracks", this takes the first one.
		// let track: &Track = &gpx.tracks[0];
		// assert_eq!(track.name, Some(String::from("Example GPX Document")));

		// Each track will have different segments full of waypoints, where a
		// waypoint contains info like latitude, longitude, and elevation.
		// let segment: &TrackSegment = &track.segments[0];
	}
}
