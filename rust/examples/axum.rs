/*
 * MIT License
 *
 * Copyright (c) 2025 Jasmine Tai
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::get;
use axum::{Json, Router};
use cf_access::Validator;
use serde_json::{json, Value};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    #[cfg(feature = "env")]
    let validator = Validator::from_env().unwrap();
    #[cfg(not(feature = "env"))]
    let validator = Validator::new("team_name", "audience");

    let app = Router::new()
        .route("/", get(handler))
        .with_state(Arc::new(validator));

    let port = 3000;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(validator: State<Arc<Validator>>, headers: HeaderMap) -> Json<Value> {
    let Some(jwt) = headers.get("cf-access-jwt-assertion") else {
        return Json(json!({ "error": "JWT not found" }));
    };
    let Ok(jwt) = jwt.to_str() else {
        return Json(json!({ "error": "JWT is invalid" }));
    };
    match validator.validate(jwt).await {
        Ok(token) => Json(json!({ "result": token, "jwt": jwt })),
        Err(error) => Json(json!({ "error": error.to_string() })),
    }
}
