pub enum AuthAPIError {
    MissingToken,
    IncorrectCredentials,
    InvalidToken,
    UnexpectedError,
    UserAlreadyExists,
    UserNotFound,
    ValidationError,
}
