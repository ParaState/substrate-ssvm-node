use crate::{AccountCodes, AccountStorages, Accounts, Event, Module, Trait};
use codec::{Decode, Encode};
use evm::backend::{Apply, ApplyBackend, Backend as BackendT};
use frame_support::storage::{StorageDoubleMap, StorageMap};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use sp_core::{H160, H256, U256};
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::if_std;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use ssvm::{
    Address, Bytes, Bytes32, CallKind, HostInterface, StatusCode, StorageStatus, ADDRESS_LENGTH,
    BYTES32_LENGTH,
};

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
/// Ethereum account nonce, balance and code. Used by storage.
pub struct Account {
    /// Account nonce.
    pub nonce: U256,
    /// Account balance.
    pub balance: U256,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
/// Ethereum log. Used for `deposit_event`.
pub struct Log {
    /// Source address of the log.
    pub address: H160,
    /// Topics of the log.
    pub topics: Vec<H256>,
    /// Byte array data of the log.
    pub data: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
/// External input from the transaction.
pub struct Vicinity {
    /// Current transaction gas price.
    pub gas_price: U256,
    /// Origin of the transaction.
    pub origin: H160,
}

/// Substrate backend for EVM.
pub struct Backend<'vicinity, T> {
    vicinity: &'vicinity Vicinity,
    _marker: PhantomData<T>,
}

impl<'vicinity, T> Backend<'vicinity, T> {
    /// Create a new backend with given vicinity.
    pub fn new(vicinity: &'vicinity Vicinity) -> Self {
        Self {
            vicinity,
            _marker: PhantomData,
        }
    }
}

impl<'vicinity, T: Trait> BackendT for Backend<'vicinity, T> {
    fn gas_price(&self) -> U256 {
        self.vicinity.gas_price
    }
    fn origin(&self) -> H160 {
        self.vicinity.origin
    }

    fn block_hash(&self, number: U256) -> H256 {
        if number > U256::from(u32::max_value()) {
            H256::default()
        } else {
            let number = T::BlockNumber::from(number.as_u32());
            H256::from_slice(frame_system::Module::<T>::block_hash(number).as_ref())
        }
    }

    fn block_number(&self) -> U256 {
        let number: u128 = frame_system::Module::<T>::block_number().unique_saturated_into();
        U256::from(number)
    }

    fn block_coinbase(&self) -> H160 {
        H160::default()
    }

    fn block_timestamp(&self) -> U256 {
        let now: u128 = pallet_timestamp::Module::<T>::get().unique_saturated_into();
        U256::from(now)
    }

    fn block_difficulty(&self) -> U256 {
        U256::zero()
    }

    fn block_gas_limit(&self) -> U256 {
        U256::zero()
    }

    fn chain_id(&self) -> U256 {
        U256::from(sp_io::misc::chain_id())
    }

    fn exists(&self, _address: H160) -> bool {
        true
    }

    fn basic(&self, address: H160) -> evm::backend::Basic {
        let account = Accounts::get(&address);

        evm::backend::Basic {
            balance: account.balance,
            nonce: account.nonce,
        }
    }

    fn code_size(&self, address: H160) -> usize {
        AccountCodes::decode_len(&address).unwrap_or(0)
    }

    fn code_hash(&self, address: H160) -> H256 {
        H256::from_slice(Keccak256::digest(&AccountCodes::get(&address)).as_slice())
    }

    fn code(&self, address: H160) -> Vec<u8> {
        AccountCodes::get(&address)
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        AccountStorages::get(address, index)
    }
}

impl<'vicinity, T: Trait> ApplyBackend for Backend<'vicinity, T> {
    fn apply<A, I, L>(&mut self, values: A, logs: L, delete_empty: bool)
    where
        A: IntoIterator<Item = Apply<I>>,
        I: IntoIterator<Item = (H256, H256)>,
        L: IntoIterator<Item = evm::backend::Log>,
    {
        for apply in values {
            match apply {
                Apply::Modify {
                    address,
                    basic,
                    code,
                    storage,
                    reset_storage,
                } => {
                    Accounts::mutate(&address, |account| {
                        account.balance = basic.balance;
                        account.nonce = basic.nonce;
                    });

                    if let Some(code) = code {
                        AccountCodes::insert(address, code);
                    }

                    if reset_storage {
                        AccountStorages::remove_prefix(address);
                    }

                    for (index, value) in storage {
                        if value == H256::default() {
                            AccountStorages::remove(address, index);
                        } else {
                            AccountStorages::insert(address, index, value);
                        }
                    }

                    if delete_empty {
                        Module::<T>::remove_account_if_empty(&address);
                    }
                }
                Apply::Delete { address } => Module::<T>::remove_account(&address),
            }
        }

        for log in logs {
            Module::<T>::deposit_event(Event::Log(Log {
                address: log.address,
                topics: log.topics,
                data: log.data,
            }));
        }
    }
}

pub fn create_address(caller: H160, nonce: U256) -> H160 {
    let mut stream = rlp::RlpStream::new_list(2);
    stream.append(&caller);
    stream.append(&nonce);
    H256::from_slice(Keccak256::digest(&stream.out()).as_slice()).into()
}

pub struct TxContext {
    tx_gas_price: U256,
    tx_origin: H160,
    block_coinbase: H160,
    block_number: i64,
    block_timestamp: i64,
    block_gas_limit: i64,
    block_difficulty: U256,
    chain_id: U256,
}

impl TxContext {
    pub fn new(
        tx_gas_price: U256,
        tx_origin: H160,
        block_coinbase: H160,
        block_number: i64,
        block_timestamp: i64,
        block_gas_limit: i64,
        block_difficulty: U256,
        chain_id: U256,
    ) -> Self {
        Self {
            tx_gas_price,
            tx_origin,
            block_coinbase,
            block_number,
            block_timestamp,
            block_gas_limit,
            block_difficulty,
            chain_id,
        }
    }
}

#[cfg(feature = "std")]
pub struct HostContext<T> {
    tx_context: TxContext,
    _marker: PhantomData<T>,
}

#[cfg(feature = "std")]
impl<T> HostContext<T> {
    pub fn new(tx_context: TxContext) -> Self {
        Self {
            tx_context,
            _marker: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<T: Trait> HostInterface for HostContext<T> {
    fn account_exists(&mut self, _addr: &[u8; 20]) -> bool {
        println!("Host: account_exists");
        true
    }
    fn get_storage(&mut self, address: &Address, key: &Bytes32) -> Bytes32 {
        println!("Host: get_storage {:?}", hex::encode(address));
        let ret =
            Module::<T>::get_storage(H160::from(address.to_owned()), H256::from(key.to_owned()));
        println!(
            "{:?} -> {:?}",
            hex::encode(key),
            hex::encode(ret.as_fixed_bytes())
        );
        ret.to_fixed_bytes()
    }
    fn set_storage(&mut self, address: &Address, key: &Bytes32, value: &Bytes32) -> StorageStatus {
        println!("Host: set_storage {:?}", hex::encode(address));
        println!("{:?} -> {:?}", hex::encode(key), hex::encode(value));
        Module::<T>::set_storage(
            H160::from(address.to_owned()),
            H256::from(key.to_owned()),
            H256::from(value.to_owned()),
        );
        StorageStatus::EVMC_STORAGE_MODIFIED
    }
    fn get_balance(&mut self, address: &Address) -> Bytes32 {
        let balance = Accounts::get(H160::from(address.to_owned())).balance;
        println!("Host: get_balance {:?}", hex::encode(address));
        println!("balance[{:?}] = {:?}", hex::encode(address), balance);
        balance.into()
    }
    fn get_code_size(&mut self, address: &Address) -> usize {
        AccountCodes::decode_len(H160::from(address)).unwrap_or(0)
    }
    fn get_code_hash(&mut self, address: &Address) -> Bytes32 {
        H256::from_slice(Keccak256::digest(&AccountCodes::get(H160::from(address))).as_slice())
            .into()
    }
    fn copy_code(
        &mut self,
        _addr: &Address,
        _offset: &usize,
        _buffer_data: &*mut u8,
        _buffer_size: &usize,
    ) -> usize {
        println!("Host: copy_code");
        return 0;
    }
    fn selfdestruct(&mut self, _addr: &Address, _beneficiary: &Address) {
        println!("Host: selfdestruct");
    }
    fn get_tx_context(&mut self) -> (Bytes32, Address, Address, i64, i64, i64, Bytes32) {
        println!("Host: get_tx_context");
        (
            self.tx_context.tx_gas_price.into(),
            self.tx_context.tx_origin.to_fixed_bytes(),
            self.tx_context.block_coinbase.to_fixed_bytes(),
            self.tx_context.block_number,
            self.tx_context.block_timestamp,
            self.tx_context.block_gas_limit,
            self.tx_context.block_difficulty.into(),
        )
    }
    fn get_block_hash(&mut self, block_number: i64) -> Bytes32 {
        let number = U256::from(block_number);
        if number > U256::from(u32::max_value()) {
            H256::default().into()
        } else {
            let number = T::BlockNumber::from(number.as_u32());
            H256::from_slice(frame_system::Module::<T>::block_hash(number).as_ref()).into()
        }
    }
    fn emit_log(&mut self, address: &Address, topics: &Vec<Bytes32>, data: &Bytes) {
        Module::<T>::deposit_event(Event::Log(Log {
            address: H160::from(address.to_owned()),
            topics: topics
                .iter()
                .map(|b32| H256::from(b32))
                .collect::<Vec<H256>>(),
            data: data.to_vec(),
        }));
    }
    fn call(
        &mut self,
        _kind: CallKind,
        _destination: &Address,
        _sender: &Address,
        _value: &Bytes32,
        _input: &[u8],
        _gas: i64,
        _depth: i32,
        _is_static: bool,
    ) -> (Vec<u8>, i64, Address, StatusCode) {
        println!("Host: call");
        let (output, gas_left, status_code) = Module::<T>::execute_ssvm(
            _sender.into(),
            _destination.into(),
            _value.into(),
            _input.to_vec(),
            _gas as u32,
            self.tx_context.tx_gas_price.into(),
            Accounts::get(H160::from(_sender)).nonce,
            _kind,
        )
        .unwrap();
        return (output, gas_left, [0u8; ADDRESS_LENGTH], status_code);
    }
}
