///`LzCallParams(address,address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct LzCallParams {
    pub refund_address: ::ethers::core::types::Address,
    pub zro_payment_address: ::ethers::core::types::Address,
    pub adapter_params: ::ethers::core::types::Bytes,
}
///`ValidatorData(address,bytes,bytes32,bytes,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ValidatorData {
    pub operator: ::ethers::core::types::Address,
    pub pubkey: ::ethers::core::types::Bytes,
    pub withdrawal_credentials: [u8; 32],
    pub signature: ::ethers::core::types::Bytes,
    pub deposit_data_root: [u8; 32],
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
