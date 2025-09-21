/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use slog::{Logger, slog_o};

use super::shared::SharedLoggerType;

pub(crate) fn get_logger(connection_id: &str) -> Option<Logger> {
    let config = crate::config::log::get_connection_default_config();
    let logger_name = format!("lc-{connection_id}");
    let common_values = slog_o!(
        "daemon_name" => crate::opts::daemon_group(),
        "log_type" => super::LOG_TYPE_CONNECTION,
        "pid" => std::process::id(),
        "connection_id" => connection_id.to_string(),
    );
    config.build_logger(logger_name, super::LOG_TYPE_CONNECTION, common_values)
}

#[allow(dead_code)]
pub(crate) fn get_shared_logger(
    name: &str,
    connection_id: &str,
) -> Option<Logger> {
    let logger_name = format!("lc-{name}");
    super::shared::get_shared_logger(SharedLoggerType::Connection, logger_name, |logger| {
        logger.new(slog_o!(
            "connection_id" => connection_id.to_string(),
        ))
    })
}

#[allow(dead_code)]
pub(crate) enum ConnectionEvent {
    Accepted,
    RequestReceived,
    ResponseSent,
    Error,
    Closed,
}

impl ConnectionEvent {
    pub(crate) fn log(&self, logger: &Logger, message: &str) {
        match self {
            ConnectionEvent::Accepted => {
                slog::info!(logger, "{}", message; "event" => "accepted");
            }
            ConnectionEvent::RequestReceived => {
                slog::debug!(logger, "{}", message; "event" => "request_received");
            }
            ConnectionEvent::ResponseSent => {
                slog::debug!(logger, "{}", message; "event" => "response_sent");
            }
            ConnectionEvent::Error => {
                slog::error!(logger, "{}", message; "event" => "error");
            }
            ConnectionEvent::Closed => {
                slog::info!(logger, "{}", message; "event" => "closed");
            }
        }
    }
}
