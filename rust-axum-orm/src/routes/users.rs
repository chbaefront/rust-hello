use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::{Value, json};

use crate::{
    AppState,
    entities::users::{self, Entity, Model},
};

pub fn users() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/{user_id}", get(find_user))
}

async fn list_users(State(state): State<AppState>) -> Json<Value> {
    let users: Vec<Model> = Entity::find().all(&state.db).await.unwrap();
    Json(json!({ "users": users }))
}

async fn find_user(Path(user_id): Path<i32>, State(state): State<AppState>) -> Json<Value> {
    let user = Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .one(&state.db)
        .await
        .unwrap();

    Json(json!({ "user": user }))
}
