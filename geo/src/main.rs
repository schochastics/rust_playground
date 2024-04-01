use csv;
use geo::algorithm::{Centroid, Area};
use geo::geometry::{Point, Geometry};
use proj::Transform;
use wkt;

let mut feature_reader = {
  use std::fs::File;
  let file = File::open("src/data/philly_waterways/philly_waterways.csv").expect("file path must be valid");
  csv::Reader::from_reader(file)
};

let mut acceptable_walkabout_bridges: Vec<Point> = vec![];

for row in feature_reader.records() {
  let creek_segment = row.expect("must be able to read row from CSV");

  let creek_name = creek_segment.get(0).expect("'creek_name' field must be present");
  let infrastructure_label = creek_segment.get(1).expect("'inf1' field must be present");
  let geometry_str = creek_segment.get(2).expect("`geometry` field must be present");

  // We're only interested in Bridged segments.
  if infrastructure_label != "Bridged" {
    continue;
  }

  // We're only interested in bridges that cross Wissahickon Creek.
  if creek_name != "Wissahickon Creek" {
    continue;
  }

  // Ok, we've utilized some attributes to narrow our search,
  // now let's dig deeper with some geometric analysis.

  use wkt::TryFromWkt;
  let geometry = Geometry::try_from_wkt_str(geometry_str).expect("wkt must be valid");

  let bridge_centroid = geometry.centroid().expect("a centroid should exist for any non-empty geometry");

  // We're only interested in the part of the Wissahickon Creek that's within
  // the Wissahickon Valley Park.
  let SOUTHERN_PARK_BORDER = 40.013214;
  let NORTHERN_PARK_BORDER = 40.084306;
  if bridge_centroid.y() < SOUTHERN_PARK_BORDER || bridge_centroid.y() > NORTHERN_PARK_BORDER {
    continue;
  }

  // Compute the size of the bridge
  let bridge_area = {
    // In the previous article about projections, we learned how to transform lat/lon to a local
    // projection to get useful area calculations.
    //
    // WGS84 - World Geodetic System, aka lat/lon
    // EPSG:3364 - NAD83(HARN) / Pennsylvania South (meters)
    let geometry_in_meters = geometry.transformed_crs_to_crs("WGS84", "EPSG:3364").expect("valid transformation");
    geometry_in_meters.unsigned_area()
  };

  // We're not intested in walking across large automobile bridges.
  if bridge_area > 250.0 {
    continue;
  }

  // Using attribute data and geometric processing, we've identified a good walking bridge!
  acceptable_walkabout_bridges.push(bridge_centroid);
}

assert_eq!(acceptable_walkabout_bridges.len(), 8);
approx::assert_relative_eq!(acceptable_walkabout_bridges[3], Point::new(-75.22563703858332, 40.071892693259315));
