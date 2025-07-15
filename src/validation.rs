use crate::{Client, RoboatError, XCSRF_HEADER};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

// I really hate the way new updates of this library work but I don't want a dependency to be outdated
use base64::{engine::general_purpose, Engine as _};
/// Roblox's error response used when a status code of 403 is given. Only the first error
/// is used when converting to [`RoboatError`].
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
struct RobloxErrorResponse {
    pub errors: Vec<RobloxErrorRaw>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct RobloxErrorRaw {
    pub code: u16,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChallengeMetadata {
    pub user_id: String,
    pub challenge_id: String,
    pub should_show_remember_device_checkbox: bool,
    pub remember_device: bool,
    pub session_cookie: String,
    pub verification_token: String,
    pub action_type: String,
    pub request_path: String,
    pub request_method: String,
}

impl Client {
    /// Used to process a 403 response from an endpoint. This status is returned when a challenge is needed
    /// or when the xcsrf is invalid.
    async fn process_403(request_response: Response) -> RoboatError {
        let headers = request_response.headers().clone();

        // We branch here depending on whether it can parse into a `RobloxErrorResponse` or not.
        // If it can, it means a challenge is required and we return a `RoboatError::ChallengeRequired(_)`.
        // Otherwise, we return an xcsrf related error.

        match request_response.json::<RobloxErrorResponse>().await {
            Ok(x) => {
                // We make sure the first error exists and is a challenge required error.
                match x.errors.first() {
                    Some(error) => {
                        if error.code == 0 {
                            // A hack here, but sometimes they give a 403 with a code of 0
                            // with no message. This is a xcsrf error.
                            let xcsrf = headers
                                .get(XCSRF_HEADER)
                                .map(|x| x.to_str().unwrap().to_string());

                            return match xcsrf {
                                Some(x) => RoboatError::InvalidXcsrf(x),
                                None => RoboatError::XcsrfNotReturned,
                            };
                        }

                        if error.message != "Challenge is required to authorize the request" {
                            return RoboatError::UnknownRobloxErrorCode {
                                code: error.code,
                                message: error.message.clone(),
                            };
                        }
                    }
                    None => {
                        return RoboatError::UnknownStatus403Format;
                    }
                }

                // For some really really *stupid* reason, the header `rblx-challenge-id` is not the real challenge id.
                // The challenge id is actually inside the header `rblx-challenge-metadata`, which is encoding in base64.

                // We get the challenge metadata from the headers, and error if we cant.
                let metadata_encoded = match headers
                    .get("rblx-challenge-metadata")
                    .map(|x| x.to_str().unwrap().to_string())
                {
                    Some(x) => x,
                    None => {
                        return RoboatError::UnknownStatus403Format;
                    }
                };

                // We can unwrap here because we're kinda screwed if it's spitting out other stuff and the library would need to be fixed.
                let metadata = general_purpose::STANDARD.decode(metadata_encoded).unwrap();

                // We parse the metadata into a struct, and error if we cant.
                let metadata_struct: ChallengeMetadata = match serde_json::from_slice(&metadata) {
                    Ok(x) => x,
                    Err(_) => {
                        return RoboatError::UnknownStatus403Format;
                    }
                };

                // We return the challenge required error.
                RoboatError::ChallengeRequired(metadata_struct.challenge_id)
            }
            Err(_) => {
                // If we're down here, it means that the response is not a challenge required error and we
                // can return xcsrf if it exists
                let xcsrf = headers
                    .get(XCSRF_HEADER)
                    .map(|x| x.to_str().unwrap().to_string());

                match xcsrf {
                    Some(x) => RoboatError::InvalidXcsrf(x),
                    None => RoboatError::XcsrfNotReturned,
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
            Some(error) => RoboatError::UnknownRobloxErrorCode {
                code: error.code,
                message: error.message.clone(),
            },
            None => RoboatError::BadRequest,
        }
    }

    /// Jump to the [Examples](crate#examples) section.
    async fn handle_non_200_status_codes(
        request_response: Response,
    ) -> Result<Response, RoboatError> {
        let status_code = request_response.status().as_u16();

        match status_code {
            200 => Ok(request_response),
            400 => Err(Self::process_400(request_response).await),
            401 => Err(RoboatError::InvalidRoblosecurity),
            403 => Err(Self::process_403(request_response).await),
            429 => Err(RoboatError::TooManyRequests),
            500 => Err(RoboatError::InternalServerError),
            _ => Err(RoboatError::UnidentifiedStatusCode(status_code)),
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
            Ok(response) => Self::handle_non_200_status_codes(response).await,
            Err(e) => Err(RoboatError::ReqwestError(e)),
        }
    }

    /// Parses a json from a [`reqwest::Response`] into a response struct, returning an error if the response is malformed.
    pub(crate) async fn parse_to_raw<T: DeserializeOwned>(
        response: Response,
    ) -> Result<T, RoboatError> {
        let response_struct = match response.json::<T>().await {
            Ok(x) => x,
            Err(_) => {
                return Err(RoboatError::MalformedResponse);
            }
        };

        Ok(response_struct)
    }
}
