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
async fn process_403(request_response: Response) -> RoboatError {
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

/// Used to process a status code 400 response from an endpoint. Although this usually just
/// returns `Bad Request`, sometimes roblox encodes errors in the response.
async fn process_400(request_response: Response) -> RoboatError {
    let error_response = match request_response.json::<RobloxErrorResponse>().await {
        Ok(x) => x,
        Err(_) => {
            return RoboatError::BadRequest;
        }
    };

    match error_response.errors.first() {
        Some(error) => match error.code {
            5 => RoboatError::AssetIdIsInvalid,
            _ => RoboatError::UnknownRobloxErrorCode {
                code: error.code,
                message: error.message.clone(),
            },
        },
        None => RoboatError::BadRequest,
    }
}

/// Takes the result of a `reqwest` request and catches any possible errors, whether it be
/// a non-200 status code or a `reqwest` error.
///
/// If this returns successfully, the response is guaranteed to have a status code of 200.
pub(crate) async fn validate_request_result(
    request_result: Result<Response, reqwest::Error>,
) -> Result<Response, RoboatError> {
    match request_result {
        Ok(response) => handle_non_200_status_codes(response).await,
        Err(e) => Err(RoboatError::ReqwestError(e)),
    }
}

async fn handle_non_200_status_codes(request_response: Response) -> Result<Response, RoboatError> {
    let status_code = request_response.status().as_u16();

    match status_code {
        200 => Ok(request_response),
        400 => Err(process_400(request_response).await),
        401 => Err(RoboatError::InvalidRoblosecurity),
        403 => Err(process_403(request_response).await),
        429 => Err(RoboatError::TooManyRequests),
        500 => Err(RoboatError::InternalServerError),
        503 => Err(RoboatError::RobloxOutage),
        _ => Err(RoboatError::UnidentifiedStatusCode(status_code)),
    }
}
