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

use std::sync::Arc;

use common_error::ext::BoxedError;
use common_recordbatch::SendableRecordBatchStream;

use crate::storage::ScanRequest;

/// This trait represents a common data source abstraction which provides an interface
/// for retrieving data in the form of a stream of record batches.
pub trait DataSource {
    /// Retrieves a stream of record batches based on the provided scan request.
    fn get_stream(&self, request: ScanRequest) -> Result<SendableRecordBatchStream, BoxedError>;
}

pub type DataSourceRef = Arc<dyn DataSource>;

pub type TableFactory = Arc<dyn Fn() -> DataSourceRef>;