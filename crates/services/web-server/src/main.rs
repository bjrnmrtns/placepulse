use std::sync::{Arc, Mutex};

use askama::Template;
use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use geo::{HaversineDistance, Point};

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
