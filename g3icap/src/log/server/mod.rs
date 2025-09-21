/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use slog::{Logger, slog_o};

use super::shared::SharedLoggerType;

pub(crate) fn get_logger(server_name: &str) -> Option<Logger> {
    let config = crate::config::log::get_server_default_config();
    let logger_name = format!("ls-{server_name}");
    let common_values = slog_o!(
        "daemon_name" => crate::opts::daemon_group(),
        "log_type" => super::LOG_TYPE_SERVER,
        "pid" => std::process::id(),
        "server_name" => server_name.to_string(),
    );
    config.build_logger(logger_name, super::LOG_TYPE_SERVER, common_values)
}

#[allow(dead_code)]
pub(crate) fn get_shared_logger(
    name: &str,
    server_name: &str,
) -> Option<Logger> {
    let logger_name = format!("ls-{name}");
    super::shared::get_shared_logger(SharedLoggerType::Server, logger_name, |logger| {
        logger.new(slog_o!(
            "server_name" => server_name.to_string(),
        ))
    })
}

#[allow(dead_code)]
pub(crate) enum ServerEvent {
    Started,
    Stopped,
    ServiceRegistered,
    ServiceUnregistered,
    Error,
}

impl ServerEvent {
    pub(crate) fn log(&self, logger: &Logger, message: &str) {
        match self {
            ServerEvent::Started => {
                slog::info!(logger, "{}", message; "event" => "started");
            }
            ServerEvent::Stopped => {
                slog::info!(logger, "{}", message; "event" => "stopped");
            }
            ServerEvent::ServiceRegistered => {
                slog::info!(logger, "{}", message; "event" => "service_registered");
            }
            ServerEvent::ServiceUnregistered => {
                slog::info!(logger, "{}", message; "event" => "service_unregistered");
            }
            ServerEvent::Error => {
                slog::error!(logger, "{}", message; "event" => "error");
            }
        }
    }
}
