pub use mev_eth_errors::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod mev_eth_errors {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyClaimed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadyClaimed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyFinalised"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadyFinalised"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadyInitialized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositTooSmall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DepositTooSmall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositWasFrontrun"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DepositWasFrontrun"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeesTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FeesTooHigh"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IndexExceedsQueueLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IndexExceedsQueueLength",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidOperator"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPendingMevEthShareVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidPendingMevEthShareVault",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPendingStakingModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidPendingStakingModule",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSender"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NonZeroVaultBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NonZeroVaultBalance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughEth"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotFinalised"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotFinalised"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "PrematureMevEthShareVaultUpdateFinalization",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PrematureMevEthShareVaultUpdateFinalization",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "PrematureStakingModuleUpdateFinalization",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PrematureStakingModuleUpdateFinalization",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SandwichProtection"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SandwichProtection"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SendError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SendError"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakingPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StakingPaused"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferExceedsAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferExceedsAllowance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TransferFailed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnAuthorizedCaller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnAuthorizedCaller"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawTooSmall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("WithdrawTooSmall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongDepositAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("WrongDepositAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongWithdrawAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WrongWithdrawAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroValue"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MEVETHERRORS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct MevEthErrors<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MevEthErrors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MevEthErrors<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MevEthErrors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MevEthErrors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MevEthErrors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MevEthErrors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MEVETHERRORS_ABI.clone(),
                    client,
                ),
            )
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MevEthErrors<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyClaimed` with signature `AlreadyClaimed()` and selector `0x646cf558`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AlreadyClaimed", abi = "AlreadyClaimed()")]
    pub struct AlreadyClaimed;
    ///Custom Error type `AlreadyFinalised` with signature `AlreadyFinalised()` and selector `0x26b7f2fe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AlreadyFinalised", abi = "AlreadyFinalised()")]
    pub struct AlreadyFinalised;
    ///Custom Error type `AlreadyInitialized` with signature `AlreadyInitialized()` and selector `0x0dc149f0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AlreadyInitialized", abi = "AlreadyInitialized()")]
    pub struct AlreadyInitialized;
    ///Custom Error type `DepositTooSmall` with signature `DepositTooSmall()` and selector `0x6ba4a1c7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DepositTooSmall", abi = "DepositTooSmall()")]
    pub struct DepositTooSmall;
    ///Custom Error type `DepositWasFrontrun` with signature `DepositWasFrontrun()` and selector `0xc61cf4c5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DepositWasFrontrun", abi = "DepositWasFrontrun()")]
    pub struct DepositWasFrontrun;
    ///Custom Error type `FeesTooHigh` with signature `FeesTooHigh()` and selector `0xc9034e18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FeesTooHigh", abi = "FeesTooHigh()")]
    pub struct FeesTooHigh;
    ///Custom Error type `IndexExceedsQueueLength` with signature `IndexExceedsQueueLength()` and selector `0x4b4a6954`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "IndexExceedsQueueLength", abi = "IndexExceedsQueueLength()")]
    pub struct IndexExceedsQueueLength;
    ///Custom Error type `InvalidOperator` with signature `InvalidOperator()` and selector `0xccea9e6f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidOperator", abi = "InvalidOperator()")]
    pub struct InvalidOperator;
    ///Custom Error type `InvalidPendingMevEthShareVault` with signature `InvalidPendingMevEthShareVault()` and selector `0x864db37e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidPendingMevEthShareVault",
        abi = "InvalidPendingMevEthShareVault()"
    )]
    pub struct InvalidPendingMevEthShareVault;
    ///Custom Error type `InvalidPendingStakingModule` with signature `InvalidPendingStakingModule()` and selector `0x817ae115`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidPendingStakingModule",
        abi = "InvalidPendingStakingModule()"
    )]
    pub struct InvalidPendingStakingModule;
    ///Custom Error type `InvalidSender` with signature `InvalidSender()` and selector `0xddb5de5e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSender", abi = "InvalidSender()")]
    pub struct InvalidSender;
    ///Custom Error type `NonZeroVaultBalance` with signature `NonZeroVaultBalance()` and selector `0x77e8e5de`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NonZeroVaultBalance", abi = "NonZeroVaultBalance()")]
    pub struct NonZeroVaultBalance;
    ///Custom Error type `NotEnoughEth` with signature `NotEnoughEth()` and selector `0xf14a42b7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotEnoughEth", abi = "NotEnoughEth()")]
    pub struct NotEnoughEth;
    ///Custom Error type `NotFinalised` with signature `NotFinalised()` and selector `0x1ba17204`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotFinalised", abi = "NotFinalised()")]
    pub struct NotFinalised;
    ///Custom Error type `PrematureMevEthShareVaultUpdateFinalization` with signature `PrematureMevEthShareVaultUpdateFinalization()` and selector `0xd29a06f8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "PrematureMevEthShareVaultUpdateFinalization",
        abi = "PrematureMevEthShareVaultUpdateFinalization()"
    )]
    pub struct PrematureMevEthShareVaultUpdateFinalization;
    ///Custom Error type `PrematureStakingModuleUpdateFinalization` with signature `PrematureStakingModuleUpdateFinalization()` and selector `0x8399e638`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "PrematureStakingModuleUpdateFinalization",
        abi = "PrematureStakingModuleUpdateFinalization()"
    )]
    pub struct PrematureStakingModuleUpdateFinalization;
    ///Custom Error type `SandwichProtection` with signature `SandwichProtection()` and selector `0x7ef2d89b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SandwichProtection", abi = "SandwichProtection()")]
    pub struct SandwichProtection;
    ///Custom Error type `SendError` with signature `SendError()` and selector `0x8a48aa1f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SendError", abi = "SendError()")]
    pub struct SendError;
    ///Custom Error type `StakingPaused` with signature `StakingPaused()` and selector `0x26d1807b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "StakingPaused", abi = "StakingPaused()")]
    pub struct StakingPaused;
    ///Custom Error type `TransferExceedsAllowance` with signature `TransferExceedsAllowance()` and selector `0x0e812521`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TransferExceedsAllowance", abi = "TransferExceedsAllowance()")]
    pub struct TransferExceedsAllowance;
    ///Custom Error type `TransferFailed` with signature `TransferFailed()` and selector `0x90b8ec18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
    ///Custom Error type `UnAuthorizedCaller` with signature `UnAuthorizedCaller()` and selector `0xe3272bbb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnAuthorizedCaller", abi = "UnAuthorizedCaller()")]
    pub struct UnAuthorizedCaller;
    ///Custom Error type `WithdrawTooSmall` with signature `WithdrawTooSmall()` and selector `0x93c76c6f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "WithdrawTooSmall", abi = "WithdrawTooSmall()")]
    pub struct WithdrawTooSmall;
    ///Custom Error type `WrongDepositAmount` with signature `WrongDepositAmount()` and selector `0xdfe159f2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "WrongDepositAmount", abi = "WrongDepositAmount()")]
    pub struct WrongDepositAmount;
    ///Custom Error type `WrongWithdrawAmount` with signature `WrongWithdrawAmount()` and selector `0x51718c96`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "WrongWithdrawAmount", abi = "WrongWithdrawAmount()")]
    pub struct WrongWithdrawAmount;
    ///Custom Error type `ZeroAddress` with signature `ZeroAddress()` and selector `0xd92e233d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroAddress", abi = "ZeroAddress()")]
    pub struct ZeroAddress;
    ///Custom Error type `ZeroValue` with signature `ZeroValue()` and selector `0x7c946ed7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroValue", abi = "ZeroValue()")]
    pub struct ZeroValue;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MevEthErrorsErrors {
        AlreadyClaimed(AlreadyClaimed),
        AlreadyFinalised(AlreadyFinalised),
        AlreadyInitialized(AlreadyInitialized),
        DepositTooSmall(DepositTooSmall),
        DepositWasFrontrun(DepositWasFrontrun),
        FeesTooHigh(FeesTooHigh),
        IndexExceedsQueueLength(IndexExceedsQueueLength),
        InvalidOperator(InvalidOperator),
        InvalidPendingMevEthShareVault(InvalidPendingMevEthShareVault),
        InvalidPendingStakingModule(InvalidPendingStakingModule),
        InvalidSender(InvalidSender),
        NonZeroVaultBalance(NonZeroVaultBalance),
        NotEnoughEth(NotEnoughEth),
        NotFinalised(NotFinalised),
        PrematureMevEthShareVaultUpdateFinalization(
            PrematureMevEthShareVaultUpdateFinalization,
        ),
        PrematureStakingModuleUpdateFinalization(
            PrematureStakingModuleUpdateFinalization,
        ),
        SandwichProtection(SandwichProtection),
        SendError(SendError),
        StakingPaused(StakingPaused),
        TransferExceedsAllowance(TransferExceedsAllowance),
        TransferFailed(TransferFailed),
        UnAuthorizedCaller(UnAuthorizedCaller),
        WithdrawTooSmall(WithdrawTooSmall),
        WrongDepositAmount(WrongDepositAmount),
        WrongWithdrawAmount(WrongWithdrawAmount),
        ZeroAddress(ZeroAddress),
        ZeroValue(ZeroValue),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MevEthErrorsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <AlreadyClaimed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyClaimed(decoded));
            }
            if let Ok(decoded)
                = <AlreadyFinalised as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyFinalised(decoded));
            }
            if let Ok(decoded)
                = <AlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyInitialized(decoded));
            }
            if let Ok(decoded)
                = <DepositTooSmall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositTooSmall(decoded));
            }
            if let Ok(decoded)
                = <DepositWasFrontrun as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositWasFrontrun(decoded));
            }
            if let Ok(decoded)
                = <FeesTooHigh as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeesTooHigh(decoded));
            }
            if let Ok(decoded)
                = <IndexExceedsQueueLength as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IndexExceedsQueueLength(decoded));
            }
            if let Ok(decoded)
                = <InvalidOperator as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidOperator(decoded));
            }
            if let Ok(decoded)
                = <InvalidPendingMevEthShareVault as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidPendingMevEthShareVault(decoded));
            }
            if let Ok(decoded)
                = <InvalidPendingStakingModule as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidPendingStakingModule(decoded));
            }
            if let Ok(decoded)
                = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSender(decoded));
            }
            if let Ok(decoded)
                = <NonZeroVaultBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonZeroVaultBalance(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughEth as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotEnoughEth(decoded));
            }
            if let Ok(decoded)
                = <NotFinalised as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotFinalised(decoded));
            }
            if let Ok(decoded)
                = <PrematureMevEthShareVaultUpdateFinalization as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PrematureMevEthShareVaultUpdateFinalization(decoded));
            }
            if let Ok(decoded)
                = <PrematureStakingModuleUpdateFinalization as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PrematureStakingModuleUpdateFinalization(decoded));
            }
            if let Ok(decoded)
                = <SandwichProtection as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SandwichProtection(decoded));
            }
            if let Ok(decoded)
                = <SendError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendError(decoded));
            }
            if let Ok(decoded)
                = <StakingPaused as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakingPaused(decoded));
            }
            if let Ok(decoded)
                = <TransferExceedsAllowance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferExceedsAllowance(decoded));
            }
            if let Ok(decoded)
                = <TransferFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFailed(decoded));
            }
            if let Ok(decoded)
                = <UnAuthorizedCaller as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnAuthorizedCaller(decoded));
            }
            if let Ok(decoded)
                = <WithdrawTooSmall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTooSmall(decoded));
            }
            if let Ok(decoded)
                = <WrongDepositAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongDepositAmount(decoded));
            }
            if let Ok(decoded)
                = <WrongWithdrawAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongWithdrawAmount(decoded));
            }
            if let Ok(decoded)
                = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <ZeroValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroValue(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MevEthErrorsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyClaimed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AlreadyFinalised(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositWasFrontrun(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeesTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IndexExceedsQueueLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPendingMevEthShareVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPendingStakingModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonZeroVaultBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotFinalised(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrematureMevEthShareVaultUpdateFinalization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrematureStakingModuleUpdateFinalization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SandwichProtection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferExceedsAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnAuthorizedCaller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrongDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrongWithdrawAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MevEthErrorsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadyClaimed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AlreadyFinalised as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AlreadyInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DepositTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DepositWasFrontrun as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FeesTooHigh as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <IndexExceedsQueueLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOperator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPendingMevEthShareVault as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPendingStakingModule as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NonZeroVaultBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughEth as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotFinalised as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PrematureMevEthShareVaultUpdateFinalization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PrematureStakingModuleUpdateFinalization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SandwichProtection as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SendError as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <StakingPaused as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferExceedsAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnAuthorizedCaller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WithdrawTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WrongDepositAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WrongWithdrawAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroValue as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MevEthErrorsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyClaimed(element) => ::core::fmt::Display::fmt(element, f),
                Self::AlreadyFinalised(element) => ::core::fmt::Display::fmt(element, f),
                Self::AlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositWasFrontrun(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeesTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::IndexExceedsQueueLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPendingMevEthShareVault(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPendingStakingModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonZeroVaultBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotFinalised(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrematureMevEthShareVaultUpdateFinalization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrematureStakingModuleUpdateFinalization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SandwichProtection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendError(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferExceedsAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnAuthorizedCaller(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongDepositAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WrongWithdrawAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MevEthErrorsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyClaimed> for MevEthErrorsErrors {
        fn from(value: AlreadyClaimed) -> Self {
            Self::AlreadyClaimed(value)
        }
    }
    impl ::core::convert::From<AlreadyFinalised> for MevEthErrorsErrors {
        fn from(value: AlreadyFinalised) -> Self {
            Self::AlreadyFinalised(value)
        }
    }
    impl ::core::convert::From<AlreadyInitialized> for MevEthErrorsErrors {
        fn from(value: AlreadyInitialized) -> Self {
            Self::AlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<DepositTooSmall> for MevEthErrorsErrors {
        fn from(value: DepositTooSmall) -> Self {
            Self::DepositTooSmall(value)
        }
    }
    impl ::core::convert::From<DepositWasFrontrun> for MevEthErrorsErrors {
        fn from(value: DepositWasFrontrun) -> Self {
            Self::DepositWasFrontrun(value)
        }
    }
    impl ::core::convert::From<FeesTooHigh> for MevEthErrorsErrors {
        fn from(value: FeesTooHigh) -> Self {
            Self::FeesTooHigh(value)
        }
    }
    impl ::core::convert::From<IndexExceedsQueueLength> for MevEthErrorsErrors {
        fn from(value: IndexExceedsQueueLength) -> Self {
            Self::IndexExceedsQueueLength(value)
        }
    }
    impl ::core::convert::From<InvalidOperator> for MevEthErrorsErrors {
        fn from(value: InvalidOperator) -> Self {
            Self::InvalidOperator(value)
        }
    }
    impl ::core::convert::From<InvalidPendingMevEthShareVault> for MevEthErrorsErrors {
        fn from(value: InvalidPendingMevEthShareVault) -> Self {
            Self::InvalidPendingMevEthShareVault(value)
        }
    }
    impl ::core::convert::From<InvalidPendingStakingModule> for MevEthErrorsErrors {
        fn from(value: InvalidPendingStakingModule) -> Self {
            Self::InvalidPendingStakingModule(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for MevEthErrorsErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<NonZeroVaultBalance> for MevEthErrorsErrors {
        fn from(value: NonZeroVaultBalance) -> Self {
            Self::NonZeroVaultBalance(value)
        }
    }
    impl ::core::convert::From<NotEnoughEth> for MevEthErrorsErrors {
        fn from(value: NotEnoughEth) -> Self {
            Self::NotEnoughEth(value)
        }
    }
    impl ::core::convert::From<NotFinalised> for MevEthErrorsErrors {
        fn from(value: NotFinalised) -> Self {
            Self::NotFinalised(value)
        }
    }
    impl ::core::convert::From<PrematureMevEthShareVaultUpdateFinalization>
    for MevEthErrorsErrors {
        fn from(value: PrematureMevEthShareVaultUpdateFinalization) -> Self {
            Self::PrematureMevEthShareVaultUpdateFinalization(value)
        }
    }
    impl ::core::convert::From<PrematureStakingModuleUpdateFinalization>
    for MevEthErrorsErrors {
        fn from(value: PrematureStakingModuleUpdateFinalization) -> Self {
            Self::PrematureStakingModuleUpdateFinalization(value)
        }
    }
    impl ::core::convert::From<SandwichProtection> for MevEthErrorsErrors {
        fn from(value: SandwichProtection) -> Self {
            Self::SandwichProtection(value)
        }
    }
    impl ::core::convert::From<SendError> for MevEthErrorsErrors {
        fn from(value: SendError) -> Self {
            Self::SendError(value)
        }
    }
    impl ::core::convert::From<StakingPaused> for MevEthErrorsErrors {
        fn from(value: StakingPaused) -> Self {
            Self::StakingPaused(value)
        }
    }
    impl ::core::convert::From<TransferExceedsAllowance> for MevEthErrorsErrors {
        fn from(value: TransferExceedsAllowance) -> Self {
            Self::TransferExceedsAllowance(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for MevEthErrorsErrors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    impl ::core::convert::From<UnAuthorizedCaller> for MevEthErrorsErrors {
        fn from(value: UnAuthorizedCaller) -> Self {
            Self::UnAuthorizedCaller(value)
        }
    }
    impl ::core::convert::From<WithdrawTooSmall> for MevEthErrorsErrors {
        fn from(value: WithdrawTooSmall) -> Self {
            Self::WithdrawTooSmall(value)
        }
    }
    impl ::core::convert::From<WrongDepositAmount> for MevEthErrorsErrors {
        fn from(value: WrongDepositAmount) -> Self {
            Self::WrongDepositAmount(value)
        }
    }
    impl ::core::convert::From<WrongWithdrawAmount> for MevEthErrorsErrors {
        fn from(value: WrongWithdrawAmount) -> Self {
            Self::WrongWithdrawAmount(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for MevEthErrorsErrors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
        }
    }
    impl ::core::convert::From<ZeroValue> for MevEthErrorsErrors {
        fn from(value: ZeroValue) -> Self {
            Self::ZeroValue(value)
        }
    }
}
