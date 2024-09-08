use alloc::string::String;
use thiserror_no_std::Error;

use super::XRPLOtherResult;

#[derive(Debug, Error)]
pub enum XRPLResultException {
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("Expected result or error in the response.")]
    ExpectedResultOrError,
    #[error("Unexpected result type (expected {0:?}, got {1:?}).")]
    UnexpectedResultType(String, String),
    #[error("Index not found.")]
    IndexNotFound,
    #[error("Called unwrap on `XRPLOtherResult`.")]
    UnwrapOnOther,
    #[error("Expected a XRPL Result model but got `XRPLOtherResult`: {0:?}.")]
    ExpectedResult(XRPLOtherResult),
}