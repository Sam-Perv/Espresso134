// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the Espresso library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use espresso_core::state::{
    state_comm::LedgerStateCommitment, BlockCommitment, ElaboratedBlock, TransactionCommitment,
    ValidatorState,
};

pub struct BlockQueryData {
    pub raw_block: ElaboratedBlock,
    pub block_hash: BlockCommitment,
    pub block_id: u64,
    pub records_from: u64,
    pub record_count: u64,
    pub txn_hashes: Vec<TransactionCommitment>,
}

pub struct StateQueryData {
    pub state: ValidatorState,
    pub commitment: LedgerStateCommitment,
    pub block_id: u64,
    pub event_index: u64,
}