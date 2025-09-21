/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! ICAP error codes and error handling
//! 
//! This module implements ICAP-specific error codes as defined in RFC 3507.

use http::StatusCode;
use std::fmt;

/// ICAP error codes as defined in RFC 3507
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IcapErrorCode {
    // 4xx Client Errors
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    RequestEntityTooLarge = 413,
    RequestUriTooLarge = 414,
    UnsupportedMediaType = 415,
    RequestedRangeNotSatisfiable = 416,
    ExpectationFailed = 417,

    // 5xx Server Errors
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,

    // ICAP-specific errors
    InvalidRequest,
    InvalidResponse,
    InvalidEncapsulated,
    InvalidPreview,
    PreviewRequired,
    Continue,
    NoContent,
}

impl IcapErrorCode {
    /// Get the HTTP status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            // 4xx Client Errors
            IcapErrorCode::BadRequest => StatusCode::BAD_REQUEST,
            IcapErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
            IcapErrorCode::PaymentRequired => StatusCode::PAYMENT_REQUIRED,
            IcapErrorCode::Forbidden => StatusCode::FORBIDDEN,
            IcapErrorCode::NotFound => StatusCode::NOT_FOUND,
            IcapErrorCode::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            IcapErrorCode::NotAcceptable => StatusCode::NOT_ACCEPTABLE,
            IcapErrorCode::ProxyAuthenticationRequired => StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            IcapErrorCode::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            IcapErrorCode::Conflict => StatusCode::CONFLICT,
            IcapErrorCode::Gone => StatusCode::GONE,
            IcapErrorCode::LengthRequired => StatusCode::LENGTH_REQUIRED,
            IcapErrorCode::PreconditionFailed => StatusCode::PRECONDITION_FAILED,
            IcapErrorCode::RequestEntityTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            IcapErrorCode::RequestUriTooLarge => StatusCode::URI_TOO_LONG,
            IcapErrorCode::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            IcapErrorCode::RequestedRangeNotSatisfiable => StatusCode::RANGE_NOT_SATISFIABLE,
            IcapErrorCode::ExpectationFailed => StatusCode::EXPECTATION_FAILED,

            // 5xx Server Errors
            IcapErrorCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            IcapErrorCode::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            IcapErrorCode::BadGateway => StatusCode::BAD_GATEWAY,
            IcapErrorCode::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            IcapErrorCode::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            IcapErrorCode::HttpVersionNotSupported => StatusCode::HTTP_VERSION_NOT_SUPPORTED,

            // ICAP-specific errors
            IcapErrorCode::InvalidRequest => StatusCode::BAD_REQUEST,
            IcapErrorCode::InvalidResponse => StatusCode::BAD_REQUEST,
            IcapErrorCode::InvalidEncapsulated => StatusCode::BAD_REQUEST,
            IcapErrorCode::InvalidPreview => StatusCode::BAD_REQUEST,
            IcapErrorCode::PreviewRequired => StatusCode::CONTINUE,
            IcapErrorCode::Continue => StatusCode::CONTINUE,
            IcapErrorCode::NoContent => StatusCode::NO_CONTENT,
        }
    }

    /// Get the error message
    pub fn message(&self) -> &'static str {
        match self {
            // 4xx Client Errors
            IcapErrorCode::BadRequest => "Bad Request",
            IcapErrorCode::Unauthorized => "Unauthorized",
            IcapErrorCode::PaymentRequired => "Payment Required",
            IcapErrorCode::Forbidden => "Forbidden",
            IcapErrorCode::NotFound => "Not Found",
            IcapErrorCode::MethodNotAllowed => "Method Not Allowed",
            IcapErrorCode::NotAcceptable => "Not Acceptable",
            IcapErrorCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            IcapErrorCode::RequestTimeout => "Request Timeout",
            IcapErrorCode::Conflict => "Conflict",
            IcapErrorCode::Gone => "Gone",
            IcapErrorCode::LengthRequired => "Length Required",
            IcapErrorCode::PreconditionFailed => "Precondition Failed",
            IcapErrorCode::RequestEntityTooLarge => "Request Entity Too Large",
            IcapErrorCode::RequestUriTooLarge => "Request URI Too Large",
            IcapErrorCode::UnsupportedMediaType => "Unsupported Media Type",
            IcapErrorCode::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
            IcapErrorCode::ExpectationFailed => "Expectation Failed",

            // 5xx Server Errors
            IcapErrorCode::InternalServerError => "Internal Server Error",
            IcapErrorCode::NotImplemented => "Not Implemented",
            IcapErrorCode::BadGateway => "Bad Gateway",
            IcapErrorCode::ServiceUnavailable => "Service Unavailable",
            IcapErrorCode::GatewayTimeout => "Gateway Timeout",
            IcapErrorCode::HttpVersionNotSupported => "HTTP Version Not Supported",

            // ICAP-specific errors
            IcapErrorCode::InvalidRequest => "Invalid ICAP Request",
            IcapErrorCode::InvalidResponse => "Invalid ICAP Response",
            IcapErrorCode::InvalidEncapsulated => "Invalid Encapsulated Data",
            IcapErrorCode::InvalidPreview => "Invalid Preview",
            IcapErrorCode::PreviewRequired => "Preview Required",
            IcapErrorCode::Continue => "Continue",
            IcapErrorCode::NoContent => "No Content",
        }
    }

    /// Get the error description
    pub fn description(&self) -> &'static str {
        match self {
            // 4xx Client Errors
            IcapErrorCode::BadRequest => "The request could not be understood by the server due to malformed syntax.",
            IcapErrorCode::Unauthorized => "The request requires user authentication.",
            IcapErrorCode::PaymentRequired => "The request requires payment.",
            IcapErrorCode::Forbidden => "The server understood the request, but is refusing to fulfill it.",
            IcapErrorCode::NotFound => "The server has not found anything matching the Request-URI.",
            IcapErrorCode::MethodNotAllowed => "The method specified in the Request-Line is not allowed for the resource identified by the Request-URI.",
            IcapErrorCode::NotAcceptable => "The resource identified by the request is only capable of generating response entities which have content characteristics not acceptable according to the accept headers sent in the request.",
            IcapErrorCode::ProxyAuthenticationRequired => "The client must first authenticate itself with the proxy.",
            IcapErrorCode::RequestTimeout => "The client did not produce a request within the time that the server was prepared to wait.",
            IcapErrorCode::Conflict => "The request could not be completed due to a conflict with the current state of the resource.",
            IcapErrorCode::Gone => "The requested resource is no longer available at the server and no forwarding address is known.",
            IcapErrorCode::LengthRequired => "The server refuses to accept the request without a defined Content-Length.",
            IcapErrorCode::PreconditionFailed => "The precondition given in one or more of the request-header fields evaluated to false when it was tested on the server.",
            IcapErrorCode::RequestEntityTooLarge => "The server is refusing to process a request because the request entity is larger than the server is willing or able to process.",
            IcapErrorCode::RequestUriTooLarge => "The server is refusing to service the request because the Request-URI is longer than the server is willing to interpret.",
            IcapErrorCode::UnsupportedMediaType => "The server is refusing to service the request because the entity of the request is in a format not supported by the requested resource for the requested method.",
            IcapErrorCode::RequestedRangeNotSatisfiable => "The server cannot fulfill the request because the requested range is not satisfiable.",
            IcapErrorCode::ExpectationFailed => "The expectation given in an Expect request-header field could not be met by this server.",

            // 5xx Server Errors
            IcapErrorCode::InternalServerError => "The server encountered an unexpected condition which prevented it from fulfilling the request.",
            IcapErrorCode::NotImplemented => "The server does not support the functionality required to fulfill the request.",
            IcapErrorCode::BadGateway => "The server, while acting as a gateway or proxy, received an invalid response from the upstream server it accessed in attempting to fulfill the request.",
            IcapErrorCode::ServiceUnavailable => "The server is currently unable to handle the request due to a temporary overloading or maintenance of the server.",
            IcapErrorCode::GatewayTimeout => "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server.",
            IcapErrorCode::HttpVersionNotSupported => "The server does not support, or refuses to support, the HTTP protocol version that was used in the request message.",

            // ICAP-specific errors
            IcapErrorCode::InvalidRequest => "The ICAP request is malformed or invalid.",
            IcapErrorCode::InvalidResponse => "The ICAP response is malformed or invalid.",
            IcapErrorCode::InvalidEncapsulated => "The encapsulated data in the ICAP request is malformed or invalid.",
            IcapErrorCode::InvalidPreview => "The preview data in the ICAP request is malformed or invalid.",
            IcapErrorCode::PreviewRequired => "The request requires preview mode processing.",
            IcapErrorCode::Continue => "The server has received the request headers and the client should proceed to send the request body.",
            IcapErrorCode::NoContent => "The server successfully processed the request and is not returning any content.",
        }
    }

    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            IcapErrorCode::BadRequest
                | IcapErrorCode::Unauthorized
                | IcapErrorCode::PaymentRequired
                | IcapErrorCode::Forbidden
                | IcapErrorCode::NotFound
                | IcapErrorCode::MethodNotAllowed
                | IcapErrorCode::NotAcceptable
                | IcapErrorCode::ProxyAuthenticationRequired
                | IcapErrorCode::RequestTimeout
                | IcapErrorCode::Conflict
                | IcapErrorCode::Gone
                | IcapErrorCode::LengthRequired
                | IcapErrorCode::PreconditionFailed
                | IcapErrorCode::RequestEntityTooLarge
                | IcapErrorCode::RequestUriTooLarge
                | IcapErrorCode::UnsupportedMediaType
                | IcapErrorCode::RequestedRangeNotSatisfiable
                | IcapErrorCode::ExpectationFailed
                | IcapErrorCode::InvalidRequest
                | IcapErrorCode::InvalidResponse
                | IcapErrorCode::InvalidEncapsulated
                | IcapErrorCode::InvalidPreview
        )
    }

    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            IcapErrorCode::InternalServerError
                | IcapErrorCode::NotImplemented
                | IcapErrorCode::BadGateway
                | IcapErrorCode::ServiceUnavailable
                | IcapErrorCode::GatewayTimeout
                | IcapErrorCode::HttpVersionNotSupported
        )
    }

    /// Check if this is an informational response (1xx)
    pub fn is_informational(&self) -> bool {
        matches!(
            self,
            IcapErrorCode::PreviewRequired | IcapErrorCode::Continue
        )
    }

    /// Check if this is a successful response (2xx)
    pub fn is_success(&self) -> bool {
        matches!(self, IcapErrorCode::NoContent)
    }
}

impl fmt::Display for IcapErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", *self as u16, self.message())
    }
}

impl From<u16> for IcapErrorCode {
    fn from(code: u16) -> Self {
        match code {
            400 => IcapErrorCode::BadRequest,
            401 => IcapErrorCode::Unauthorized,
            402 => IcapErrorCode::PaymentRequired,
            403 => IcapErrorCode::Forbidden,
            404 => IcapErrorCode::NotFound,
            405 => IcapErrorCode::MethodNotAllowed,
            406 => IcapErrorCode::NotAcceptable,
            407 => IcapErrorCode::ProxyAuthenticationRequired,
            408 => IcapErrorCode::RequestTimeout,
            409 => IcapErrorCode::Conflict,
            410 => IcapErrorCode::Gone,
            411 => IcapErrorCode::LengthRequired,
            412 => IcapErrorCode::PreconditionFailed,
            413 => IcapErrorCode::RequestEntityTooLarge,
            414 => IcapErrorCode::RequestUriTooLarge,
            415 => IcapErrorCode::UnsupportedMediaType,
            416 => IcapErrorCode::RequestedRangeNotSatisfiable,
            417 => IcapErrorCode::ExpectationFailed,
            500 => IcapErrorCode::InternalServerError,
            501 => IcapErrorCode::NotImplemented,
            502 => IcapErrorCode::BadGateway,
            503 => IcapErrorCode::ServiceUnavailable,
            504 => IcapErrorCode::GatewayTimeout,
            505 => IcapErrorCode::HttpVersionNotSupported,
            100 => IcapErrorCode::Continue,
            204 => IcapErrorCode::NoContent,
            _ => IcapErrorCode::InternalServerError,
        }
    }
}

impl From<StatusCode> for IcapErrorCode {
    fn from(status: StatusCode) -> Self {
        IcapErrorCode::from(status.as_u16())
    }
}

/// ICAP error response builder
pub struct IcapErrorResponseBuilder {
    error_code: IcapErrorCode,
    message: Option<String>,
    details: Option<String>,
}

impl IcapErrorResponseBuilder {
    /// Create a new error response builder
    pub fn new(error_code: IcapErrorCode) -> Self {
        Self {
            error_code,
            message: None,
            details: None,
        }
    }

    /// Set custom error message
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Set error details
    pub fn details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// Build the error response
    pub fn build(self) -> IcapErrorResponse {
        IcapErrorResponse {
            error_code: self.error_code,
            message: self.message.unwrap_or_else(|| self.error_code.message().to_string()),
            details: self.details,
        }
    }
}

/// ICAP error response
#[derive(Debug, Clone)]
pub struct IcapErrorResponse {
    pub error_code: IcapErrorCode,
    pub message: String,
    pub details: Option<String>,
}

impl IcapErrorResponse {
    /// Create a new error response
    pub fn new(error_code: IcapErrorCode) -> Self {
        Self {
            error_code,
            message: error_code.message().to_string(),
            details: None,
        }
    }

    /// Create error response with custom message
    pub fn with_message(error_code: IcapErrorCode, message: String) -> Self {
        Self {
            error_code,
            message,
            details: None,
        }
    }

    /// Create error response with details
    pub fn with_details(error_code: IcapErrorCode, message: String, details: String) -> Self {
        Self {
            error_code,
            message,
            details: Some(details),
        }
    }

    /// Get the HTTP status code
    pub fn status_code(&self) -> StatusCode {
        self.error_code.status_code()
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the error details
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    /// Check if this is a client error
    pub fn is_client_error(&self) -> bool {
        self.error_code.is_client_error()
    }

    /// Check if this is a server error
    pub fn is_server_error(&self) -> bool {
        self.error_code.is_server_error()
    }

    /// Check if this is an informational response
    pub fn is_informational(&self) -> bool {
        self.error_code.is_informational()
    }

    /// Check if this is a successful response
    pub fn is_success(&self) -> bool {
        self.error_code.is_success()
    }
}

impl fmt::Display for IcapErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_code)?;
        if let Some(details) = &self.details {
            write!(f, ": {}", details)?;
        }
        Ok(())
    }
}
