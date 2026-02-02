use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use validator::Validate;

use crate::{db::UserExt, dtos::{RequestQueryDto, UserReceiveFileDto, UserReceiveFileListResponseDto, UserSendFileDto, UserSendFileListResponseDto}, error::HttpError, middleware::JWTAuthMiddeware, AppState};

pub fn file_list_handler() -> Router {
    Router:: new()
        .route("/send", get(get_user_shared_files))
        .route("/recieve", get(get_receice_shared_files))
}

pub async fn get_user_shared_files(
    Query(params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware >
) -> Result<impl IntoResponse, HttpError> {
    params
        .validate()
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let page = params.page.unwrap_or(1);
    let limit = params.page.unwrap_or(10);

    let (shared_files, total_count) = app_state.db_client
        .get_sent_files(user_id.clone(), page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filtered_sent_files = UserSendFileDto::filter_send_user_files(&shared_files);

    let response = UserSendFileListResponseDto {
        status: "success".to_string(),
        files: filtered_sent_files,
        results: total_count,
    };

    Ok(Json(response))
}

pub async fn get_receive_shared_files(
    Query(params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware >
) -> Result<impl IntoResponse, HttpError> {
    params
        .validate()
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let page = params.page.unwrap_or(1);
    let limit = params.page.unwrap_or(10);

    let (received_files, total_count) = app_state.db_client
        .get_receive_files(user_id.clone(), page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filtered_received_files = UserReceiveFileDto::filter_receive_user_files(&received_files);

    let response = UserReceiveFileListResponseDto {
        status: "success".to_string(),
        files: filtered_received_files,
        results: total_count,
    };

    Ok(Json(response))
}