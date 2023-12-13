// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};

use crate::wal::kafka::KafkaOptions;
use crate::wal::raft_engine::RaftEngineOptions;

pub mod kafka;
pub mod raft_engine;

// TODO(niebayes): maybe rename all wal-related options to config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalConfig {
    RaftEngine(RaftEngineOptions),
    Kafka(KafkaOptions),
}

impl Default for WalConfig {
    fn default() -> Self {
        WalConfig::RaftEngine(RaftEngineOptions::default())
    }
}
