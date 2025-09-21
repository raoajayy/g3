/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

mod shared;

pub(crate) mod connection;
pub(crate) mod server;

const LOG_TYPE_CONNECTION: &str = "Connection";
const LOG_TYPE_SERVER: &str = "Server";

use slog::{Logger, slog_o};

use crate::opts::daemon_group;

#[allow(dead_code)]
pub(crate) fn get_connection_logger(connection_id: &str) -> Option<Logger> {
    let config = crate::config::log::get_connection_default_config();
    let logger_name = format!("lc-{connection_id}");
    let common_values = slog_o!(
        "daemon_name" => daemon_group(),
        "log_type" => LOG_TYPE_CONNECTION,
        "pid" => std::process::id(),
        "connection_id" => connection_id.to_string(),
    );
    config.build_logger(logger_name, LOG_TYPE_CONNECTION, common_values)
}

#[allow(dead_code)]
pub(crate) fn get_server_logger(server_name: &str) -> Option<Logger> {
    let config = crate::config::log::get_server_default_config();
    let logger_name = format!("ls-{server_name}");
    let common_values = slog_o!(
        "daemon_name" => daemon_group(),
        "log_type" => LOG_TYPE_SERVER,
        "pid" => std::process::id(),
        "server_name" => server_name.to_string(),
    );
    config.build_logger(logger_name, LOG_TYPE_SERVER, common_values)
}

#[allow(dead_code)]
pub(crate) enum ConnectionEvent {
    Accepted,
    RequestReceived,
    ResponseSent,
    Error,
    Closed,
}

#[allow(dead_code)]
pub(crate) enum ServerEvent {
    Started,
    Stopped,
    ServiceRegistered,
    ServiceUnregistered,
    Error,
}
