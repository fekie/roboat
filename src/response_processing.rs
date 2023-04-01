use crate::{RoboatError, XCSRF_HEADER};
use reqwest::Response;
use serde::{Deserialize, Serialize};

/// Roblox's error response used when a status code of 403 is given. Only the first error
/// is used when converting to [`RoboatError`].
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
struct RobloxErrorResponse {
    errors: Vec<RobloxErrorRaw>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
struct RobloxErrorRaw {
    code: u16,
    message: String,
}

/// Used to process a 403 response from an endpoint. This requires new xcsrf to be
/// pulled and returned inside an error
pub(crate) async fn process_403(request_response: Response) -> RoboatError {
    let headers = request_response.headers().clone();
    let xcsrf = headers
        .get(XCSRF_HEADER)
        .map(|x| x.to_str().unwrap().to_string());

    match xcsrf {
        // If the xcsrf exists, we can send back invalid xcsrfs.
        Some(xcsrf) => {
            // If the response cannot be parsed, and the xcsrf exists, we return an invalid xcsrf error.
            let error_response = match request_response.json::<RobloxErrorResponse>().await {
                Ok(x) => x,
                Err(_) => {
                    return RoboatError::InvalidXcsrf(xcsrf);
                }
            };

            match error_response.errors.first() {
                Some(error) => match error.code {
                    0 => RoboatError::InvalidXcsrf(xcsrf),
                    9 => RoboatError::UserDoesNotOwnAsset,
                    _ => RoboatError::UnknownRobloxErrorCode {
                        code: error.code,
                        message: error.message.clone(),
                    },
                },
                None => RoboatError::InvalidXcsrf(xcsrf),
            }
        }
        // Otherwise, we parse the error knowing it doesn't exist
        None => {
            // If the response cannot be parsed, and the xcsrf does not exist, we return an xcsrf not returned error.
            let error_response = match request_response.json::<RobloxErrorResponse>().await {
                Ok(x) => x,
                Err(_) => {
                    return RoboatError::XcsrfNotReturned;
                }
            };

            match error_response.errors.first() {
                Some(error) => match error.code {
                    0 => RoboatError::XcsrfNotReturned,
                    9 => RoboatError::UserDoesNotOwnAsset,
                    _ => RoboatError::UnknownRobloxErrorCode {
                        code: error.code,
                        message: error.message.clone(),
                    },
                },
                None => RoboatError::MalformedResponse,
            }
        }
    }
}
