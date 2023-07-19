/*
 * Copyright 2023 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::path::PathBuf;

use anyhow::anyhow;
use clap::ArgMatches;

use g3keymess_proto::proc_capnp::proc_control;
use g3keymess_proto::server_capnp::server_control;

use super::{CommandError, CommandResult};
use crate::common::{parse_fetch_result, parse_operation_result, print_list_text};

pub const COMMAND_VERSION: &str = "version";
pub const COMMAND_OFFLINE: &str = "offline";
pub const COMMAND_LIST: &str = "list";
pub const COMMAND_PUBLISH_KEY: &str = "publish-key";

const COMMAND_LIST_ARG_RESOURCE: &str = "resource";
const RESOURCE_VALUE_SERVER: &str = "server";

const COMMAND_ARG_FILE: &str = "file";

pub mod commands {
    use super::*;
    use clap::{value_parser, Arg, Command, ValueHint};

    pub fn version() -> Command {
        Command::new(COMMAND_VERSION)
    }

    pub fn offline() -> Command {
        Command::new(COMMAND_OFFLINE).about("Put this daemon into offline mode")
    }

    pub fn list() -> Command {
        Command::new(COMMAND_LIST).arg(
            Arg::new(COMMAND_LIST_ARG_RESOURCE)
                .required(true)
                .num_args(1)
                .value_parser([RESOURCE_VALUE_SERVER])
                .ignore_case(true),
        )
    }

    pub fn publish_key() -> Command {
        Command::new(COMMAND_PUBLISH_KEY).arg(
            Arg::new(COMMAND_ARG_FILE)
                .help("Private key file in pem format")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(PathBuf))
                .value_hint(ValueHint::FilePath),
        )
    }
}

pub async fn version(client: &proc_control::Client) -> CommandResult<()> {
    let req = client.version_request();
    let rsp = req.send().promise.await?;
    let ver = rsp.get()?.get_version()?;
    println!("{ver}");
    Ok(())
}

pub async fn offline(client: &proc_control::Client) -> CommandResult<()> {
    let req = client.offline_request();
    let rsp = req.send().promise.await?;
    parse_operation_result(rsp.get()?.get_result()?)
}

pub async fn list(client: &proc_control::Client, args: &ArgMatches) -> CommandResult<()> {
    match args
        .get_one::<String>(COMMAND_LIST_ARG_RESOURCE)
        .unwrap()
        .as_str()
    {
        RESOURCE_VALUE_SERVER => list_server(client).await,
        _ => unreachable!(),
    }
}

async fn list_server(client: &proc_control::Client) -> CommandResult<()> {
    let req = client.list_server_request();
    let rsp = req.send().promise.await?;
    print_list_text(rsp.get()?.get_result()?)
}

pub(crate) async fn get_server(
    client: &proc_control::Client,
    name: &str,
) -> CommandResult<server_control::Client> {
    let mut req = client.get_server_request();
    req.get().set_name(name);
    let rsp = req.send().promise.await?;
    parse_fetch_result(rsp.get()?.get_server()?)
}

pub async fn publish_key(client: &proc_control::Client, args: &ArgMatches) -> CommandResult<()> {
    let file = args.get_one::<PathBuf>(COMMAND_ARG_FILE).unwrap();
    let content = std::fs::read_to_string(file).map_err(|e| {
        CommandError::Cli(anyhow!(
            "failed to read content of file {}: {e}",
            file.display()
        ))
    })?;
    let mut req = client.publish_key_request();
    req.get().set_pem(&content);
    let rsp = req.send().promise.await?;
    parse_operation_result(rsp.get()?.get_result()?)
}
