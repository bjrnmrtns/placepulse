use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use quick_xml::de::from_str;
use serde::Deserialize;
use zip::ZipArchive;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct RootTemplate {
    documents: Vec<Document>,
}

#[derive(Clone)]
struct AppState {
    documents: Vec<Document>,
}

#[tokio::main]
async fn main() {
    let documents = parse_camt053_zip();
    let state = Arc::new(Mutex::new(AppState { documents }));
    let app = Router::new()
        .route("/", get(root))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let documents = state.lock().unwrap().documents.clone();
    let template = RootTemplate { documents };
    let html = template.render().unwrap();
    Html(html).into_response()
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Document {
    #[serde(rename = "BkToCstmrStmt")]
    bk_to_cstmr_stmt: BkToCstmrStmt,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BkToCstmrStmt {
    grp_hdr: GrpHdr,
    stmt: Stmt,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GrpHdr {
    msg_id: String,
    cre_dt_tm: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Stmt {
    id: String,
    elctrnc_seq_nb: String,
    stmt_dtls: Option<StmtDtls>,
    ntry: Vec<Ntry>,
    bal: Vec<Bal>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Bal {
    amt: String,
    cdt_dbt_ind: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Ntry {
    amt: f64,
    cdt_dbt_ind: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StmtDtls {
    tx: Tx,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Tx {
    amt: f64,
    cdt_dbt_ind: String,
    bk_tx_cd: String,
    rmt_inf: String,
}

fn parse_camt053_zip() -> Vec<Document> {
    let zip_file =
        File::open("/home/bjorn/projects/personal/Documents/bank-transations/2024/203221559_060225200125.zip").unwrap();
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    let mut documents = Vec::new();
    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i).unwrap();
        let mut xml_data = String::new();
        file.read_to_string(&mut xml_data).unwrap();
        let document: Document = from_str(&xml_data).unwrap();
        documents.push(document);
    }
    documents
}

