use crate::errors::ServiceError;

pub mod user;
pub mod profile;

pub type Multiple<T> = Result<Vec<T>, ServiceError>;
pub type Single<T> = Result<T, ServiceError>;
