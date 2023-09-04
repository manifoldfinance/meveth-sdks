pub use gnosis_safe_proxy_factory::*;
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
pub mod gnosis_safe_proxy_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "calculateCreateProxyWithNonceAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateCreateProxyWithNonceAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("saltNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GnosisSafeProxy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createProxy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GnosisSafeProxy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createProxyWithCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createProxyWithCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("saltNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callback"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IProxyCreationCallback",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GnosisSafeProxy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createProxyWithNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createProxyWithNonce",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("saltNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GnosisSafeProxy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxyCreationCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxyCreationCode"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxyRuntimeCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxyRuntimeCode"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ProxyCreation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProxyCreation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
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
    pub static GNOSISSAFEPROXYFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\tW\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81c\x16\x88\xF0\xB9\x14a\0zWP\x80c%\0Q\x0E\x14a\0uW\x80cS\xE5\xD95\x14a\0pW\x80ca\xB6\x9A\xBD\x14a\0kW\x80c\xAD\xDA\xCC\x0F\x14a\0fWc\xD1\x8A\xF5M\x14a\0aW`\0\x80\xFD[a\x03\xEFV[a\x03\xA9V[a\x02\xB1V[a\x02jV[a\x01\xB5V[4a\0\xE6W``6`\x03\x19\x01\x12a\0\xE6W`\x045\x90a\0\x98\x82a\0\xE9V[`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xE6Wa\0\xE2a\0\xC8\x84a\0\xBF6`\x04\x87\x01a\x01\x97V[`D5\x91a\x05\x1EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\0\xFAWV[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01)W`@RV[a\0\xFFV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01)W`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01)W`@Q\x91a\x01z`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01.V[\x82\x94\x81\x84R\x81\x83\x01\x11a\0\xFAW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\0\xFAW\x81` a\x01\xB2\x935\x91\x01a\x01PV[\x90V[4a\0\xFAW``6`\x03\x19\x01\x12a\0\xFAW`\x045a\x01\xD2\x81a\0\xE9V[`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x83\x11a\0\xFAW6`#\x84\x01\x12\x15a\0\xFAW\x82`\x04\x015\x91\x82\x11a\0\xFAW6`$\x83\x85\x01\x01\x11a\0\xFAW`$`D5\x93\x01\x90a\x06\xB6V[`\0\x91\x03\x12a\0\xFAWV[`\0[\x83\x81\x10a\x025WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02%V[\x90` \x91a\x02^\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02\"V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[4a\0\xFAW`\x006`\x03\x19\x01\x12a\0\xFAWa\0\xE2a\x01k`@Q\x90a\x02\x92` \x82\x01\x83a\x01.V[\x80\x82Ra\x071` \x83\x019`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02EV[4a\0\xFAW`@6`\x03\x19\x01\x12a\0\xFAW`\x045a\x02\xCE\x81a\0\xE9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\0\xFAWa\x02\xEF\x906\x90`\x04\x01a\x01\x97V[\x90`@Qa\x01k\x80\x82\x01\x92\x82\x84\x10\x90\x84\x11\x17a\x01)Wa\x071\x829`\x01`\x01`\xA0\x1B\x03\x84\x16\x82R` \x81`\0\x93\x03\x01\x90\x82\xF0\x91\x82\x15a\x03\xA4W\x80Q\x90\x81a\x03\x8DW[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x87\x16` \x82\x01Ra\0\xE2\x91\x86\x91\x7FOQ\xFA\xF6\xC4V\x1F\xF9_\x06vW\xE449\xF0\xF8V\xD9|\x04\xD9\xEC\x90p\xA6\x19\x9A\xD4\x18\xE25\x91\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x82\x91` \x83\x92\x01\x82\x86Z\xF1\x15a\0\xE6W\x80\x80a\x031V[a\x05\x12V[4a\0\xFAW`\x006`\x03\x19\x01\x12a\0\xFAWa\0\xE2`\x86`@Q\x90a\x03\xD0` \x82\x01\x83a\x01.V[\x80\x82Ra\x08\x9C` \x83\x019`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02EV[4a\0\xFAW`\x806`\x03\x19\x01\x12a\0\xFAW`\x045a\x04\x0C\x81a\0\xE9V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFAWa\x04,\x906\x90`\x04\x01a\x01\x97V[\x90`d5\x90`D5a\x04=\x83a\0\xE9V[`\x01`\x01`\xA0\x1B\x03a\x04\x95`@Q` \x81\x01\x90a\x04\x8B\x81a\x04}\x89\x88\x86\x90\x91`4\x92\x82Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16` \x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\x01.V[Q\x90 \x86\x85a\x05\x1EV[\x93\x16\x80a\x04\xB1W[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x90\xF3[\x80;\x15a\0\xFAWa\x04\xDE\x94`\0\x80\x94`@Q\x97\x88\x95\x86\x94\x85\x93c\x03\xCAV\xA3`\xE3\x1B\x85R\x8A`\x04\x86\x01a\x06\x83V[\x03\x92Z\xF1\x91\x82\x15a\x03\xA4Wa\0\xE2\x92a\x04\xF9W[\x80\x80a\x04\x9DV[\x80a\x05\x06a\x05\x0C\x92a\x01\x15V[\x80a\x02\x17V[8a\x04\xF2V[`@Q=`\0\x82>=\x90\xFD[\x90\x92\x91a\x05,\x90\x84\x83a\x05\xDBV[\x92\x80Q\x90\x81a\x05wW[PP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x92\x90\x92\x16` \x83\x01R\x7FOQ\xFA\xF6\xC4V\x1F\xF9_\x06vW\xE449\xF0\xF8V\xD9|\x04\xD9\xEC\x90p\xA6\x19\x9A\xD4\x18\xE25\x91\xA1V[`\0\x91` \x83\x92\x01\x82\x87Z\xF1\x15a\0\xFAW8\x80a\x056V[\x15a\x05\x96WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FCreate2 call failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x92\x91\x81Q` \x80\x93\x01 \x91`@\x92\x83Q\x92\x82\x84\x01\x91\x82R\x84\x84\x01R\x83\x83R``\x83\x01\x90\x83\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01)W\x84\x82a\x06p\x92a\x06R\x94a\x06\x81\x98R\x86Q\x90 \x95`\x80a\x01k\x91a\x067\x88\x84\x01\x85a\x01.V[\x82\x84R\x01\x90a\x071\x829\x82Q\x94\x85\x92Q\x80\x92\x88\x85\x01\x90a\x02\"V[\x81\x01`\x01`\x01`\xA0\x1B\x03\x80\x9A\x16\x86\x82\x01R\x03\x84\x81\x01\x84R\x01\x82a\x01.V[\x80Q\x91\x01`\0\xF5\x92\x83\x16\x15\x15a\x05\x8FV[V[\x94\x93\x92a\x06\xB1\x91``\x93`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x88R\x16` \x87\x01R`\x80`@\x87\x01R`\x80\x86\x01\x90a\x02EV[\x93\x01RV[\x90a\x06\xC7\x90a\x06\xCD\x94\x936\x91a\x01PV[\x90a\x05\xDBV[`@Q\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16` \x82\x01R`\x14\x81R`@\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01)W`@\x82\x90RbF\x1B\xCD`\xE5\x1B\x82R` `D\x82\x01R`?\x19\x90a\x07+`d\x82\x01\x82a\x02EV[\x03\x01\x90\xFD\xFE`\x804a\0\xC9W`\x1Fa\x01k8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xCEW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\xC9WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0\xC9W\x80\x15a\0yW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Q`\x86\x90\x81a\0\xE5\x829\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FInvalid singleton address provid`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x805cS\x0C\xA47`\xE1\x1B\x14`IW\x80\x80\x926\x82\x807\x816\x91Z\xF4=\x82\x80>\x15`EW=\x90\xF3[=\x90\xFD[` \x91\x81R\xF3\xFE\xA2dipfsX\"\x12 #X\x0C\xDD\xE8\xE9\x1D%yj)\x80ow$\x83Xr(\x15;c\xE0\xD7\xF8h\x98\xE0P!\xE8[dsolcC\0\x08\x13\x003`\x80`@R`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x805cS\x0C\xA47`\xE1\x1B\x14`IW\x80\x80\x926\x82\x807\x816\x91Z\xF4=\x82\x80>\x15`EW=\x90\xF3[=\x90\xFD[` \x91\x81R\xF3\xFE\xA2dipfsX\"\x12 #X\x0C\xDD\xE8\xE9\x1D%yj)\x80ow$\x83Xr(\x15;c\xE0\xD7\xF8h\x98\xE0P!\xE8[dsolcC\0\x08\x13\x003\xA2dipfsX\"\x12 L\x8C\x95B\xA7\xD2dw#\xEA\x0C_[Po\xBF\xCBt\xB3\xCF\xAA\xD6\xA5\\\xAA\xBA\xC4\x11\xC8E\x96\rdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static GNOSISSAFEPROXYFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81c\x16\x88\xF0\xB9\x14a\0zWP\x80c%\0Q\x0E\x14a\0uW\x80cS\xE5\xD95\x14a\0pW\x80ca\xB6\x9A\xBD\x14a\0kW\x80c\xAD\xDA\xCC\x0F\x14a\0fWc\xD1\x8A\xF5M\x14a\0aW`\0\x80\xFD[a\x03\xEFV[a\x03\xA9V[a\x02\xB1V[a\x02jV[a\x01\xB5V[4a\0\xE6W``6`\x03\x19\x01\x12a\0\xE6W`\x045\x90a\0\x98\x82a\0\xE9V[`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xE6Wa\0\xE2a\0\xC8\x84a\0\xBF6`\x04\x87\x01a\x01\x97V[`D5\x91a\x05\x1EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\0\xFAWV[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01)W`@RV[a\0\xFFV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01)W`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01)W`@Q\x91a\x01z`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01.V[\x82\x94\x81\x84R\x81\x83\x01\x11a\0\xFAW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\0\xFAW\x81` a\x01\xB2\x935\x91\x01a\x01PV[\x90V[4a\0\xFAW``6`\x03\x19\x01\x12a\0\xFAW`\x045a\x01\xD2\x81a\0\xE9V[`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x83\x11a\0\xFAW6`#\x84\x01\x12\x15a\0\xFAW\x82`\x04\x015\x91\x82\x11a\0\xFAW6`$\x83\x85\x01\x01\x11a\0\xFAW`$`D5\x93\x01\x90a\x06\xB6V[`\0\x91\x03\x12a\0\xFAWV[`\0[\x83\x81\x10a\x025WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x02%V[\x90` \x91a\x02^\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x02\"V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[4a\0\xFAW`\x006`\x03\x19\x01\x12a\0\xFAWa\0\xE2a\x01k`@Q\x90a\x02\x92` \x82\x01\x83a\x01.V[\x80\x82Ra\x071` \x83\x019`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02EV[4a\0\xFAW`@6`\x03\x19\x01\x12a\0\xFAW`\x045a\x02\xCE\x81a\0\xE9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\0\xFAWa\x02\xEF\x906\x90`\x04\x01a\x01\x97V[\x90`@Qa\x01k\x80\x82\x01\x92\x82\x84\x10\x90\x84\x11\x17a\x01)Wa\x071\x829`\x01`\x01`\xA0\x1B\x03\x84\x16\x82R` \x81`\0\x93\x03\x01\x90\x82\xF0\x91\x82\x15a\x03\xA4W\x80Q\x90\x81a\x03\x8DW[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x87\x16` \x82\x01Ra\0\xE2\x91\x86\x91\x7FOQ\xFA\xF6\xC4V\x1F\xF9_\x06vW\xE449\xF0\xF8V\xD9|\x04\xD9\xEC\x90p\xA6\x19\x9A\xD4\x18\xE25\x91\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x82\x91` \x83\x92\x01\x82\x86Z\xF1\x15a\0\xE6W\x80\x80a\x031V[a\x05\x12V[4a\0\xFAW`\x006`\x03\x19\x01\x12a\0\xFAWa\0\xE2`\x86`@Q\x90a\x03\xD0` \x82\x01\x83a\x01.V[\x80\x82Ra\x08\x9C` \x83\x019`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02EV[4a\0\xFAW`\x806`\x03\x19\x01\x12a\0\xFAW`\x045a\x04\x0C\x81a\0\xE9V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFAWa\x04,\x906\x90`\x04\x01a\x01\x97V[\x90`d5\x90`D5a\x04=\x83a\0\xE9V[`\x01`\x01`\xA0\x1B\x03a\x04\x95`@Q` \x81\x01\x90a\x04\x8B\x81a\x04}\x89\x88\x86\x90\x91`4\x92\x82Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16` \x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\x01.V[Q\x90 \x86\x85a\x05\x1EV[\x93\x16\x80a\x04\xB1W[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x90\xF3[\x80;\x15a\0\xFAWa\x04\xDE\x94`\0\x80\x94`@Q\x97\x88\x95\x86\x94\x85\x93c\x03\xCAV\xA3`\xE3\x1B\x85R\x8A`\x04\x86\x01a\x06\x83V[\x03\x92Z\xF1\x91\x82\x15a\x03\xA4Wa\0\xE2\x92a\x04\xF9W[\x80\x80a\x04\x9DV[\x80a\x05\x06a\x05\x0C\x92a\x01\x15V[\x80a\x02\x17V[8a\x04\xF2V[`@Q=`\0\x82>=\x90\xFD[\x90\x92\x91a\x05,\x90\x84\x83a\x05\xDBV[\x92\x80Q\x90\x81a\x05wW[PP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x92\x90\x92\x16` \x83\x01R\x7FOQ\xFA\xF6\xC4V\x1F\xF9_\x06vW\xE449\xF0\xF8V\xD9|\x04\xD9\xEC\x90p\xA6\x19\x9A\xD4\x18\xE25\x91\xA1V[`\0\x91` \x83\x92\x01\x82\x87Z\xF1\x15a\0\xFAW8\x80a\x056V[\x15a\x05\x96WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FCreate2 call failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x92\x91\x81Q` \x80\x93\x01 \x91`@\x92\x83Q\x92\x82\x84\x01\x91\x82R\x84\x84\x01R\x83\x83R``\x83\x01\x90\x83\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01)W\x84\x82a\x06p\x92a\x06R\x94a\x06\x81\x98R\x86Q\x90 \x95`\x80a\x01k\x91a\x067\x88\x84\x01\x85a\x01.V[\x82\x84R\x01\x90a\x071\x829\x82Q\x94\x85\x92Q\x80\x92\x88\x85\x01\x90a\x02\"V[\x81\x01`\x01`\x01`\xA0\x1B\x03\x80\x9A\x16\x86\x82\x01R\x03\x84\x81\x01\x84R\x01\x82a\x01.V[\x80Q\x91\x01`\0\xF5\x92\x83\x16\x15\x15a\x05\x8FV[V[\x94\x93\x92a\x06\xB1\x91``\x93`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x88R\x16` \x87\x01R`\x80`@\x87\x01R`\x80\x86\x01\x90a\x02EV[\x93\x01RV[\x90a\x06\xC7\x90a\x06\xCD\x94\x936\x91a\x01PV[\x90a\x05\xDBV[`@Q\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16` \x82\x01R`\x14\x81R`@\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01)W`@\x82\x90RbF\x1B\xCD`\xE5\x1B\x82R` `D\x82\x01R`?\x19\x90a\x07+`d\x82\x01\x82a\x02EV[\x03\x01\x90\xFD\xFE`\x804a\0\xC9W`\x1Fa\x01k8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xCEW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\xC9WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0\xC9W\x80\x15a\0yW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Q`\x86\x90\x81a\0\xE5\x829\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FInvalid singleton address provid`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x805cS\x0C\xA47`\xE1\x1B\x14`IW\x80\x80\x926\x82\x807\x816\x91Z\xF4=\x82\x80>\x15`EW=\x90\xF3[=\x90\xFD[` \x91\x81R\xF3\xFE\xA2dipfsX\"\x12 #X\x0C\xDD\xE8\xE9\x1D%yj)\x80ow$\x83Xr(\x15;c\xE0\xD7\xF8h\x98\xE0P!\xE8[dsolcC\0\x08\x13\x003`\x80`@R`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x805cS\x0C\xA47`\xE1\x1B\x14`IW\x80\x80\x926\x82\x807\x816\x91Z\xF4=\x82\x80>\x15`EW=\x90\xF3[=\x90\xFD[` \x91\x81R\xF3\xFE\xA2dipfsX\"\x12 #X\x0C\xDD\xE8\xE9\x1D%yj)\x80ow$\x83Xr(\x15;c\xE0\xD7\xF8h\x98\xE0P!\xE8[dsolcC\0\x08\x13\x003\xA2dipfsX\"\x12 L\x8C\x95B\xA7\xD2dw#\xEA\x0C_[Po\xBF\xCBt\xB3\xCF\xAA\xD6\xA5\\\xAA\xBA\xC4\x11\xC8E\x96\rdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static GNOSISSAFEPROXYFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GnosisSafeProxyFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GnosisSafeProxyFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GnosisSafeProxyFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GnosisSafeProxyFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GnosisSafeProxyFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GnosisSafeProxyFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GnosisSafeProxyFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GNOSISSAFEPROXYFACTORY_ABI.clone(),
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
                GNOSISSAFEPROXYFACTORY_ABI.clone(),
                GNOSISSAFEPROXYFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `calculateCreateProxyWithNonceAddress` (0x2500510e) function
        pub fn calculate_create_proxy_with_nonce_address(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([37, 0, 81, 14], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxy` (0x61b69abd) function
        pub fn create_proxy(
            &self,
            singleton: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([97, 182, 154, 189], (singleton, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithCallback` (0xd18af54d) function
        pub fn create_proxy_with_callback(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [209, 138, 245, 77],
                    (singleton, initializer, salt_nonce, callback),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithNonce` (0x1688f0b9) function
        pub fn create_proxy_with_nonce(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 136, 240, 185], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyCreationCode` (0x53e5d935) function
        pub fn proxy_creation_code(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([83, 229, 217, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyRuntimeCode` (0xaddacc0f) function
        pub fn proxy_runtime_code(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([173, 218, 204, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ProxyCreation` event
        pub fn proxy_creation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProxyCreationFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProxyCreationFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GnosisSafeProxyFactory<M> {
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
    #[ethevent(name = "ProxyCreation", abi = "ProxyCreation(address,address)")]
    pub struct ProxyCreationFilter {
        pub proxy: ::ethers::core::types::Address,
        pub singleton: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `calculateCreateProxyWithNonceAddress` function with signature `calculateCreateProxyWithNonceAddress(address,bytes,uint256)` and selector `0x2500510e`
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
        name = "calculateCreateProxyWithNonceAddress",
        abi = "calculateCreateProxyWithNonceAddress(address,bytes,uint256)"
    )]
    pub struct CalculateCreateProxyWithNonceAddressCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createProxy` function with signature `createProxy(address,bytes)` and selector `0x61b69abd`
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
    #[ethcall(name = "createProxy", abi = "createProxy(address,bytes)")]
    pub struct CreateProxyCall {
        pub singleton: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
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
        name = "createProxyWithCallback",
        abi = "createProxyWithCallback(address,bytes,uint256,address)"
    )]
    pub struct CreateProxyWithCallbackCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
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
        name = "createProxyWithNonce",
        abi = "createProxyWithNonce(address,bytes,uint256)"
    )]
    pub struct CreateProxyWithNonceCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
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
    #[ethcall(name = "proxyCreationCode", abi = "proxyCreationCode()")]
    pub struct ProxyCreationCodeCall;
    ///Container type for all input parameters for the `proxyRuntimeCode` function with signature `proxyRuntimeCode()` and selector `0xaddacc0f`
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
    #[ethcall(name = "proxyRuntimeCode", abi = "proxyRuntimeCode()")]
    pub struct ProxyRuntimeCodeCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GnosisSafeProxyFactoryCalls {
        CalculateCreateProxyWithNonceAddress(CalculateCreateProxyWithNonceAddressCall),
        CreateProxy(CreateProxyCall),
        CreateProxyWithCallback(CreateProxyWithCallbackCall),
        CreateProxyWithNonce(CreateProxyWithNonceCall),
        ProxyCreationCode(ProxyCreationCodeCall),
        ProxyRuntimeCode(ProxyRuntimeCodeCall),
    }
    impl ::ethers::core::abi::AbiDecode for GnosisSafeProxyFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CalculateCreateProxyWithNonceAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateCreateProxyWithNonceAddress(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateProxy(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CreateProxyWithCallback(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyWithNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CreateProxyWithNonce(decoded));
            }
            if let Ok(decoded)
                = <ProxyCreationCodeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProxyCreationCode(decoded));
            }
            if let Ok(decoded)
                = <ProxyRuntimeCodeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProxyRuntimeCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GnosisSafeProxyFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateCreateProxyWithNonceAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyCreationCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyRuntimeCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GnosisSafeProxyFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateCreateProxyWithNonceAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateProxy(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateProxyWithCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateProxyWithNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProxyCreationCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyRuntimeCode(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CalculateCreateProxyWithNonceAddressCall>
    for GnosisSafeProxyFactoryCalls {
        fn from(value: CalculateCreateProxyWithNonceAddressCall) -> Self {
            Self::CalculateCreateProxyWithNonceAddress(value)
        }
    }
    impl ::core::convert::From<CreateProxyCall> for GnosisSafeProxyFactoryCalls {
        fn from(value: CreateProxyCall) -> Self {
            Self::CreateProxy(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithCallbackCall>
    for GnosisSafeProxyFactoryCalls {
        fn from(value: CreateProxyWithCallbackCall) -> Self {
            Self::CreateProxyWithCallback(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithNonceCall>
    for GnosisSafeProxyFactoryCalls {
        fn from(value: CreateProxyWithNonceCall) -> Self {
            Self::CreateProxyWithNonce(value)
        }
    }
    impl ::core::convert::From<ProxyCreationCodeCall> for GnosisSafeProxyFactoryCalls {
        fn from(value: ProxyCreationCodeCall) -> Self {
            Self::ProxyCreationCode(value)
        }
    }
    impl ::core::convert::From<ProxyRuntimeCodeCall> for GnosisSafeProxyFactoryCalls {
        fn from(value: ProxyRuntimeCodeCall) -> Self {
            Self::ProxyRuntimeCode(value)
        }
    }
    ///Container type for all return fields from the `calculateCreateProxyWithNonceAddress` function with signature `calculateCreateProxyWithNonceAddress(address,bytes,uint256)` and selector `0x2500510e`
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
    pub struct CalculateCreateProxyWithNonceAddressReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxy` function with signature `createProxy(address,bytes)` and selector `0x61b69abd`
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
    pub struct CreateProxyReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
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
    pub struct CreateProxyWithCallbackReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
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
    pub struct CreateProxyWithNonceReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
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
    pub struct ProxyCreationCodeReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `proxyRuntimeCode` function with signature `proxyRuntimeCode()` and selector `0xaddacc0f`
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
    pub struct ProxyRuntimeCodeReturn(pub ::ethers::core::types::Bytes);
}
