use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    Router,
};
use std::{io, path::PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct PathState {
    path: PathBuf,
}

pub async fn serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    if !dir.exists() {
        anyhow::bail!("Directory does not exist: {}", dir.display());
    }
    let state = PathState { path: dir.clone() };

    let addr = format!("0.0.0.0:{}", port);
    let app = Router::new()
        .nest_service(
            "/",
            ServeDir::new(dir).append_index_html_on_directories(true),
        )
        .layer(middleware::from_fn_with_state(state, gen_index_html));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn gen_index_html(
    State(PathState { path }): State<PathState>,
    request: Request,
    next: Next,
) -> Response {
    let url_path = request.uri().path();
    let url_path = url_path.strip_prefix('/').unwrap_or(url_path);
    let path = path.join(url_path);
    if path.is_dir() {
        let index = path.join("index.html");
        if !index.exists() {
            if let Err(err) = generate_directory_index(path).await {
                println!("Failed to generate directory index: {}", err)
            }
        }
    }
    next.run(request).await
}

async fn generate_directory_index(dir_path: PathBuf) -> io::Result<()> {
    if !dir_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

    let mut entries = fs::read_dir(dir_path.clone()).await?;
    let index_path = dir_path.join("index.html");
    let mut index_file = File::create(index_path).await?;

    index_file
        .write_all(b"<html><head><title>Directory Index</title></head><body>")
        .await?;
    index_file
        .write_all(format!("<h1>Index of {}</h1>", dir_path.display()).as_bytes())
        .await?;
    index_file.write_all(b"<ul>").await?;

    while let Some(entry) = entries.next_entry().await? {
        let file_name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|_| String::from("Invalid UTF-8"));

        // Write the list item for the directory entry.
        index_file
            .write_all(format!("<li><a href=\"{}\">{}</a></li>", file_name, file_name).as_bytes())
            .await?;
    }

    index_file.write_all(b"</ul>").await?;
    index_file.write_all(b"</body></html>").await?;

    Ok(())
}
