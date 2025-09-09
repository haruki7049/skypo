use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(greeting))
        .route("/addresses/list", get(response));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn response() -> Response {
    addresses().await.into_response()
}

async fn greeting() -> impl IntoResponse {
    Body::new(String::from("HI, I am Skypo server!!"))
}

async fn addresses() -> Json<Vec<SkypoID>> {
    Json(vec![SkypoID {
        id: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
    }])
}

#[derive(Clone, Copy)]
struct SkypoID {
    id: [u8; 32],
}

impl Serialize for SkypoID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for SkypoID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.id
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        )
    }
}
