use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
// 文字列をプリント、またはフォーマットするための機能
use std::fmt;

#[derive(Debug, Deserialize)]
// CustomErrorの型を定義
pub struct CustomError {
  pub error_status_code: u16,
  pub error_message: String,
}

// implを使って、型にトレイト(クラス)を実装
impl CustomError {
  pub fn new(error_status_code: u16, error_message: String) -> CustomError {
    CustomError {
      error_status_code,
      error_message,
    }
  }
}

// CustomErrorトレイトをfmt::Displayに実装(CustomErrorのメソッドを定義)
impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // 書き込みが成功したかを返す
    f.write_str(self.error_message.as_str())
  }
}

impl From<DieselError> for CustomError {
  fn from(error: DieselError) -> CustomError {
    match error {
      // DieselError::DatabaseErrorの場合は409のエラーを返す
      DieselError::DatabaseError(_, err) => CustomError::new(409, err.message().to_string()),
      // DieselError::NotFoundの場合は404のエラーを返す
      DieselError::NotFound => {
        CustomError::new(404, "The employee record not found".to_string())
      }
      // errの場合は500のエラーを返す
      err => CustomError::new(500, format!("Unknown Diesel error: {}", err)),
    }
  }
}

impl ResponseError for CustomError {
  fn error_response(&self) -> HttpResponse {
    // status_codeの変数を定義
    let status_code = match StatusCode::from_u16(self.error_status_code) {
      Ok(status_code) => status_code,
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    // error_messageの定義
    let error_message = match status_code.as_u16() < 500 {
      true => self.error_message.clone(),
      false => "Internal server error".to_string(),
    };

    // 返り値を定義
    HttpResponse::build(status_code).json(json!({ "message": error_message }))
  }
}