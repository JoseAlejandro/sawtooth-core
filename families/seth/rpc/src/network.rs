/*
 * Copyright 2017 Intel Corporation
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
 * ------------------------------------------------------------------------------
 */

use jsonrpc_core::{Params, Value, Error};

use sawtooth_sdk::messaging::zmq_stream::ZmqMessageSender;
use super::error;

pub fn version(_params: Params, mut _sender: ZmqMessageSender) -> Result<Value, Error> {
    Ok(Value::String(String::from(SAWTOOTH_NET_VERSION)))
}
pub fn peer_count(_params: Params, mut _sender: ZmqMessageSender) -> Result<Value, Error> {
    Err(error::not_implemented())
}
pub fn listening(_params: Params, mut _sender: ZmqMessageSender) -> Result<Value, Error> {
    Err(error::not_implemented())
}
