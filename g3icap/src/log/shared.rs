/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::collections::HashMap;
use std::sync::Mutex;

use foldhash::fast::FixedState;
use slog::Logger;

#[allow(dead_code)]
static SHARED_LOGGER: Mutex<HashMap<String, Option<Logger>, FixedState>> =
    Mutex::new(HashMap::with_hasher(FixedState::with_seed(0)));

#[allow(dead_code)]
pub(super) enum SharedLoggerType {
    Connection,
    Server,
}

#[allow(dead_code)]
pub(super) fn get_shared_logger<F>(
    logger_type: SharedLoggerType,
    logger_name: String,
    sub_logger: F,
) -> Option<Logger>
where
    F: Fn(&Logger) -> Logger,
{
    let (config, log_type) = match logger_type {
        SharedLoggerType::Connection => (
            crate::config::log::get_connection_default_config(),
            super::LOG_TYPE_CONNECTION,
        ),
        SharedLoggerType::Server => (
            crate::config::log::get_server_default_config(),
            super::LOG_TYPE_SERVER,
        ),
    };
    let mut container = SHARED_LOGGER.lock().unwrap();
    let logger = container
        .entry(format!("{log_type}/{logger_name}"))
        .or_insert_with(|| {
            config.build_shared_logger(logger_name, crate::opts::daemon_group(), log_type)
        });
    logger.as_ref().map(sub_logger)
}
