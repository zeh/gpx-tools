use std::path::PathBuf;

use structopt::StructOpt;

use gpx_tools::commands::elevation;
use gpx_tools::filters::remove_gpxs_without_elevation;
use gpx_tools::utils::files::{get_files_from_masks, read_gpx_from_file, read_gpx_from_files};

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
			println!("ELEVATION");
			println!("=========");

			let route_gpx = read_gpx_from_file(&route);

			println!("Using route file: {:?}", route);
			println!("- Created by {:?}", route_gpx.creator.unwrap_or("Unknown".to_string()));
			println!("- This file has {:?} track(s) and {:?} route(s)", route_gpx.tracks.len(), route_gpx.routes.len());

			assert!(route_gpx.tracks.len() == 1, "Route can only contain one track");

			for track in &route_gpx.tracks {
				println!("  - Track: {:?}", track.name.clone().unwrap_or("Unknown".to_owned()));
				println!("  - This track has {:?} segment(s)", track.segments.len());

				assert!(route_gpx.tracks[0].segments.len() == 1, "Track can only contain one segment");

				for segment in &track.segments {
					println!("    - This segment has {:?} point(s)", segment.points.len());
					println!("    - Points with elevation: {}",
						if segment.points.iter().all(|p| p.elevation.is_some()) {
							"All"
						} else {
							if segment.points.iter().any(|p| p.elevation.is_some()) {
								"Some"
							} else {
								"None"
							}
						}
					);
				}
			}

			println!("Using elevation mask(s): {:?}", elevation);

			let elevation_files = get_files_from_masks(&elevation).unwrap();
			println!("  - Using {:?} elevation files", elevation_files.len());

			let elevation_gpxs = read_gpx_from_files(&elevation_files);
			let num_gpx = elevation_gpxs.len();
			println!("  - Parsed {:?} GPX files", num_gpx);

			let elevation_gpxs = remove_gpxs_without_elevation(&elevation_gpxs);
			println!("  - Removed {:?} files due to lack of tracks with elevation", elevation_gpxs.len() - num_gpx);
			let num_gpx = elevation_gpxs.len();

			println!("  - Using {:?} GPX files", num_gpx);

			/*
			println!("Input: {:?}", route);

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
			*/
		}
	}
}
