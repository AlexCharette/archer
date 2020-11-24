// use std::error::Error;
// use std::fmt::{Display, Formatter, Result};

// TODO implement errors

// #[derive(Debug)]
// enum ApiError {
//     BadRequest(),
//     InternalError(),
//     Unauthorized(),
//     NotFound(),
//     Other,
// }

// impl Display for ApiError {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match *self {
//             ApiError::BadRequest(ref cause) => write!(f, "Api Error - bad request: {}", cause),
//             ApiError::InternalError(ref cause) => write!(f, "Api Error - internal error: {}", cause),
//             ApiError::Unauthorized(ref cause) => write!(f, "Api Error - unauthorized: {}", cause),
//             ApiError::NotFound(ref cause) => write!(f, "Api Error - not found: {}", cause),
//             ApiError::Other => write!(f, "Unknown error!"),
//         }
//     }
// }

// impl Error for ApiError {
//     fn description(&self) -> &str {
//         match *self {
//             ApiError::BadRequest(ref cause) => cause.description(),
//             ApiError::InternalError(ref cause) => cause.description(),
//             ApiError::Unauthorized(ref cause) => cause.description(),
//             ApiError::NotFound(ref cause) => cause.description(),
//             ApiError::Other => "Unknown error!",
//         }
//     }

//     fn cause(&self) -> Option<&Error> {
//         match *self {
//             ApiError::BadRequest(ref cause) => Some(cause),
//             ApiError::InternalError(ref cause) => Some(cause),
//             ApiError::Unauthorized(ref cause) => Some(cause),
//             ApiError::NotFound(ref cause) => Some(cause),
//             ApiError::Other => None,
//         }
//     }
// }
