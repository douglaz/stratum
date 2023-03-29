#[cfg(not(feature = "with_serde"))]
use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2;
use binary_sv2::{Deserialize, Seq064K, Serialize, ShortTxId, B0255, B064K, U256};
use core::convert::TryInto;

/// ## CommitMiningJob (Client -> Server)
/// A request sent by the Job Negotiator that proposes a selected set of transactions to the upstream (pool) node.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct CommitMiningJob<'decoder> {
    pub request_id: u32,
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub mining_job_token: B0255<'decoder>,
    pub version: u32,
    pub coninbase_tx_version: u32,
    pub coninbase_prefix: B0255<'decoder>,
    pub coninbase_tx_input_nsequence: u32,
    pub coninbase_tx_value_remaining: u64,
    pub coinbase_tx_outputs: B064K<'decoder>,
    pub coinbase_tx_locktime: u32,
    pub min_extranonce_size: u16,
    pub tx_short_hash_nonce: u64,
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub tx_short_hash_list: Seq064K<'decoder, ShortTxId<'decoder>>,
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub tx_hash_list_hash: U256<'decoder>,
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub excess_data: B064K<'decoder>,
}

/// ## CommitMiningJob (Server -> Client)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct CommitMiningJobSuccess<'decoder> {
    pub request_id: u32,
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub new_mining_job_token: B0255<'decoder>,
}

#[cfg(feature = "with_serde")]
use binary_sv2::GetSize;
#[cfg(feature = "with_serde")]
impl<'d> GetSize for CommitMiningJob<'d> {
    fn get_size(&self) -> usize {
        self.request_id.get_size()
            + self.mining_job_token.get_size()
            + self.version.get_size()
            + self.coninbase_tx_version.get_size()
            + self.coninbase_prefix.get_size()
            + self.coninbase_tx_input_nsequence.get_size()
            + self.coninbase_tx_value_remaining.get_size()
            + self.coinbase_tx_outputs.get_size()
            + self.coinbase_tx_locktime.get_size()
            + self.min_extranonce_size.get_size()
            + self.tx_short_hash_nonce.get_size()
            + self.tx_short_hash_list.get_size()
            + self.tx_hash_list_hash.get_size()
            + self.excess_data.get_size()
    }
}
#[cfg(feature = "with_serde")]
impl<'d> GetSize for CommitMiningJobSuccess<'d> {
    fn get_size(&self) -> usize {
        self.request_id.get_size() + self.new_mining_job_token.get_size()
    }
}
