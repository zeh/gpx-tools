use gpx::Gpx;

/// Only return GPX data that have tracks with elevation
pub fn remove_gpxs_without_elevation(gpxs: &Vec<Gpx>) -> Vec<Gpx> {
    gpxs.iter().filter(|g| g.tracks.len() > 0).map(|g| g.clone()).collect::<Vec<Gpx>>()
}
