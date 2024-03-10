#![allow(clippy::redundant_closure)]
use revm::{
    db::{PlainAccount},
    primitives::{Address, B256, U256, keccak256},
};

use alloy_rlp::{RlpEncodable, RlpMaxEncodedLen};
use triehash::sec_trie_root;
use hash_db::Hasher;
use plain_hasher::PlainHasher;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeccakHasher;

impl Hasher for KeccakHasher {
    type Out = B256;
    type StdHasher = PlainHasher;
    const LENGTH: usize = 32;

    #[inline]
    fn hash(x: &[u8]) -> Self::Out {
        keccak256(x)
    }
}

#[derive(RlpEncodable, RlpMaxEncodedLen)]
pub struct TrieAccount {
    nonce: u64,
    balance: U256,
    root_hash: B256,
    code_hash: B256,
}

impl TrieAccount {
    fn new(acc: &PlainAccount) -> Self {
        Self {
            nonce: acc.info.nonce,
            balance: acc.info.balance,
            root_hash: sec_trie_root::<KeccakHasher, _, _, _>(
                acc.storage
                    .iter()
                    .filter(|(_k, &v)| v != U256::ZERO)
                    .map(|(k, v)| (k.to_be_bytes::<32>(), alloy_rlp::encode_fixed_size(v))),
            ),
            code_hash: acc.info.code_hash,
        }
    }
}

#[inline]
pub fn trie_root<I, A, B>(input: I) -> B256
where
    I: IntoIterator<Item = (A, B)>,
    A: AsRef<[u8]>,
    B: AsRef<[u8]>,
{
    sec_trie_root::<KeccakHasher, _, _, _>(input)
}

pub fn state_merkle_trie_root(
    accounts: impl IntoIterator<Item = (Address, PlainAccount)>,
) -> B256 {
    trie_root(accounts.into_iter().map(|(address, acc)| {
        (
            address,
            alloy_rlp::encode_fixed_size(&TrieAccount::new(&acc)),
        )
    }))
}

