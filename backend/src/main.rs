use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

const JWT_SECRET: &[u8] = b"ferris-hub-secret-key-2026";

#[derive(Clone, Serialize, Deserialize)]
struct User {
    username: String,
    role: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    name: String,
    exp: usize,
}

#[derive(Clone, Serialize, Deserialize)]
struct ProjectItem {
    id: String,
    title: String,
    category: String,
    status: String,
    col_span: u8,
    row_span: u8,
    metrics: String,
    updated_at: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ActivityLog {
    id: String,
    action: String,
    detail: String,
    time: String,
}

struct AppState {
    projects: Mutex<Vec<ProjectItem>>,
    logs: Mutex<Vec<ActivityLog>>,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Deserialize)]
struct CreateProjectRequest {
    title: String,
    category: String,
    status: String,
    col_span: Option<u8>,
    row_span: Option<u8>,
    metrics: Option<String>,
}

#[tokio::main]
async fn main() {
    let initial_projects = vec![
        ProjectItem {
            id: "1".into(),
            title: "Rust Axum Core Runtime".into(),
            category: "Backend Engine".into(),
            status: "active".into(),
            col_span: 2,
            row_span: 1,
            metrics: "0.2ms latency / 12k req/s".into(),
            updated_at: "Just now".into(),
        },
        ProjectItem {
            id: "2".into(),
            title: "JWT Key Vault".into(),
            category: "Security".into(),
            status: "verified".into(),
            col_span: 1,
            row_span: 1,
            metrics: "HS256 signed".into(),
            updated_at: "2m ago".into(),
        },
        ProjectItem {
            id: "3".into(),
            title: "TanStack Query Cache".into(),
            category: "Client State".into(),
            status: "synced".into(),
            col_span: 1,
            row_span: 2,
            metrics: "5m staleTime / 0 GC".into(),
            updated_at: "4m ago".into(),
        },
        ProjectItem {
            id: "4".into(),
            title: "Vue 3 Composable Bridge".into(),
            category: "Frontend Core".into(),
            status: "active".into(),
            col_span: 2,
            row_span: 1,
            metrics: "TypeScript strict".into(),
            updated_at: "10m ago".into(),
        },
        ProjectItem {
            id: "5".into(),
            title: "Herd Local Reverse Proxy".into(),
            category: "Infrastructure".into(),
            status: "listening".into(),
            col_span: 1,
            row_span: 1,
            metrics: "ferris-hub.test:80".into(),
            updated_at: "Live".into(),
        },
        ProjectItem {
            id: "6".into(),
            title: "FlyEnv MySQL Connector".into(),
            category: "Database Pool".into(),
            status: "ready".into(),
            col_span: 2,
            row_span: 1,
            metrics: "Port 3306 available".into(),
            updated_at: "Standby".into(),
        },
    ];

    let initial_logs = vec![
        ActivityLog {
            id: "log-1".into(),
            action: "SYS_INIT".into(),
            detail: "Axum router initialized on port 8088".into(),
            time: "10:00:00".into(),
        },
        ActivityLog {
            id: "log-2".into(),
            action: "AUTH_READY".into(),
            detail: "JWT verification engine loaded".into(),
            time: "10:00:02".into(),
        },
    ];

    let state = Arc::new(AppState {
        projects: Mutex::new(initial_projects),
        logs: Mutex::new(initial_logs),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/me", get(me_handler))
        .route("/api/projects", get(get_projects).post(create_project))
        .route("/api/projects/:id", axum::routing::delete(delete_project))
        .route("/api/logs", get(get_logs))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8088")
        .await
        .expect("Failed to bind port 8088");

    println!("FerrisHub Rust API running on http://127.0.0.1:8088");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "FerrisHub Rust Engine",
        "framework": "Axum 0.7",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<impl IntoResponse, StatusCode> {
    // Demo credential: admin / ferris123
    if payload.username == "admin" && payload.password == "ferris123" {
        let claims = Claims {
            sub: "admin".into(),
            role: "Developer".into(),
            name: "Ferris Engineer".into(),
            exp: 10_000_000_000,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET),
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(LoginResponse {
            token,
            user: User {
                username: "admin".into(),
                role: "Lead Fullstack Dev".into(),
                name: "Ferris Engineer".into(),
            },
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn extract_jwt(headers: &HeaderMap) -> Result<Claims, StatusCode> {
    let auth = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(decoded.claims)
}

async fn me_handler(headers: HeaderMap) -> Result<impl IntoResponse, StatusCode> {
    let claims = extract_jwt(&headers)?;
    Ok(Json(User {
        username: claims.sub,
        role: claims.role,
        name: claims.name,
    }))
}

async fn get_projects(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let list = state.projects.lock().unwrap().clone();
    Json(list)
}

async fn create_project(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let _claims = extract_jwt(&headers)?;

    let new_item = ProjectItem {
        id: uuid::Uuid::new_v4().to_string()[..8].to_string(),
        title: body.title,
        category: body.category,
        status: body.status,
        col_span: body.col_span.unwrap_or(1),
        row_span: body.row_span.unwrap_or(1),
        metrics: body.metrics.unwrap_or_else(|| "100% active".into()),
        updated_at: "Just now".into(),
    };

    {
        let mut list = state.projects.lock().unwrap();
        list.insert(0, new_item.clone());
    }

    {
        let mut logs = state.logs.lock().unwrap();
        logs.insert(
            0,
            ActivityLog {
                id: format!("log-{}", uuid::Uuid::new_v4().to_string()[..6].to_string()),
                action: "ITEM_CREATED".into(),
                detail: format!("Created '{}'", new_item.title),
                time: chrono::Local::now().format("%H:%M:%S").to_string(),
            },
        );
    }

    Ok((StatusCode::CREATED, Json(new_item)))
}

async fn delete_project(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let _claims = extract_jwt(&headers)?;

    let mut list = state.projects.lock().unwrap();
    if let Some(pos) = list.iter().position(|p| p.id == id) {
        let removed = list.remove(pos);
        drop(list);

        let mut logs = state.logs.lock().unwrap();
        logs.insert(
            0,
            ActivityLog {
                id: format!("log-{}", uuid::Uuid::new_v4().to_string()[..6].to_string()),
                action: "ITEM_DELETED".into(),
                detail: format!("Deleted '{}'", removed.title),
                time: chrono::Local::now().format("%H:%M:%S").to_string(),
            },
        );

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn get_logs(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let logs = state.logs.lock().unwrap().clone();
    Json(logs)
}
