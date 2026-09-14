use axum::extract::State;
use axum::{http::StatusCode, Json};

use crate::state::AppState;

#[derive(serde::Serialize)]
pub struct DesktopReleaseResponse {
    version: String,
    macos_dmg: Option<String>,
    windows_exe: Option<String>,
}

pub async fn latest_release(
    State(state): State<AppState>,
    axum::Extension(_claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<DesktopReleaseResponse>, StatusCode> {
    let client = reqwest::Client::new();
    let mut request = client
        .get("https://api.github.com/repos/AnshuPrince/aniki/releases/latest")
        .header("User-Agent", "aniki-api");
    if let Some(token) = &state.config.github_token {
        request = request.bearer_auth(token);
    }

    let response = request.send().await.map_err(|e| {
        tracing::error!(error = %e, "fetch latest desktop release failed");
        StatusCode::BAD_GATEWAY
    })?;
    if response.status() == StatusCode::NOT_FOUND {
        return Err(StatusCode::NOT_FOUND);
    }
    if !response.status().is_success() {
        tracing::error!(status = %response.status(), "fetch latest desktop release failed");
        return Err(StatusCode::BAD_GATEWAY);
    }

    let release: serde_json::Value = response.json().await.map_err(|e| {
        tracing::error!(error = %e, "decode latest desktop release failed");
        StatusCode::BAD_GATEWAY
    })?;
    let assets = release["assets"]
        .as_array()
        .ok_or(StatusCode::BAD_GATEWAY)?;
    let asset_url = |suffix: &str| {
        assets.iter().find_map(|asset| {
            if asset["name"].as_str()?.ends_with(suffix) {
                asset["browser_download_url"].as_str().map(str::to_string)
            } else {
                None
            }
        })
    };

    Ok(Json(DesktopReleaseResponse {
        version: release["tag_name"].as_str().unwrap_or_default().to_string(),
        macos_dmg: asset_url(".dmg"),
        windows_exe: asset_url(".exe").or_else(|| asset_url(".msi")),
    }))
}
