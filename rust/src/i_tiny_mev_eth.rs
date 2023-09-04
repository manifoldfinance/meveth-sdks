pub use i_tiny_mev_eth::*;
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
pub mod i_tiny_mev_eth {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("grantRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRewards"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantValidatorWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "grantValidatorWithdraw",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
    pub static ITINYMEVETH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ITinyMevEth<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ITinyMevEth<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ITinyMevEth<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ITinyMevEth<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ITinyMevEth<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ITinyMevEth))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ITinyMevEth<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ITINYMEVETH_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `grantRewards` (0x558cb7f7) function
        pub fn grant_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 140, 183, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantValidatorWithdraw` (0xfe183211) function
        pub fn grant_validator_withdraw(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 24, 50, 17], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ITinyMevEth<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `grantRewards` function with signature `grantRewards()` and selector `0x558cb7f7`
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
    #[ethcall(name = "grantRewards", abi = "grantRewards()")]
    pub struct GrantRewardsCall;
    ///Container type for all input parameters for the `grantValidatorWithdraw` function with signature `grantValidatorWithdraw()` and selector `0xfe183211`
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
    #[ethcall(name = "grantValidatorWithdraw", abi = "grantValidatorWithdraw()")]
    pub struct GrantValidatorWithdrawCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ITinyMevEthCalls {
        GrantRewards(GrantRewardsCall),
        GrantValidatorWithdraw(GrantValidatorWithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for ITinyMevEthCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GrantRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRewards(decoded));
            }
            if let Ok(decoded)
                = <GrantValidatorWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GrantValidatorWithdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ITinyMevEthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GrantRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantValidatorWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ITinyMevEthCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GrantRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantValidatorWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GrantRewardsCall> for ITinyMevEthCalls {
        fn from(value: GrantRewardsCall) -> Self {
            Self::GrantRewards(value)
        }
    }
    impl ::core::convert::From<GrantValidatorWithdrawCall> for ITinyMevEthCalls {
        fn from(value: GrantValidatorWithdrawCall) -> Self {
            Self::GrantValidatorWithdraw(value)
        }
    }
}
