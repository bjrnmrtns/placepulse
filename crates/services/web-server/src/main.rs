use std::{
    fs::File, io::{BufRead, BufReader}, sync::{Arc, Mutex}
};

use askama::Template;
use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use geo::{HaversineDistance, Point};
use quick_xml::{events::Event, Reader};
use zip::ZipArchive;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct RootTemplate {
    counter: u32,
}

#[derive(askama::Template)]
#[template(path = "counter.html")]
struct CounterTemplate {
    counter: u32,
}

#[derive(Clone)]
struct AppState {
    counter: u32,
}

#[tokio::main]
async fn main() {
    parse_camt053_zip();
    let state = Arc::new(Mutex::new(AppState { counter: 0 }));
    let app = Router::new()
        .route("/", get(root))
        .route("/increment", post(increment))
        .route("/upload", post(upload))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn increment(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    let template = CounterTemplate { counter: state.counter };
    let html = template.render().unwrap();
    Html(html).into_response()
}

async fn root(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let template = RootTemplate {
        counter: state.lock().unwrap().counter,
    };
    let html = template.render().unwrap();
    Html(html).into_response()
}

fn calculate_segment_distance(segment: &gpx::TrackSegment) -> f64 {
    segment
        .points
        .windows(2)
        .map(|window| {
            let start = &window[0];
            let end = &window[1];
            Point::new(start.point().x(), start.point().y())
                .haversine_distance(&Point::new(end.point().x(), end.point().y()))
        })
        .sum()
}

fn calculate_track_distance(track: &gpx::Track) -> f64 {
    track.segments.iter().map(calculate_segment_distance).sum()
}

fn parse_camt053_zip() {
    let zip_file =
        File::open("/home/bjorn/projects/personal/Documents/bank-transations/2024/203221559_060225200121.zip").unwrap();
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i).unwrap();
        let reader = BufReader::new(&mut file);
        parse_camt053(reader);
    }
}

fn parse_camt053<R: BufRead>(reader: R) {
        let mut buf = Vec::new();
        let mut xml_reader = Reader::from_reader(reader);
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => println!("EOF"),
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"Stmt" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                _ => (),
            },
            Ok(Event::Text(e)) => println!("Text: {:?}", e.unescape()),
            Err(_) => panic!("error bitch"),
            _ => (),
        }
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    if let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap().to_string();
        if let Ok(data) = field.bytes().await {
            let len = data.len();
            let cursor = std::io::Cursor::new(data);
            if let Ok(gpx) = gpx::read(cursor) {
                let distance: f64 = gpx.tracks.iter().map(calculate_track_distance).sum();
                return Html(format!(
                    r#"<li>name: {}, bytes {}, version: {}, track-length: {}</li>"#,
                    name, len, gpx.version, distance
                ))
                .into_response();
            } else {
                return Html("<li>file is not a gpx file</li>").into_response();
            }
        } else {
            return Html("<li>file upload failed</li>").into_response();
        }
    }
    Html("<li>no file to upload</li>").into_response()
}
