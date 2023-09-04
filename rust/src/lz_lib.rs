pub use lz_lib::*;
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
pub mod lz_lib {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AirdropNotSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AirdropNotSet"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GasTooLow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("GasTooLow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAdapterParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidAdapterParams",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsupportedTxType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnsupportedTxType"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroAirdrop"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroAirdrop"),
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
    pub static LZLIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4`\x17W`:\x90\x81`\x1D\x8290\x81PP\xF3[`\0\x80\xFD\xFE`\0\x80\xFD\xFE\xA2dipfsX\"\x12 =\xF2Y@\xB8\xDFz'\xC0\xB9b\xCE3\xF9\x937\xB4\xCB\xAF\xFE40\xB2y\xFC\xD0G\xAF\xCFR\xB4\xC1dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LZLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\0\x80\xFD\xFE\xA2dipfsX\"\x12 =\xF2Y@\xB8\xDFz'\xC0\xB9b\xCE3\xF9\x937\xB4\xCB\xAF\xFE40\xB2y\xFC\xD0G\xAF\xCFR\xB4\xC1dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LZLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LzLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LzLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LzLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LzLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LzLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LzLib)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LzLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LZLIB_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                LZLIB_ABI.clone(),
                LZLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LzLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AirdropNotSet` with signature `AirdropNotSet()` and selector `0xa1765567`
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
    #[etherror(name = "AirdropNotSet", abi = "AirdropNotSet()")]
    pub struct AirdropNotSet;
    ///Custom Error type `GasTooLow` with signature `GasTooLow()` and selector `0x858c8974`
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
    #[etherror(name = "GasTooLow", abi = "GasTooLow()")]
    pub struct GasTooLow;
    ///Custom Error type `InvalidAdapterParams` with signature `InvalidAdapterParams()` and selector `0xcef80ea3`
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
    #[etherror(name = "InvalidAdapterParams", abi = "InvalidAdapterParams()")]
    pub struct InvalidAdapterParams;
    ///Custom Error type `UnsupportedTxType` with signature `UnsupportedTxType()` and selector `0xdf87b546`
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
    #[etherror(name = "UnsupportedTxType", abi = "UnsupportedTxType()")]
    pub struct UnsupportedTxType;
    ///Custom Error type `ZeroAirdrop` with signature `ZeroAirdrop()` and selector `0x88245399`
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
    #[etherror(name = "ZeroAirdrop", abi = "ZeroAirdrop()")]
    pub struct ZeroAirdrop;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LzLibErrors {
        AirdropNotSet(AirdropNotSet),
        GasTooLow(GasTooLow),
        InvalidAdapterParams(InvalidAdapterParams),
        UnsupportedTxType(UnsupportedTxType),
        ZeroAirdrop(ZeroAirdrop),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LzLibErrors {
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
                = <AirdropNotSet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AirdropNotSet(decoded));
            }
            if let Ok(decoded)
                = <GasTooLow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GasTooLow(decoded));
            }
            if let Ok(decoded)
                = <InvalidAdapterParams as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidAdapterParams(decoded));
            }
            if let Ok(decoded)
                = <UnsupportedTxType as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnsupportedTxType(decoded));
            }
            if let Ok(decoded)
                = <ZeroAirdrop as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAirdrop(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LzLibErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AirdropNotSet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GasTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsupportedTxType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAirdrop(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LzLibErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AirdropNotSet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <GasTooLow as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidAdapterParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsupportedTxType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAirdrop as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LzLibErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AirdropNotSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsupportedTxType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAirdrop(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LzLibErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AirdropNotSet> for LzLibErrors {
        fn from(value: AirdropNotSet) -> Self {
            Self::AirdropNotSet(value)
        }
    }
    impl ::core::convert::From<GasTooLow> for LzLibErrors {
        fn from(value: GasTooLow) -> Self {
            Self::GasTooLow(value)
        }
    }
    impl ::core::convert::From<InvalidAdapterParams> for LzLibErrors {
        fn from(value: InvalidAdapterParams) -> Self {
            Self::InvalidAdapterParams(value)
        }
    }
    impl ::core::convert::From<UnsupportedTxType> for LzLibErrors {
        fn from(value: UnsupportedTxType) -> Self {
            Self::UnsupportedTxType(value)
        }
    }
    impl ::core::convert::From<ZeroAirdrop> for LzLibErrors {
        fn from(value: ZeroAirdrop) -> Self {
            Self::ZeroAirdrop(value)
        }
    }
}
