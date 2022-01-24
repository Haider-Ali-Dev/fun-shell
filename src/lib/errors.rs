use thiserror::Error;


#[derive(Error, Debug)]
pub enum ArgsError {
    #[error("Not a valid argument")]
    InvalidArg,
    #[error("System Error")]
    SystemError
}