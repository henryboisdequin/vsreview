pub enum AuthErr<'a> {
    PasswordNotMatch(&'a str),
    InvalidEmail(&'a str),
    PasswordRequirementsNotMet(&'a str),
    UsernameTaken(&'a str),
}
