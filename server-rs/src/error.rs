use diesel::result::Error;

pub enum AuthErr<'a> {
    PasswordNotMatch(&'a str),
    InvalidEmail(&'a str),
    PasswordRequirementsNotMet(&'a str),
    DuplicatedUsername(&'a str),
    DuplicatedEmail(&'a str),
    Other(Error),
}
