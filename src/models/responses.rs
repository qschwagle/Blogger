use uuid::Uuid;
use crate::models::user::GetUser;

pub enum ApiResponse {
    UserOrPasswordNotMatch,
    Authenticated(Uuid, Uuid),
    InternalError,
    UserNotFound,
    UserDeleted,
    UserNotDeleted,
    NotImplemented,
    UserCreated(GetUser),
    UserRetrieved(GetUser),
    UserRetrieveFailed,
    UserPatched,
    UserCreateFailed,
    DbError,
}


