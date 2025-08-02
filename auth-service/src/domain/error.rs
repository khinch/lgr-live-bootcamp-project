pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    ValidationError,
    UnexpectedError,
    IncorrectCredentials,
}
