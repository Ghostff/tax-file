use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::{Data, Json, Path};
use serde_json::json;

use crate::AppState;
use crate::controllers::api::OrganizationPathParams;
use crate::models::user_model::{CreateUserSchema};
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;
use crate::utilities::error_bag::ErrorBag;
use crate::utilities::json_response::JsonResponse;

pub async fn index(_req: HttpRequest, _app: Data<AppState>) -> Result<HttpResponse, ErrorBag> {
    // crate::gate!(&app.pool, &req, "users.view");

    Ok(JsonResponse::success(""))
}

pub async fn show(_req: HttpRequest, app: Data<AppState>, path: Path<OrganizationPathParams>) -> Result<HttpResponse, ErrorBag> {
    // crate::gate!(&app.pool, &req, "users.view");

    let target_user_id = path.into_inner().id;
    let target_user = UserRepository::find_by_id(&app.pool, &target_user_id).await?;

    Ok(JsonResponse::success(json!({ "user": target_user })))
}

pub async fn update(
    _req: HttpRequest,
    app: Data<AppState>,
    path: Path<OrganizationPathParams>,
    body: Json<CreateUserSchema>,
) -> Result<HttpResponse, ErrorBag> {
    // crate::gate!(&app.pool, &req, "users.update");

    let mut target_user = UserRepository::find_by_id(&app.pool, &path.into_inner().id).await?;

    let email = body.email.clone().trim().to_lowercase();
    if target_user.email != email && UserRepository::email_exist(&app.pool, &email).await? {
        return Err(ErrorBag::EmailInUse);
    }

    target_user.first_name = body.first_name.clone();
    target_user.last_name = body.last_name.clone();
    target_user.email = email;

    let mut tx = app.pool.begin().await?;

    UserRepository::update(&mut *tx, &target_user).await?;

    // @todo: update user cache

    Ok(JsonResponse::success(json!({ "message": "UserModel updated successfully" })))
}

pub async fn create(_req: HttpRequest, app: Data<AppState>, body: Json<CreateUserSchema>) -> Result<HttpResponse, ErrorBag> {


    let mut tx = app.pool.begin().await?;

    let new_user = UserService::create(&mut tx, &body).await?;


    tx.commit().await?;

    Ok(JsonResponse::success(json!({ "user": new_user })))
}

pub async fn delete(_req: HttpRequest, app: Data<AppState>, path: Path<OrganizationPathParams>) -> Result<HttpResponse, ErrorBag> {
    // crate::gate!(&app.pool, &req, "users.delete");

    match UserRepository::delete(&app.pool, &path.into_inner().id).await? {
        0 => Err(ErrorBag::NotFound("UserModel".into())),
        _ => Ok(JsonResponse::success(json!({ "message": "UserModel deleted successfully" }))),
    }
}
