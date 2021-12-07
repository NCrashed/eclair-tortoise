
use serde::{Deserialize, Serialize};
use super::node::NodeFeatures;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    pub node_id: String,
    pub channel_id: String,
    pub state: ChannelState,
    pub data: ChannelData,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChannelState {
    Normal,
    Opening,
    Closing,
    Offline,
    Syncing,
}


#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelData {
    pub _type: String,
    pub commitments: ChannelCommitments,
    pub short_channel_id: Option<String>,
    pub buried: Option<bool>,
    pub channel_announcement: Option<ChannelAnnouncement>,
    pub channel_update: Option<ChannelUpdate>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelCommitments {
    pub channel_id: String,
    pub channel_config: Vec<String>,
    pub channel_features: Vec<String>,
    pub local_params: ChannelLocalParams,
    pub remote_params: ChannelRemoteParams,
    pub channel_flags: u64,
    pub local_commit: LocalCommit,
    pub remote_commit: RemoteCommit,
    pub local_changes: ChannelChanges,
    pub remote_changes: ChannelChanges,
    pub local_next_htlc_id: u64,
    pub remote_next_htlc_id: u64,
    // pub origin_channels: ?,
    pub remote_next_commit_info: String,
    pub commit_input: CommitInput,
    pub remote_per_commitment_secrets: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelLocalParams {
    pub node_id: String,
    pub funding_key_path: KeyPath,
    pub dust_limit: u64,
    pub max_htlc_value_in_flight_msat: u64,
    pub channel_reserve: u64,
    pub htlc_minimum: u32,
    pub to_self_delay: u32,
    pub max_accepted_htlcs: u32,
    pub is_funder: bool,
    pub default_final_script_pub_key: String,
    pub init_features: NodeFeatures,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyPath {
    pub path: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelRemoteParams {
    pub node_id: String,
    pub dust_limit: u64,
    pub max_htlc_value_in_flight_msat: u64,
    pub channel_reserve: u64,
    pub htlc_minimum: u32,
    pub to_self_delay: u32,
    pub max_accepted_htlcs: u32,
    pub funding_pub_key: String,
    pub revocation_basepoint: String,
    pub payment_basepoint: String,
    pub delayed_payment_basepoint: String,
    pub htlc_basepoint: String,
    pub init_features: NodeFeatures,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalCommit {
    pub index: u32,
    pub spec: CommitSpec,
    pub commit_tx_and_remote_sig: CommitTxAndSign,
    pub htlc_txs_and_remote_sigs: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommitSpec {
    pub htlcs: Vec<String>,
    pub commit_tx_feerate: u64,
    pub to_local: u64,
    pub to_remote: u64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommitTxAndSign {
    pub commit_tx: CommitTx,
    pub remote_sig: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommitTx {
    pub txid: String,
    pub tx: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteCommit {
    pub index: u32,
    pub spec: CommitSpec,
    pub txid: String,
    pub remote_per_commitment_point: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelChanges {
    pub proposed: Vec<String>,
    pub signed: Vec<String>,
    pub acked: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommitInput {
    pub out_point: String,
    pub amount_satoshis: u64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAnnouncement {
    pub node_signature1: String,
    pub node_signature2: String,
    pub bitcoin_signature1: String,
    pub bitcoin_signature2: String,
    pub features: NodeFeatures,
    pub chain_hash: String,
    pub short_channel_id: String,
    pub node_id1: String,
    pub node_id2: String,
    pub bitcoin_key1: String,
    pub bitcoin_key2: String,
    pub tlv_stream: TlvStream,
}


#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TlvStream {
    pub records: Vec<String>,
    pub unknown: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelUpdate {
    pub signature: String,
    pub chain_hash: String,
    pub short_channel_id: String,
    pub timestamp: u64,
    pub channel_flags: ChannelFlags,
    pub cltv_expiry_delta: u32,
    pub htlc_minimum_msat: u64,
    pub htlc_maximum_msat: u64,
    pub fee_base_msat: u64,
    pub fee_proportional_millionths: u64,
    pub tlv_stream: TlvStream,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelFlags {
    pub is_enabled: bool,
    pub is_node1: bool,
}