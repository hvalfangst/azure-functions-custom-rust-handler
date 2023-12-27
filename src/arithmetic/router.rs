pub mod router {
    use axum::{
        http::StatusCode,
        response::IntoResponse,
        Router,
        Json,
    };
    use axum::extract::Query;
    use serde_json::{json, Value};
    use crate::arithmetic::{
        service::service::{add_numbers, subtract_numbers, multiply_numbers, divide_numbers},
    };
    use serde_derive::Deserialize;

    // - - - - - - - - - - - [ROUTES] - - - - - - - - - - -

    pub fn arithmetic_routes() -> Router {
        Router::new().route("/api/arithmetic", axum::routing::get(arithmetic_handler))
    }

    // - - - - - - - - - - - [HANDLERS] - - - - - - - - - - -

    pub async fn arithmetic_handler(query: Query<ArithmeticParams>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let params = query.0;

        match params.mode.as_str() {
            "add" => {
                let result = add_numbers(params.num1, params.num2);
                Ok((StatusCode::CREATED, Json(result)))
            }
            "subtract" => {
                let result = subtract_numbers(params.num1, params.num2);
                Ok((StatusCode::CREATED, Json(result)))
            }
            "multiply" => {
                let result = multiply_numbers(params.num1, params.num2);
                Ok((StatusCode::CREATED, Json(result)))
            }
            "divide" => match divide_numbers(params.num1, params.num2) {
                Some(result) => Ok((StatusCode::CREATED, Json(result))),
                None => Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error": "Divide by zero"})))),
            },
            _ => {
                // Return magic number on illegal mode
                Ok((StatusCode::IM_A_TEAPOT, Json(-666.0)))
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct ArithmeticParams {
        mode: String,
        num1: i32,
        num2: i32,
    }
}
