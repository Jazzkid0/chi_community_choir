use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
async fn home() -> Result<Response> {
    format::json(())
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(home))
}
