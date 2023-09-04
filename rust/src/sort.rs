pub use sort::*;
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
pub mod sort {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("sort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sort"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static SORT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x1AWa\x03\xDC\x90\x81a\0 \x8290\x81PP\xF3[`\0\x80\xFD\xFE`\x046\x10\x15a\0\rW`\0\x80\xFD[`\x005`\xE0\x1Cc\x12\x84$\xA7\x14a\0\"W`\0\x80\xFD[` 6`\x03\x19\x01\x12a\x03\x8BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\x03\x8BW6`#\x82\x01\x12\x15a\x03\x8BW\x80`\x04\x015\x90\x82\x82\x11a\x03\x90W\x81`\x05\x1B\x91`\x1F\x19`?\x84\x01\x16`\x80\x01\x93`\x80\x85\x10\x90\x85\x11\x17a\x03\x90W`@\x93\x84R`\x80R`\xA0\x91\x82\x91\x90\x81\x01`$\x01\x906\x82\x11a\x03\x8BW`$\x01\x91[\x81\x83\x10a\x03^WPPP`\x80Q\x91`\0`\x80R\x80Q\x91`\x02\x84\x10\x84`\x05\x1B`\x80\x01\x90\x15a\x02\xC1W[P\x91\x92\x81Q\x93[\x80\x85\x81\x14a\x02cW`?\x19\x01\x90\x81Q\x92`\x1F\x19\x82\x01Q\x94a\x01\x80\x85\x87\x03\x11\x15a\x01\xD9W\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x86\x01`\x01\x1C\x16\x85Q\x80\x88Q\x90\x83Q\x90\x81\x84\x10\x15a\x01\xD0W[P\x81\x83\x10\x15a\x01\xC8W[\x81\x81\x10\x15a\x01\xC2W[\x83R\x88R\x86RQ\x96\x86\x98` \x87[[\x01\x80Q\x8A\x11\x15a\x01VW` \x90a\x01DV[\x99\x93\x90\x91\x92\x93[`\x1F\x19\x01\x80Q\x8A\x10a\x01]W\x99\x93\x92\x91\x90\x93\x8A\x80\x82\x10\x15a\x01\x88W\x81Q\x81Q\x83R\x90R` \x90a\x01CV[PP\x98\x94\x92\x97P\x94\x92\x90\x95` \x85\x01\x80\x93R`\x06\x92\x10\x82\x1B\x01\x91\x80`?\x19\x84\x01R\x83`\x1F\x19\x84\x01R`?\x19\x93\x11\x90\x1B\x01\x01\x92[\x90\x92a\0\xC5V[\x90a\x015V[\x90\x91\x90a\x01,V[\x92P\x918a\x01\"V[\x91P\x94` \x84\x97\x94\x01\x96\x87Q\x90\x80Q\x82\x81\x10\x15a\x02XW[PPP\x91\x94\x90\x92[` \x87\x01\x95\x85\x87\x11a\x02KW\x86Q\x97\x80Q\x89\x81\x11\x15a\x02=W[` \x82\x01R`\x1F\x19\x01\x80Q\x89\x81\x11a\x02\x13WP` \x90\x98\x91\x94\x92\x95\x93\x97\x98\x01R[\x91\x94\x90\x92a\x01\xF9V[PP\x92\x90\x93\x91\x95\x96Pa\x024V[\x95P\x95P\x90\x91\x92Pa\x01\xBBV[\x89RR8\x80\x80a\x01\xF1V[\x84\x84\x84`\x80R\x80Q\x91\x82\x91` \x83\x01\x90` \x84R`\x80Q\x80\x92R\x83\x01\x91\x90`\0[\x81\x81\x10a\x02\x92WPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02\x84V[\x93\x91\x81\x93\x85\x83[` \x81\x01\x90\x81Q\x81Q\x11\x89\x82\x14\x17a\x02\xE0WPa\x02\xC8V[\x88\x92\x93\x94\x95\x96\x97\x98\x91P\x14a\x03RW[`\x1F\x19\x81\x01\x90\x81Q\x81Q\x11a\x03\x05WPa\x02\xF0V[\x85\x96\x97\x92\x93\x94\x95\x91P\x14a\x03#W\x82R` \x82\x01R\x81\x01\x918a\0\xBEV[\x90` \x90\x94\x92\x94[\x82Q\x81Q\x84R\x81R\x91\x01\x90`\x1F\x19\x01\x80\x82\x10\x15a\x03JW` \x90a\x03+V[PP8a\0\xBEV[PP\x92\x93\x91\x90\x91a\0\xBEV[\x825s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\x8BW\x81R` \x92\x83\x01\x92\x01a\0\x96V[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD1\xA86vf\xA3\xD0-\xC5\xB5\x9BV\xF5-2j\xF5B\xF7Xj\x0B2b\xD5Y}\r\x12p\x07\x83dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SORT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x046\x10\x15a\0\rW`\0\x80\xFD[`\x005`\xE0\x1Cc\x12\x84$\xA7\x14a\0\"W`\0\x80\xFD[` 6`\x03\x19\x01\x12a\x03\x8BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\x03\x8BW6`#\x82\x01\x12\x15a\x03\x8BW\x80`\x04\x015\x90\x82\x82\x11a\x03\x90W\x81`\x05\x1B\x91`\x1F\x19`?\x84\x01\x16`\x80\x01\x93`\x80\x85\x10\x90\x85\x11\x17a\x03\x90W`@\x93\x84R`\x80R`\xA0\x91\x82\x91\x90\x81\x01`$\x01\x906\x82\x11a\x03\x8BW`$\x01\x91[\x81\x83\x10a\x03^WPPP`\x80Q\x91`\0`\x80R\x80Q\x91`\x02\x84\x10\x84`\x05\x1B`\x80\x01\x90\x15a\x02\xC1W[P\x91\x92\x81Q\x93[\x80\x85\x81\x14a\x02cW`?\x19\x01\x90\x81Q\x92`\x1F\x19\x82\x01Q\x94a\x01\x80\x85\x87\x03\x11\x15a\x01\xD9W\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x86\x01`\x01\x1C\x16\x85Q\x80\x88Q\x90\x83Q\x90\x81\x84\x10\x15a\x01\xD0W[P\x81\x83\x10\x15a\x01\xC8W[\x81\x81\x10\x15a\x01\xC2W[\x83R\x88R\x86RQ\x96\x86\x98` \x87[[\x01\x80Q\x8A\x11\x15a\x01VW` \x90a\x01DV[\x99\x93\x90\x91\x92\x93[`\x1F\x19\x01\x80Q\x8A\x10a\x01]W\x99\x93\x92\x91\x90\x93\x8A\x80\x82\x10\x15a\x01\x88W\x81Q\x81Q\x83R\x90R` \x90a\x01CV[PP\x98\x94\x92\x97P\x94\x92\x90\x95` \x85\x01\x80\x93R`\x06\x92\x10\x82\x1B\x01\x91\x80`?\x19\x84\x01R\x83`\x1F\x19\x84\x01R`?\x19\x93\x11\x90\x1B\x01\x01\x92[\x90\x92a\0\xC5V[\x90a\x015V[\x90\x91\x90a\x01,V[\x92P\x918a\x01\"V[\x91P\x94` \x84\x97\x94\x01\x96\x87Q\x90\x80Q\x82\x81\x10\x15a\x02XW[PPP\x91\x94\x90\x92[` \x87\x01\x95\x85\x87\x11a\x02KW\x86Q\x97\x80Q\x89\x81\x11\x15a\x02=W[` \x82\x01R`\x1F\x19\x01\x80Q\x89\x81\x11a\x02\x13WP` \x90\x98\x91\x94\x92\x95\x93\x97\x98\x01R[\x91\x94\x90\x92a\x01\xF9V[PP\x92\x90\x93\x91\x95\x96Pa\x024V[\x95P\x95P\x90\x91\x92Pa\x01\xBBV[\x89RR8\x80\x80a\x01\xF1V[\x84\x84\x84`\x80R\x80Q\x91\x82\x91` \x83\x01\x90` \x84R`\x80Q\x80\x92R\x83\x01\x91\x90`\0[\x81\x81\x10a\x02\x92WPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02\x84V[\x93\x91\x81\x93\x85\x83[` \x81\x01\x90\x81Q\x81Q\x11\x89\x82\x14\x17a\x02\xE0WPa\x02\xC8V[\x88\x92\x93\x94\x95\x96\x97\x98\x91P\x14a\x03RW[`\x1F\x19\x81\x01\x90\x81Q\x81Q\x11a\x03\x05WPa\x02\xF0V[\x85\x96\x97\x92\x93\x94\x95\x91P\x14a\x03#W\x82R` \x82\x01R\x81\x01\x918a\0\xBEV[\x90` \x90\x94\x92\x94[\x82Q\x81Q\x84R\x81R\x91\x01\x90`\x1F\x19\x01\x80\x82\x10\x15a\x03JW` \x90a\x03+V[PP8a\0\xBEV[PP\x92\x93\x91\x90\x91a\0\xBEV[\x825s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\x8BW\x81R` \x92\x83\x01\x92\x01a\0\x96V[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD1\xA86vf\xA3\xD0-\xC5\xB5\x9BV\xF5-2j\xF5B\xF7Xj\x0B2b\xD5Y}\r\x12p\x07\x83dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SORT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Sort<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Sort<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Sort<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Sort<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Sort<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Sort)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Sort<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SORT_ABI.clone(),
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
                SORT_ABI.clone(),
                SORT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `sort` (0x128424a7) function
        pub fn sort(
            &self,
            arr: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([18, 132, 36, 167], arr)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Sort<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `sort` function with signature `sort(address[])` and selector `0x128424a7`
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
    #[ethcall(name = "sort", abi = "sort(address[])")]
    pub struct SortCall {
        pub arr: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `sort` function with signature `sort(address[])` and selector `0x128424a7`
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
    pub struct SortReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
}
