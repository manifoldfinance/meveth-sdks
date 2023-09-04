pub use crytic_ierc4626_internal::*;
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
pub mod crytic_ierc4626_internal {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("recognizeLoss"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recognizeLoss"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("loss"),
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
                    ::std::borrow::ToOwned::to_owned("recognizeProfit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recognizeProfit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CRYTICIERC4626INTERNAL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct CryticIERC4626Internal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticIERC4626Internal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticIERC4626Internal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticIERC4626Internal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticIERC4626Internal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticIERC4626Internal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticIERC4626Internal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICIERC4626INTERNAL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `recognizeLoss` (0x6221e4f1) function
        pub fn recognize_loss(
            &self,
            loss: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 33, 228, 241], loss)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recognizeProfit` (0x55df700d) function
        pub fn recognize_profit(
            &self,
            profit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 223, 112, 13], profit)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticIERC4626Internal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `recognizeLoss` function with signature `recognizeLoss(uint256)` and selector `0x6221e4f1`
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
    #[ethcall(name = "recognizeLoss", abi = "recognizeLoss(uint256)")]
    pub struct RecognizeLossCall {
        pub loss: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `recognizeProfit` function with signature `recognizeProfit(uint256)` and selector `0x55df700d`
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
    #[ethcall(name = "recognizeProfit", abi = "recognizeProfit(uint256)")]
    pub struct RecognizeProfitCall {
        pub profit: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CryticIERC4626InternalCalls {
        RecognizeLoss(RecognizeLossCall),
        RecognizeProfit(RecognizeProfitCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticIERC4626InternalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <RecognizeLossCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecognizeLoss(decoded));
            }
            if let Ok(decoded)
                = <RecognizeProfitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecognizeProfit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticIERC4626InternalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RecognizeLoss(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecognizeProfit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticIERC4626InternalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RecognizeLoss(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecognizeProfit(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RecognizeLossCall> for CryticIERC4626InternalCalls {
        fn from(value: RecognizeLossCall) -> Self {
            Self::RecognizeLoss(value)
        }
    }
    impl ::core::convert::From<RecognizeProfitCall> for CryticIERC4626InternalCalls {
        fn from(value: RecognizeProfitCall) -> Self {
            Self::RecognizeProfit(value)
        }
    }
}
