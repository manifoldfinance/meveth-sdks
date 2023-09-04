pub use sign_message_lib::*;
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
pub mod sign_message_lib {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getMessageHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMessageHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SignMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SignMsg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SIGNMESSAGELIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x02\xD1\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x80c\n\x10(\xC4\x14a\0\xCEWc\x85\xA5\xAF\xFE\x14a\x003W`\0\x80\xFD[4a\0\xCBW` 6`\x03\x19\x01\x12a\0\xCBW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11a\0\xC7W6`#\x83\x01\x12\x15a\0\xC7W\x81`\x04\x015\x90\x81\x11a\0\xC7W6`$\x82\x84\x01\x01\x11a\0\xC7Wa\0\x91\x91a\0\x8C\x91`$6\x92\x01a\x01UV[a\x01\xA1V[\x80\x82R`\x07` R`\x01`@\x83 U\x7F\xE7\xF4gP8\xF4\xF6\x03M\xFC\xBB\xB2LM\xC0\x8EN\xBF\x10\xEB\x9D%}=\x02\xC0\xF3\x8D\x12*\xC6\xE4\x82\x80\xA2\x80\xF3[\x82\x80\xFD[\x80\xFD[P4a\0\xCBW` 6`\x03\x19\x01\x12a\0\xCBW`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xCBW6`#\x83\x01\x12\x15a\0\xCBW` a\x01\x15a\0\x8C6`\x04\x86\x015`$\x87\x01a\x01UV[`@Q\x90\x81R\xF3[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01?W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W`@Q\x91a\x01\x7F`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01\x1DV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\x9CW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x80Q` \x80\x92\x01 \x90`@\x91\x82Q\x92\x82\x84\x01\x91\x7F`\xB3\xCB\xF8\xB4\xA2#\xD6\x8Dd\x1B;m\xDF\x9A)\x8E\x7F3q\x0C\xF3\xD3\xA9\xD1\x14kZaP\xFB\xCA\x83R\x81\x85\x01R\x80\x84R``\x84\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x85\x82\x10\x84\x83\x11\x17a\x01?W\x81\x83R\x85Q\x90 c\xF6\x98\xDA%`\xE0\x1B\x82R\x90\x84\x81`\x04\x810Z\xFA\x95\x86\x15a\x02\x90W`\0\x96a\x02^W[PP\x81Q\x93\x84\x01\x94`\x19`\xF8\x1B\x86R`\x01`\xF8\x1B`!\x86\x01R`\"\x85\x01R`B\x84\x01R`B\x83R`\x80\x83\x01\x91\x83\x83\x10\x90\x83\x11\x17a\x01?WRQ\x90 \x90V[\x85\x90\x81\x97\x92\x93\x97=\x83\x11a\x02\x89W[a\x02w\x81\x86a\x01\x1DV[\x81\x01\x03\x12a\0\xCBWPQ\x938\x80a\x02 V[P=a\x02mV[\x83Q=`\0\x82>=\x90\xFD\xFE\xA2dipfsX\"\x12 `S!\xF1\xD7\xA3[j&OP\xBD\xB5\xCB\x92J[g\xF5\xFD\xE5o(\x15v0\xF6\xA1X\x15\xB5\xF8dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SIGNMESSAGELIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x80c\n\x10(\xC4\x14a\0\xCEWc\x85\xA5\xAF\xFE\x14a\x003W`\0\x80\xFD[4a\0\xCBW` 6`\x03\x19\x01\x12a\0\xCBW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11a\0\xC7W6`#\x83\x01\x12\x15a\0\xC7W\x81`\x04\x015\x90\x81\x11a\0\xC7W6`$\x82\x84\x01\x01\x11a\0\xC7Wa\0\x91\x91a\0\x8C\x91`$6\x92\x01a\x01UV[a\x01\xA1V[\x80\x82R`\x07` R`\x01`@\x83 U\x7F\xE7\xF4gP8\xF4\xF6\x03M\xFC\xBB\xB2LM\xC0\x8EN\xBF\x10\xEB\x9D%}=\x02\xC0\xF3\x8D\x12*\xC6\xE4\x82\x80\xA2\x80\xF3[\x82\x80\xFD[\x80\xFD[P4a\0\xCBW` 6`\x03\x19\x01\x12a\0\xCBW`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xCBW6`#\x83\x01\x12\x15a\0\xCBW` a\x01\x15a\0\x8C6`\x04\x86\x015`$\x87\x01a\x01UV[`@Q\x90\x81R\xF3[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01?W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W`@Q\x91a\x01\x7F`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01\x1DV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\x9CW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x80Q` \x80\x92\x01 \x90`@\x91\x82Q\x92\x82\x84\x01\x91\x7F`\xB3\xCB\xF8\xB4\xA2#\xD6\x8Dd\x1B;m\xDF\x9A)\x8E\x7F3q\x0C\xF3\xD3\xA9\xD1\x14kZaP\xFB\xCA\x83R\x81\x85\x01R\x80\x84R``\x84\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x85\x82\x10\x84\x83\x11\x17a\x01?W\x81\x83R\x85Q\x90 c\xF6\x98\xDA%`\xE0\x1B\x82R\x90\x84\x81`\x04\x810Z\xFA\x95\x86\x15a\x02\x90W`\0\x96a\x02^W[PP\x81Q\x93\x84\x01\x94`\x19`\xF8\x1B\x86R`\x01`\xF8\x1B`!\x86\x01R`\"\x85\x01R`B\x84\x01R`B\x83R`\x80\x83\x01\x91\x83\x83\x10\x90\x83\x11\x17a\x01?WRQ\x90 \x90V[\x85\x90\x81\x97\x92\x93\x97=\x83\x11a\x02\x89W[a\x02w\x81\x86a\x01\x1DV[\x81\x01\x03\x12a\0\xCBWPQ\x938\x80a\x02 V[P=a\x02mV[\x83Q=`\0\x82>=\x90\xFD\xFE\xA2dipfsX\"\x12 `S!\xF1\xD7\xA3[j&OP\xBD\xB5\xCB\x92J[g\xF5\xFD\xE5o(\x15v0\xF6\xA1X\x15\xB5\xF8dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SIGNMESSAGELIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SignMessageLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SignMessageLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SignMessageLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SignMessageLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SignMessageLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SignMessageLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SignMessageLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SIGNMESSAGELIB_ABI.clone(),
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
                SIGNMESSAGELIB_ABI.clone(),
                SIGNMESSAGELIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getMessageHash` (0x0a1028c4) function
        pub fn get_message_hash(
            &self,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([10, 16, 40, 196], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signMessage` (0x85a5affe) function
        pub fn sign_message(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 165, 175, 254], data)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SignMsg` event
        pub fn sign_msg_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignMsgFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignMsgFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SignMessageLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SignMsg", abi = "SignMsg(bytes32)")]
    pub struct SignMsgFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getMessageHash` function with signature `getMessageHash(bytes)` and selector `0x0a1028c4`
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
    #[ethcall(name = "getMessageHash", abi = "getMessageHash(bytes)")]
    pub struct GetMessageHashCall {
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `signMessage` function with signature `signMessage(bytes)` and selector `0x85a5affe`
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
    #[ethcall(name = "signMessage", abi = "signMessage(bytes)")]
    pub struct SignMessageCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SignMessageLibCalls {
        GetMessageHash(GetMessageHashCall),
        SignMessage(SignMessageCall),
    }
    impl ::ethers::core::abi::AbiDecode for SignMessageLibCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetMessageHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMessageHash(decoded));
            }
            if let Ok(decoded)
                = <SignMessageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SignMessage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SignMessageLibCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetMessageHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SignMessageLibCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetMessageHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignMessage(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetMessageHashCall> for SignMessageLibCalls {
        fn from(value: GetMessageHashCall) -> Self {
            Self::GetMessageHash(value)
        }
    }
    impl ::core::convert::From<SignMessageCall> for SignMessageLibCalls {
        fn from(value: SignMessageCall) -> Self {
            Self::SignMessage(value)
        }
    }
    ///Container type for all return fields from the `getMessageHash` function with signature `getMessageHash(bytes)` and selector `0x0a1028c4`
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
    pub struct GetMessageHashReturn(pub [u8; 32]);
}
