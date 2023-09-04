pub use i_staking_module::*;
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
pub mod i_staking_module {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("VALIDATOR_DEPOSIT_SIZE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VALIDATOR_DEPOSIT_SIZE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("batchMigrate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchMigrate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("batchData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakingModule.ValidatorData[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakingModule.ValidatorData",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("latestDepositRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mevEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mevEth"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payValidatorWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "payValidatorWithdraw",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("record"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("record"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recoverToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recoverToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerExit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerExit"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validators"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISTAKINGMODULE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IStakingModule<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStakingModule<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStakingModule<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStakingModule<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStakingModule<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IStakingModule))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStakingModule<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISTAKINGMODULE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `VALIDATOR_DEPOSIT_SIZE` (0x5552aa65) function
        pub fn validator_deposit_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 82, 170, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchMigrate` (0x832b7c6b) function
        pub fn batch_migrate(
            &self,
            batch_data: ::std::vec::Vec<ValidatorData>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 43, 124, 107], batch_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xb778a3a7) function
        pub fn deposit(
            &self,
            data: ValidatorData,
            latest_deposit_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 120, 163, 167], (data, latest_deposit_root))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mevEth` (0xbadf2663) function
        pub fn mev_eth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([186, 223, 38, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payRewards` (0x60e3a0ac) function
        pub fn pay_rewards(
            &self,
            rewards: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 227, 160, 172], rewards)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payValidatorWithdraw` (0x14b71a83) function
        pub fn pay_validator_withdraw(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 183, 26, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `record` (0x266cf109) function
        pub fn record(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128, u128, u128)> {
            self.0
                .method_hash([38, 108, 241, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recoverToken` (0xa7229fd9) function
        pub fn recover_token(
            &self,
            token: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 34, 159, 217], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerExit` (0x4ad8d34b) function
        pub fn register_exit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 216, 211, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validators` (0xca1e7819) function
        pub fn validators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 30, 120, 25], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IStakingModule<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `VALIDATOR_DEPOSIT_SIZE` function with signature `VALIDATOR_DEPOSIT_SIZE()` and selector `0x5552aa65`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "VALIDATOR_DEPOSIT_SIZE", abi = "VALIDATOR_DEPOSIT_SIZE()")]
    pub struct ValidatorDepositSizeCall;
    ///Container type for all input parameters for the `batchMigrate` function with signature `batchMigrate((address,bytes,bytes32,bytes,bytes32)[])` and selector `0x832b7c6b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "batchMigrate",
        abi = "batchMigrate((address,bytes,bytes32,bytes,bytes32)[])"
    )]
    pub struct BatchMigrateCall {
        pub batch_data: ::std::vec::Vec<ValidatorData>,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit((address,bytes,bytes32,bytes,bytes32),bytes32)` and selector `0xb778a3a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "deposit",
        abi = "deposit((address,bytes,bytes32,bytes,bytes32),bytes32)"
    )]
    pub struct DepositCall {
        pub data: ValidatorData,
        pub latest_deposit_root: [u8; 32],
    }
    ///Container type for all input parameters for the `mevEth` function with signature `mevEth()` and selector `0xbadf2663`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mevEth", abi = "mevEth()")]
    pub struct MevEthCall;
    ///Container type for all input parameters for the `payRewards` function with signature `payRewards(uint256)` and selector `0x60e3a0ac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "payRewards", abi = "payRewards(uint256)")]
    pub struct PayRewardsCall {
        pub rewards: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `payValidatorWithdraw` function with signature `payValidatorWithdraw()` and selector `0x14b71a83`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "payValidatorWithdraw", abi = "payValidatorWithdraw()")]
    pub struct PayValidatorWithdrawCall;
    ///Container type for all input parameters for the `record` function with signature `record()` and selector `0x266cf109`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "record", abi = "record()")]
    pub struct RecordCall;
    ///Container type for all input parameters for the `recoverToken` function with signature `recoverToken(address,address,uint256)` and selector `0xa7229fd9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recoverToken", abi = "recoverToken(address,address,uint256)")]
    pub struct RecoverTokenCall {
        pub token: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerExit` function with signature `registerExit()` and selector `0x4ad8d34b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "registerExit", abi = "registerExit()")]
    pub struct RegisterExitCall;
    ///Container type for all input parameters for the `validators` function with signature `validators()` and selector `0xca1e7819`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "validators", abi = "validators()")]
    pub struct ValidatorsCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IStakingModuleCalls {
        ValidatorDepositSize(ValidatorDepositSizeCall),
        BatchMigrate(BatchMigrateCall),
        Deposit(DepositCall),
        MevEth(MevEthCall),
        PayRewards(PayRewardsCall),
        PayValidatorWithdraw(PayValidatorWithdrawCall),
        Record(RecordCall),
        RecoverToken(RecoverTokenCall),
        RegisterExit(RegisterExitCall),
        Validators(ValidatorsCall),
    }
    impl ::ethers::core::abi::AbiDecode for IStakingModuleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ValidatorDepositSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidatorDepositSize(decoded));
            }
            if let Ok(decoded)
                = <BatchMigrateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BatchMigrate(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <MevEthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MevEth(decoded));
            }
            if let Ok(decoded)
                = <PayRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayRewards(decoded));
            }
            if let Ok(decoded)
                = <PayValidatorWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PayValidatorWithdraw(decoded));
            }
            if let Ok(decoded)
                = <RecordCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Record(decoded));
            }
            if let Ok(decoded)
                = <RecoverTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecoverToken(decoded));
            }
            if let Ok(decoded)
                = <RegisterExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RegisterExit(decoded));
            }
            if let Ok(decoded)
                = <ValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Validators(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IStakingModuleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ValidatorDepositSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchMigrate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MevEth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayValidatorWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Record(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecoverToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Validators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IStakingModuleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ValidatorDepositSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchMigrate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayValidatorWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Record(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecoverToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validators(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ValidatorDepositSizeCall> for IStakingModuleCalls {
        fn from(value: ValidatorDepositSizeCall) -> Self {
            Self::ValidatorDepositSize(value)
        }
    }
    impl ::core::convert::From<BatchMigrateCall> for IStakingModuleCalls {
        fn from(value: BatchMigrateCall) -> Self {
            Self::BatchMigrate(value)
        }
    }
    impl ::core::convert::From<DepositCall> for IStakingModuleCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<MevEthCall> for IStakingModuleCalls {
        fn from(value: MevEthCall) -> Self {
            Self::MevEth(value)
        }
    }
    impl ::core::convert::From<PayRewardsCall> for IStakingModuleCalls {
        fn from(value: PayRewardsCall) -> Self {
            Self::PayRewards(value)
        }
    }
    impl ::core::convert::From<PayValidatorWithdrawCall> for IStakingModuleCalls {
        fn from(value: PayValidatorWithdrawCall) -> Self {
            Self::PayValidatorWithdraw(value)
        }
    }
    impl ::core::convert::From<RecordCall> for IStakingModuleCalls {
        fn from(value: RecordCall) -> Self {
            Self::Record(value)
        }
    }
    impl ::core::convert::From<RecoverTokenCall> for IStakingModuleCalls {
        fn from(value: RecoverTokenCall) -> Self {
            Self::RecoverToken(value)
        }
    }
    impl ::core::convert::From<RegisterExitCall> for IStakingModuleCalls {
        fn from(value: RegisterExitCall) -> Self {
            Self::RegisterExit(value)
        }
    }
    impl ::core::convert::From<ValidatorsCall> for IStakingModuleCalls {
        fn from(value: ValidatorsCall) -> Self {
            Self::Validators(value)
        }
    }
    ///Container type for all return fields from the `VALIDATOR_DEPOSIT_SIZE` function with signature `VALIDATOR_DEPOSIT_SIZE()` and selector `0x5552aa65`
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
    pub struct ValidatorDepositSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mevEth` function with signature `mevEth()` and selector `0xbadf2663`
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
    pub struct MevEthReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `record` function with signature `record()` and selector `0x266cf109`
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
    pub struct RecordReturn(pub u128, pub u128, pub u128, pub u128);
    ///Container type for all return fields from the `validators` function with signature `validators()` and selector `0xca1e7819`
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
    pub struct ValidatorsReturn(pub ::ethers::core::types::U256);
}
