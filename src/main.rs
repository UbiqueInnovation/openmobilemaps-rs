use std::time::Instant;

use openmobilemaps_rs::{draw_map, MeetweenConnections};

fn main() {
    let start = Instant::now();
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let points = serde_urlencoded::to_string(
        &args
            .iter()
            .map(|s| ("startingPoints", s.as_str()))
            .collect::<Vec<(&str, &str)>>(),
    )
    .unwrap();

    let url = format!(
        "https://app-dev-routing.viadi-zero.ch/v1/meet?{points}&limitStations=3&limitWorkspaces=0"
    );
    let connections = ureq::get(&url).call().unwrap().into_string().unwrap();

    std::fs::write("tmp_connection.json", &connections);
    let meetween_connections: MeetweenConnections = serde_json::from_str(&connections).unwrap();

    let olten = draw_map(&meetween_connections.stations[0]);
    let destination = meetween_connections.stations[0].meeting_point.name.clone();
    std::fs::write(format!("{destination}.png"), olten);

    let end = Instant::now();
    println!("Everything took {}ms", (end - start).as_millis());
    println!("Saved image to {destination}.png");
}

#[cfg(test)]
mod image_tests {
    use image::RgbaImage;
    use openmobilemaps_rs::{html_hex, MeetweenConnections};

    use crate::{get_destination_box, get_start_point};
    #[test]
    fn test_destination_box() {
        let (width, height, image_data) = get_destination_box("Hallo Welt");
        let image = RgbaImage::from_raw(width as u32, height as u32, image_data).unwrap();
        image.save("destination_box.png");
    }
    #[test]
    fn test_circle() {
        let color_outer = [
            (1.0 * 255.0) as u8,
            (0.3 * 255.0) as u8,
            (0.34 * 255.0) as u8,
            50,
        ];
        let color_inner = [
            (1.0 * 255.0) as u8,
            (0.3 * 255.0) as u8,
            (0.34 * 255.0) as u8,
            255,
        ];
        let start_point = RgbaImage::from_raw(
            800,
            800,
            get_start_point("1", html_hex!("#FFF"), color_outer, color_inner, 80, 80),
        )
        .unwrap();

        start_point.save("test_output.png");
    }
    #[test]
    fn test_json() {
        let json = include_str!("../testconnection.json");
        let meetween_connections: MeetweenConnections = serde_json::from_str(json).unwrap();
        // println!("{:?}", meetween_connections);
    }
}
