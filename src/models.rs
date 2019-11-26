/*
 * Copyright 2019 Cargill Incorporated
 * Copyright 2019 Walmart Inc.
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
 * -----------------------------------------------------------------------------
 */

use std::time::SystemTime;

pub struct ConsortiumUser {
    pub email: String,
    pub public_key: String,
    pub encrypted_private_key: String,
    pub hashed_password: String,
}

pub struct Consortium {
    pub circuit_id: String,
    pub authorization_type: String,
    pub persistence: String,
    pub durability: String,
    pub routes: String,
    pub circuit_management_type: String,
    pub alias: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

pub struct ConsortiumProposal {
    pub id: i64,
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: String,
    pub requester_node_id: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

pub struct NewConsortiumProposal {
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: String,
    pub requester_node_id: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

pub struct ProposalVoteRecord {
    pub id: i64,
    pub proposal_id: i64,
    pub voter_public_key: String,
    pub voter_node_id: String,
    pub vote: String,
    pub created_time: SystemTime,
}

pub struct NewProposalVoteRecord {
    pub proposal_id: i64,
    pub voter_public_key: String,
    pub voter_node_id: String,
    pub vote: String,
    pub created_time: SystemTime,
}

pub struct ConsortiumMember {
    pub id: i64,
    pub circuit_id: String,
    pub node_id: String,
    pub endpoint: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

pub struct NewConsortiumMember {
    pub circuit_id: String,
    pub node_id: String,
    pub endpoint: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

pub struct ConsortiumService {
    pub id: i64,
    pub circuit_id: String,
    pub service_id: String,
    pub service_type: String,
    pub allowed_nodes: Vec<String>,
    pub arguments: Vec<serde_json::Value>,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

pub struct NewConsortiumService {
    pub circuit_id: String,
    pub service_id: String,
    pub service_type: String,
    pub allowed_nodes: Vec<String>,
    pub arguments: Vec<serde_json::Value>,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

pub struct ConsortiumNotification {
    pub id: i64,
    pub notification_type: String,
    pub requester: String,
    pub requester_node_id: String,
    pub target: String,
    pub created_time: SystemTime,
    pub read: bool,
}

pub struct NewConsortiumNotification {
    pub notification_type: String,
    pub requester: String,
    pub requester_node_id: String,
    pub target: String,
    pub created_time: SystemTime,
    pub read: bool,
}

pub struct ActiveConsortium {
    pub circuit_id: String,
    pub service_id: String,
    pub status: String,
    pub requester: String,
    pub requester_node_id: String,
}
