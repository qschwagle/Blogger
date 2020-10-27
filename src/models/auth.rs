

use crate::appdata::AppData;
use crate::models::responses::ApiResponse;
/// returns session_id and user_id on success
/// else returns failure
pub async fn authenticate(app_data: &AppData, username: &String, password: &String) -> ApiResponse {
    ApiResponse::InternalError
}
