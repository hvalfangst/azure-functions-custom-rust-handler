pub mod router {
    use axum::{
        extract::Path,
        http::StatusCode,
        response::IntoResponse,
        Router,
        Json,
    };
    use serde_json::{json, Value};
    use crate::arithmetic::{
        service::service::{add_numbers, subtract_numbers, multiply_numbers, divide_numbers, get_average, routes_metadata},
    };

    // - - - - - - - - - - - [ROUTES] - - - - - - - - - - -

    pub fn arithmetic_routes() -> Router {
        Router::new()
            .route("/index", axum::routing::get(index_handler))
            .route("/arithmetic/add/:num1/:num2", axum::routing::get(add_numbers_handler))
            .route("/arithmetic/subtract/:num1/:num2", axum::routing::get(subtract_numbers_handler))
            .route("/arithmetic/multiply/:num1/:num2", axum::routing::get(multiply_numbers_handler))
            .route("/arithmetic/divide/:num1/:num2", axum::routing::get(divide_numbers_handler))
            .route("/arithmetic/average", axum::routing::post(calculate_average_handler))
    }

    // - - - - - - - - - - - [HANDLERS] - - - - - - - - - - -

    pub async fn index_handler() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        Ok((StatusCode::OK, Json(routes_metadata())))
    }

    pub async fn add_numbers_handler(path: Path<(i32, i32)>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let (num1, num2) = path.0;
        let result = add_numbers(num1, num2);
        Ok((StatusCode::CREATED, Json(result)))
    }

    pub async fn subtract_numbers_handler(path: Path<(i32, i32)>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let (num1, num2) = path.0;
        let result = subtract_numbers(num1, num2);
        Ok((StatusCode::CREATED, Json(result)))
    }

    pub async fn multiply_numbers_handler(path: Path<(i32, i32)>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let (num1, num2) = path.0;
        let result = multiply_numbers(num1, num2);
        Ok((StatusCode::CREATED, Json(result)))
    }

    pub async fn divide_numbers_handler(path: Path<(i32, i32)>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let (numerator, denominator) = path.0;
        match divide_numbers(numerator, denominator) {
            Some(result) => Ok((StatusCode::CREATED, Json(result))),
            None => Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error": "Divide by zero"}))))
        }
    }

    pub async fn calculate_average_handler(Json(input_list): Json<Vec<i32>>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        match get_average(&input_list) {
            Some(average) =>Ok((StatusCode::CREATED, Json(average))),
            None => Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error": "Empty input list"}))))
        }
    }
}
