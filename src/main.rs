use std::path::PathBuf;

use structopt::StructOpt;
use gpx_tools::utils::files::{get_files_from_masks, read_gpx_from_file};

mod gpx_tools;

#[derive(Debug, StructOpt)]
#[structopt(name = "default", about = "The default use of GPX Tools")]
enum Opt {
	Elevation {
		/// The input file for the original route.
		#[structopt(short, long)]
		route: PathBuf,

		/// The input file paths or file paths mask for elevation files.
		#[structopt(short, long)]
		elevation: Vec<String>,
	}
}

fn main() {
	let args = Opt::from_args();
	match args {
		Opt::Elevation{ route, elevation } => {
			println!("Input: {:?}", route);
			let files = get_files_from_masks(&elevation).unwrap();
			println!("Files: ({:?})", files.len());

			for file in files {
				println!("  File: {:?}", file);
				let gpx = read_gpx_from_file(&file);

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
	}
}
