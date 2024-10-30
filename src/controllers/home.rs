#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;

use crate::views;

#[debug_handler]
async fn home(
    ViewEngine(v): ViewEngine<TeraView>,
    // Context(ctx): State<AppContext>,
) -> Result<Response> {
    views::home::home(&v)
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(home))
}
