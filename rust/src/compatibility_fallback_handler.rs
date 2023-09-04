pub use compatibility_fallback_handler::*;
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
pub mod compatibility_fallback_handler {
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
                    ::std::borrow::ToOwned::to_owned("getMessageHashForSafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMessageHashForSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safe"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GnosisSafe"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("getModules"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getModules"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isValidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isValidSignature"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signature"),
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
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isValidSignature"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signature"),
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
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155BatchReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "onERC1155BatchReceived",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldataPayload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("response"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokensReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokensReceived"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
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
    pub static COMPATIBILITYFALLBACKHANDLER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x0BN\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80b#\xDE)\x14a\0\xC6W\x80c\x01\xFF\xC9\xA7\x14a\0\xC1W\x80c\n\x10(\xC4\x14a\0\xBCW\x80c\x15\x0Bz\x02\x14a\0\xB7W\x80c\x16&\xBA~\x14a\0\xB2W\x80c \xC1;\x0B\x14a\0\xADW\x80cj\xC2G\x84\x14a\0\xA8W\x80c\xB2IM\xF3\x14a\0\xA3W\x80c\xBC\x19|\x81\x14a\0\x9EW\x80c\xBDa\x95\x1D\x14a\0\x99Wc\xF2:na\x14a\0\x94W`\0\x80\xFD[a\x08\xD5V[a\x08@V[a\x07\\V[a\x06 V[a\x05\x88V[a\x04UV[a\x03EV[a\x02\xEAV[a\x02\xA6V[a\x01\x90V[a\x01\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\0\xDCWV[`\0\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\0\xDCW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xDCW` \x83\x81\x86\x01\x95\x01\x01\x11a\0\xDCWV[4a\0\xDCW`\xC06`\x03\x19\x01\x12a\0\xDCWa\x01+`\x045a\0\xCBV[a\x016`$5a\0\xCBV[a\x01A`D5a\0\xCBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x845\x81\x81\x11a\0\xDCWa\x01b\x906\x90`\x04\x01a\0\xE1V[PP`\xA45\x90\x81\x11a\0\xDCWa\x01|\x906\x90`\x04\x01a\0\xE1V[\0[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x03a\0\xDCWV[4a\0\xDCW` 6`\x03\x19\x01\x12a\0\xDCW` `\x045a\x01\xAF\x81a\x01~V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16c\x02q\x18\x97`\xE5\x1B\x81\x14\x90\x81\x15a\x01\xEDW[\x81\x15a\x01\xDCW[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P8a\x01\xD1V[c\n\x85\xBD\x01`\xE1\x1B\x81\x14\x91Pa\x01\xCAV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02(W`@RV[a\x01\xFEV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02(W`@RV[\x81`\x1F\x82\x01\x12\x15a\0\xDCW\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02(W`@Q\x92a\x02\x84`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x02-V[\x82\x84R` \x83\x83\x01\x01\x11a\0\xDCW\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[4a\0\xDCW` 6`\x03\x19\x01\x12a\0\xDCW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCWa\x02\xE2a\x02\xDC` \x926\x90`\x04\x01a\x02OV[3a\t\xC2V[`@Q\x90\x81R\xF3[4a\0\xDCW`\x806`\x03\x19\x01\x12a\0\xDCWa\x03\x06`\x045a\0\xCBV[a\x03\x11`$5a\0\xCBV[`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCWa\x031\x906\x90`\x04\x01a\0\xE1V[PP`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R` \x90\xF3[4a\0\xDCW`@6`\x03\x19\x01\x12a\0\xDCW`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCW` a\x03{a\x03\xC4\x926\x90`\x04\x01a\0\xE1V[`@\x92\x91\x92Q\x92a\x03\xA7\x84a\x03\x99`\x045\x86\x83\x01\x91\x90` \x83\x01\x92RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x02-V[`@Q\x94\x85\x92\x83\x92c \xC1;\x0B`\xE0\x1B\x96\x87\x85R`\x04\x85\x01a\n\xD0V[\x03\x813Z\xFA\x91\x82\x15a\x04PW`\0\x92a\x04 W[P`\0\x91`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x04\x17WPa\x04\x13c\x0B\x13]?`\xE1\x1B[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[a\x04\x13\x90a\x03\xF8V[a\x04B\x91\x92P` =\x81\x11a\x04IW[a\x04:\x81\x83a\x02-V[\x81\x01\x90a\n\xBBV[\x908a\x03\xD8V[P=a\x040V[a\t[V[4a\0\xDCW`@6`\x03\x19\x01\x12a\0\xDCWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\0\xDCWa\x04\x87\x906\x90`\x04\x01a\x02OV[\x90`$5\x90\x81\x11a\0\xDCWa\x04\xA0\x906\x90`\x04\x01a\x02OV[\x90a\x04\xAB\x813a\t\xC2V[\x90\x82Q\x15`\0\x14a\x051WP`@QcZ\xE6\xBD7`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90P` \x81`$\x813Z\xFA\x80\x15a\x04PWa\x04\xF1\x91`\0\x91a\x05\x03W[P\x15\x15a\tvV[`@Qc \xC1;\x0B`\xE0\x1B\x81R` \x90\xF3[a\x05$\x91P` =\x81\x11a\x05*W[a\x05\x1C\x81\x83a\x02-V[\x81\x01\x90a\tgV[8a\x04\xE9V[P=a\x05\x12V[3;\x15a\0\xDCW`\0\x91a\x05Y`@Q\x94\x85\x93\x84\x93c\x93O:\x11`\xE0\x1B\x85R`\x04\x85\x01a\t0V[\x03\x813Z\xFA\x80\x15a\x04PWa\x05oW[Pa\x04\xF1V[\x80a\x05|a\x05\x82\x92a\x02\x14V[\x80a\x05\xD1V[8a\x05iV[4a\0\xDCW`@6`\x03\x19\x01\x12a\0\xDCW`\x045a\x05\xA5\x81a\0\xCBV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCW` \x91a\x05\xCBa\x02\xE2\x926\x90`\x04\x01a\x02OV[\x90a\t\xC2V[`\0\x91\x03\x12a\0\xDCWV[` \x90\x81`@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91`\0[\x82\x81\x10a\x06\x03WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x05\xF5V[4a\0\xDCW`\0\x80`\x03\x196\x01\x12a\x07 W`@Qcf\x17\xC2)`\xE1\x1B\x81R`\x01`\x04\x82\x01R`\n`$\x82\x01R\x90\x80\x82`D\x813Z\xFA\x90\x81\x15a\x04PW\x80\x91a\x06rW[`@Q\x80a\x04\x13\x84\x82a\x05\xDCV[\x90P=\x80\x82\x84>a\x06\x83\x81\x84a\x02-V[\x82\x01\x91`@\x81\x84\x03\x12a\x07'W\x80Q\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x81\x11a\x07#W\x82\x01\x81`\x1F\x82\x01\x12\x15a\x07#W\x80Q\x94\x85\x11a\x02(W\x84`\x05\x1B\x90`@Q\x94` \x96a\x06\xD2\x88\x85\x01\x88a\x02-V[\x86R\x86\x80\x87\x01\x93\x83\x01\x01\x93\x84\x11a\x07 WP\x85\x01\x90[\x82\x82\x10a\x07\x07WPPPa\x04\x13\x92a\x07\0\x91\x01a\x0B\x0BV[P8a\x06dV[\x85\x80\x91\x83Qa\x07\x15\x81a\0\xCBV[\x81R\x01\x91\x01\x90a\x06\xE8V[\x80\xFD[\x83\x80\xFD[P\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\0\xDCW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xDCW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\0\xDCWV[4a\0\xDCW`\xA06`\x03\x19\x01\x12a\0\xDCWa\x07x`\x045a\0\xCBV[a\x07\x83`$5a\0\xCBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D5\x81\x81\x11a\0\xDCWa\x07\xA4\x906\x90`\x04\x01a\x07+V[PP`d5\x81\x81\x11a\0\xDCWa\x07\xBE\x906\x90`\x04\x01a\x07+V[PP`\x845\x90\x81\x11a\0\xDCWa\x07\xD8\x906\x90`\x04\x01a\0\xE1V[PP`@Qc\xBC\x19|\x81`\xE0\x1B\x81R` \x90\xF3[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x08\x18WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x07\xF7V[\x90` a\x08=\x92\x81\x81R\x01\x90a\x07\xECV[\x90V[4a\0\xDCW6`\x03\x19\x01`@\x81\x12a\0\xDCWa\x08]`\x045a\0\xCBV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCW` \x91a\x08\x82`\0\x926\x90`\x04\x01a\0\xE1V[PP`@Q\x90c\xB4\xFA\xBA\t`\xE0\x1B\x82R`\x04\x80\x83\x0176\x90\x823Z\xF1P=`@Q\x90`\x1F\x19\x90\x81\x81\x84\x01\x01`@R\x01` \x82>`\0Q\x15a\x08\xCDWa\x04\x13\x90`@Q\x91\x82\x91\x82a\x08,V[` \x81Q\x91\x01\xFD[4a\0\xDCW`\xA06`\x03\x19\x01\x12a\0\xDCWa\x08\xF1`\x045a\0\xCBV[a\x08\xFC`$5a\0\xCBV[`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCWa\t\x1C\x906\x90`\x04\x01a\0\xE1V[PP`@Qc\xF2:na`\xE0\x1B\x81R` \x90\xF3[\x91a\tM\x90a\x08=\x94\x92\x84R``` \x85\x01R``\x84\x01\x90a\x07\xECV[\x91`@\x81\x84\x03\x91\x01Ra\x07\xECV[`@Q=`\0\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\0\xDCWQ\x90V[\x15a\t}WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FHash not approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x80Q` \x80\x92\x01 \x90\x80`@Q\x92\x83a\n\x12\x83\x82\x01\x92\x83\x91\x90` `@\x84\x01\x93\x7F`\xB3\xCB\xF8\xB4\xA2#\xD6\x8Dd\x1B;m\xDF\x9A)\x8E\x7F3q\x0C\xF3\xD3\xA9\xD1\x14kZaP\xFB\xCA\x81R\x01RV[\x03\x93a\n&`\x1F\x19\x95\x86\x81\x01\x83R\x82a\x02-V[Q\x90 \x93`\x04`@Q\x80\x97\x81\x93c\xF6\x98\xDA%`\xE0\x1B\x83R\x16Z\xFA\x93\x84\x15a\x04PW`\0\x94a\n\x96W[P`@Q`\x19`\xF8\x1B\x91\x81\x01\x91\x82R`\x01`\xF8\x1B`\x01\x83\x01R`\x02\x82\x01\x94\x90\x94R`\"\x81\x01\x92\x90\x92R\x90\x91\x90a\n\x90\x90\x82`B\x85\x01\x03\x90\x81\x01\x83R\x82a\x02-V[Q\x90 \x90V[\x81\x94Pa\n\xB3\x90a\n\x90\x93\x92=\x87\x11a\x05*Wa\x05\x1C\x81\x83a\x02-V[\x93\x90\x91a\nOV[\x90\x81` \x91\x03\x12a\0\xDCWQa\x08=\x81a\x01~V[\x91\x92` \x93a\n\xE8\x82\x93`@\x86R`@\x86\x01\x90a\x07\xECV[\x93\x85\x81\x86\x03\x91\x01R\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[Q\x90a\x0B\x16\x82a\0\xCBV[V\xFE\xA2dipfsX\"\x12 \xC8%=\x10\x8DY\xAA\xF6\xE4\xDB\xDCj\x91\xB2\x8C\xA7\xE6\xF3x2\r\xE4\x1E?*\x01u8\xE2\xDC\xAA\x97dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static COMPATIBILITYFALLBACKHANDLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80b#\xDE)\x14a\0\xC6W\x80c\x01\xFF\xC9\xA7\x14a\0\xC1W\x80c\n\x10(\xC4\x14a\0\xBCW\x80c\x15\x0Bz\x02\x14a\0\xB7W\x80c\x16&\xBA~\x14a\0\xB2W\x80c \xC1;\x0B\x14a\0\xADW\x80cj\xC2G\x84\x14a\0\xA8W\x80c\xB2IM\xF3\x14a\0\xA3W\x80c\xBC\x19|\x81\x14a\0\x9EW\x80c\xBDa\x95\x1D\x14a\0\x99Wc\xF2:na\x14a\0\x94W`\0\x80\xFD[a\x08\xD5V[a\x08@V[a\x07\\V[a\x06 V[a\x05\x88V[a\x04UV[a\x03EV[a\x02\xEAV[a\x02\xA6V[a\x01\x90V[a\x01\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\0\xDCWV[`\0\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\0\xDCW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xDCW` \x83\x81\x86\x01\x95\x01\x01\x11a\0\xDCWV[4a\0\xDCW`\xC06`\x03\x19\x01\x12a\0\xDCWa\x01+`\x045a\0\xCBV[a\x016`$5a\0\xCBV[a\x01A`D5a\0\xCBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x845\x81\x81\x11a\0\xDCWa\x01b\x906\x90`\x04\x01a\0\xE1V[PP`\xA45\x90\x81\x11a\0\xDCWa\x01|\x906\x90`\x04\x01a\0\xE1V[\0[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x03a\0\xDCWV[4a\0\xDCW` 6`\x03\x19\x01\x12a\0\xDCW` `\x045a\x01\xAF\x81a\x01~V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16c\x02q\x18\x97`\xE5\x1B\x81\x14\x90\x81\x15a\x01\xEDW[\x81\x15a\x01\xDCW[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P8a\x01\xD1V[c\n\x85\xBD\x01`\xE1\x1B\x81\x14\x91Pa\x01\xCAV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02(W`@RV[a\x01\xFEV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02(W`@RV[\x81`\x1F\x82\x01\x12\x15a\0\xDCW\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02(W`@Q\x92a\x02\x84`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x02-V[\x82\x84R` \x83\x83\x01\x01\x11a\0\xDCW\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[4a\0\xDCW` 6`\x03\x19\x01\x12a\0\xDCW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCWa\x02\xE2a\x02\xDC` \x926\x90`\x04\x01a\x02OV[3a\t\xC2V[`@Q\x90\x81R\xF3[4a\0\xDCW`\x806`\x03\x19\x01\x12a\0\xDCWa\x03\x06`\x045a\0\xCBV[a\x03\x11`$5a\0\xCBV[`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCWa\x031\x906\x90`\x04\x01a\0\xE1V[PP`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R` \x90\xF3[4a\0\xDCW`@6`\x03\x19\x01\x12a\0\xDCW`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCW` a\x03{a\x03\xC4\x926\x90`\x04\x01a\0\xE1V[`@\x92\x91\x92Q\x92a\x03\xA7\x84a\x03\x99`\x045\x86\x83\x01\x91\x90` \x83\x01\x92RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x02-V[`@Q\x94\x85\x92\x83\x92c \xC1;\x0B`\xE0\x1B\x96\x87\x85R`\x04\x85\x01a\n\xD0V[\x03\x813Z\xFA\x91\x82\x15a\x04PW`\0\x92a\x04 W[P`\0\x91`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x04\x17WPa\x04\x13c\x0B\x13]?`\xE1\x1B[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[a\x04\x13\x90a\x03\xF8V[a\x04B\x91\x92P` =\x81\x11a\x04IW[a\x04:\x81\x83a\x02-V[\x81\x01\x90a\n\xBBV[\x908a\x03\xD8V[P=a\x040V[a\t[V[4a\0\xDCW`@6`\x03\x19\x01\x12a\0\xDCWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\0\xDCWa\x04\x87\x906\x90`\x04\x01a\x02OV[\x90`$5\x90\x81\x11a\0\xDCWa\x04\xA0\x906\x90`\x04\x01a\x02OV[\x90a\x04\xAB\x813a\t\xC2V[\x90\x82Q\x15`\0\x14a\x051WP`@QcZ\xE6\xBD7`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90P` \x81`$\x813Z\xFA\x80\x15a\x04PWa\x04\xF1\x91`\0\x91a\x05\x03W[P\x15\x15a\tvV[`@Qc \xC1;\x0B`\xE0\x1B\x81R` \x90\xF3[a\x05$\x91P` =\x81\x11a\x05*W[a\x05\x1C\x81\x83a\x02-V[\x81\x01\x90a\tgV[8a\x04\xE9V[P=a\x05\x12V[3;\x15a\0\xDCW`\0\x91a\x05Y`@Q\x94\x85\x93\x84\x93c\x93O:\x11`\xE0\x1B\x85R`\x04\x85\x01a\t0V[\x03\x813Z\xFA\x80\x15a\x04PWa\x05oW[Pa\x04\xF1V[\x80a\x05|a\x05\x82\x92a\x02\x14V[\x80a\x05\xD1V[8a\x05iV[4a\0\xDCW`@6`\x03\x19\x01\x12a\0\xDCW`\x045a\x05\xA5\x81a\0\xCBV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCW` \x91a\x05\xCBa\x02\xE2\x926\x90`\x04\x01a\x02OV[\x90a\t\xC2V[`\0\x91\x03\x12a\0\xDCWV[` \x90\x81`@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91`\0[\x82\x81\x10a\x06\x03WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x05\xF5V[4a\0\xDCW`\0\x80`\x03\x196\x01\x12a\x07 W`@Qcf\x17\xC2)`\xE1\x1B\x81R`\x01`\x04\x82\x01R`\n`$\x82\x01R\x90\x80\x82`D\x813Z\xFA\x90\x81\x15a\x04PW\x80\x91a\x06rW[`@Q\x80a\x04\x13\x84\x82a\x05\xDCV[\x90P=\x80\x82\x84>a\x06\x83\x81\x84a\x02-V[\x82\x01\x91`@\x81\x84\x03\x12a\x07'W\x80Q\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x81\x11a\x07#W\x82\x01\x81`\x1F\x82\x01\x12\x15a\x07#W\x80Q\x94\x85\x11a\x02(W\x84`\x05\x1B\x90`@Q\x94` \x96a\x06\xD2\x88\x85\x01\x88a\x02-V[\x86R\x86\x80\x87\x01\x93\x83\x01\x01\x93\x84\x11a\x07 WP\x85\x01\x90[\x82\x82\x10a\x07\x07WPPPa\x04\x13\x92a\x07\0\x91\x01a\x0B\x0BV[P8a\x06dV[\x85\x80\x91\x83Qa\x07\x15\x81a\0\xCBV[\x81R\x01\x91\x01\x90a\x06\xE8V[\x80\xFD[\x83\x80\xFD[P\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\0\xDCW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xDCW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\0\xDCWV[4a\0\xDCW`\xA06`\x03\x19\x01\x12a\0\xDCWa\x07x`\x045a\0\xCBV[a\x07\x83`$5a\0\xCBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D5\x81\x81\x11a\0\xDCWa\x07\xA4\x906\x90`\x04\x01a\x07+V[PP`d5\x81\x81\x11a\0\xDCWa\x07\xBE\x906\x90`\x04\x01a\x07+V[PP`\x845\x90\x81\x11a\0\xDCWa\x07\xD8\x906\x90`\x04\x01a\0\xE1V[PP`@Qc\xBC\x19|\x81`\xE0\x1B\x81R` \x90\xF3[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x08\x18WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x07\xF7V[\x90` a\x08=\x92\x81\x81R\x01\x90a\x07\xECV[\x90V[4a\0\xDCW6`\x03\x19\x01`@\x81\x12a\0\xDCWa\x08]`\x045a\0\xCBV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCW` \x91a\x08\x82`\0\x926\x90`\x04\x01a\0\xE1V[PP`@Q\x90c\xB4\xFA\xBA\t`\xE0\x1B\x82R`\x04\x80\x83\x0176\x90\x823Z\xF1P=`@Q\x90`\x1F\x19\x90\x81\x81\x84\x01\x01`@R\x01` \x82>`\0Q\x15a\x08\xCDWa\x04\x13\x90`@Q\x91\x82\x91\x82a\x08,V[` \x81Q\x91\x01\xFD[4a\0\xDCW`\xA06`\x03\x19\x01\x12a\0\xDCWa\x08\xF1`\x045a\0\xCBV[a\x08\xFC`$5a\0\xCBV[`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xDCWa\t\x1C\x906\x90`\x04\x01a\0\xE1V[PP`@Qc\xF2:na`\xE0\x1B\x81R` \x90\xF3[\x91a\tM\x90a\x08=\x94\x92\x84R``` \x85\x01R``\x84\x01\x90a\x07\xECV[\x91`@\x81\x84\x03\x91\x01Ra\x07\xECV[`@Q=`\0\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\0\xDCWQ\x90V[\x15a\t}WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FHash not approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x80Q` \x80\x92\x01 \x90\x80`@Q\x92\x83a\n\x12\x83\x82\x01\x92\x83\x91\x90` `@\x84\x01\x93\x7F`\xB3\xCB\xF8\xB4\xA2#\xD6\x8Dd\x1B;m\xDF\x9A)\x8E\x7F3q\x0C\xF3\xD3\xA9\xD1\x14kZaP\xFB\xCA\x81R\x01RV[\x03\x93a\n&`\x1F\x19\x95\x86\x81\x01\x83R\x82a\x02-V[Q\x90 \x93`\x04`@Q\x80\x97\x81\x93c\xF6\x98\xDA%`\xE0\x1B\x83R\x16Z\xFA\x93\x84\x15a\x04PW`\0\x94a\n\x96W[P`@Q`\x19`\xF8\x1B\x91\x81\x01\x91\x82R`\x01`\xF8\x1B`\x01\x83\x01R`\x02\x82\x01\x94\x90\x94R`\"\x81\x01\x92\x90\x92R\x90\x91\x90a\n\x90\x90\x82`B\x85\x01\x03\x90\x81\x01\x83R\x82a\x02-V[Q\x90 \x90V[\x81\x94Pa\n\xB3\x90a\n\x90\x93\x92=\x87\x11a\x05*Wa\x05\x1C\x81\x83a\x02-V[\x93\x90\x91a\nOV[\x90\x81` \x91\x03\x12a\0\xDCWQa\x08=\x81a\x01~V[\x91\x92` \x93a\n\xE8\x82\x93`@\x86R`@\x86\x01\x90a\x07\xECV[\x93\x85\x81\x86\x03\x91\x01R\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[Q\x90a\x0B\x16\x82a\0\xCBV[V\xFE\xA2dipfsX\"\x12 \xC8%=\x10\x8DY\xAA\xF6\xE4\xDB\xDCj\x91\xB2\x8C\xA7\xE6\xF3x2\r\xE4\x1E?*\x01u8\xE2\xDC\xAA\x97dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static COMPATIBILITYFALLBACKHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CompatibilityFallbackHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CompatibilityFallbackHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CompatibilityFallbackHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CompatibilityFallbackHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CompatibilityFallbackHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CompatibilityFallbackHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CompatibilityFallbackHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COMPATIBILITYFALLBACKHANDLER_ABI.clone(),
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
                COMPATIBILITYFALLBACKHANDLER_ABI.clone(),
                COMPATIBILITYFALLBACKHANDLER_BYTECODE.clone().into(),
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
        ///Calls the contract's `getMessageHashForSafe` (0x6ac24784) function
        pub fn get_message_hash_for_safe(
            &self,
            safe: ::ethers::core::types::Address,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([106, 194, 71, 132], (safe, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModules` (0xb2494df3) function
        pub fn get_modules(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([178, 73, 77, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x1626ba7e) function
        pub fn is_valid_signature(
            &self,
            data_hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([22, 38, 186, 126], (data_hash, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x20c13b0b) function
        pub fn is_valid_signature_with_data(
            &self,
            data: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([32, 193, 59, 11], (data, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulate` (0xbd61951d) function
        pub fn simulate(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([189, 97, 149, 29], (target_contract, calldata_payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokensReceived` (0x0023de29) function
        pub fn tokens_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Address,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
            p5: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 35, 222, 41], (p0, p1, p2, p3, p4, p5))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CompatibilityFallbackHandler<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Container type for all input parameters for the `getMessageHashForSafe` function with signature `getMessageHashForSafe(address,bytes)` and selector `0x6ac24784`
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
        name = "getMessageHashForSafe",
        abi = "getMessageHashForSafe(address,bytes)"
    )]
    pub struct GetMessageHashForSafeCall {
        pub safe: ::ethers::core::types::Address,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getModules` function with signature `getModules()` and selector `0xb2494df3`
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
    #[ethcall(name = "getModules", abi = "getModules()")]
    pub struct GetModulesCall;
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
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
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes32,bytes)")]
    pub struct IsValidSignatureCall {
        pub data_hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
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
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes,bytes)")]
    pub struct IsValidSignatureWithDataCall {
        pub data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `simulate` function with signature `simulate(address,bytes)` and selector `0xbd61951d`
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
    #[ethcall(name = "simulate", abi = "simulate(address,bytes)")]
    pub struct SimulateCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `tokensReceived` function with signature `tokensReceived(address,address,address,uint256,bytes,bytes)` and selector `0x0023de29`
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
        name = "tokensReceived",
        abi = "tokensReceived(address,address,address,uint256,bytes,bytes)"
    )]
    pub struct TokensReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CompatibilityFallbackHandlerCalls {
        GetMessageHash(GetMessageHashCall),
        GetMessageHashForSafe(GetMessageHashForSafeCall),
        GetModules(GetModulesCall),
        IsValidSignature(IsValidSignatureCall),
        IsValidSignatureWithData(IsValidSignatureWithDataCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        Simulate(SimulateCall),
        SupportsInterface(SupportsInterfaceCall),
        TokensReceived(TokensReceivedCall),
    }
    impl ::ethers::core::abi::AbiDecode for CompatibilityFallbackHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetMessageHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMessageHash(decoded));
            }
            if let Ok(decoded)
                = <GetMessageHashForSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetMessageHashForSafe(decoded));
            }
            if let Ok(decoded)
                = <GetModulesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetModules(decoded));
            }
            if let Ok(decoded)
                = <IsValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsValidSignature(decoded));
            }
            if let Ok(decoded)
                = <IsValidSignatureWithDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsValidSignatureWithData(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded)
                = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded)
                = <SimulateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Simulate(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TokensReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokensReceived(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CompatibilityFallbackHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetMessageHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMessageHashForSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetModules(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidSignatureWithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Simulate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokensReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CompatibilityFallbackHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetMessageHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMessageHashForSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetModules(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignatureWithData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Simulate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokensReceived(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetMessageHashCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: GetMessageHashCall) -> Self {
            Self::GetMessageHash(value)
        }
    }
    impl ::core::convert::From<GetMessageHashForSafeCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: GetMessageHashForSafeCall) -> Self {
            Self::GetMessageHashForSafe(value)
        }
    }
    impl ::core::convert::From<GetModulesCall> for CompatibilityFallbackHandlerCalls {
        fn from(value: GetModulesCall) -> Self {
            Self::GetModules(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: IsValidSignatureCall) -> Self {
            Self::IsValidSignature(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureWithDataCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: IsValidSignatureWithDataCall) -> Self {
            Self::IsValidSignatureWithData(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<SimulateCall> for CompatibilityFallbackHandlerCalls {
        fn from(value: SimulateCall) -> Self {
            Self::Simulate(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TokensReceivedCall>
    for CompatibilityFallbackHandlerCalls {
        fn from(value: TokensReceivedCall) -> Self {
            Self::TokensReceived(value)
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
    ///Container type for all return fields from the `getMessageHashForSafe` function with signature `getMessageHashForSafe(address,bytes)` and selector `0x6ac24784`
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
    pub struct GetMessageHashForSafeReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getModules` function with signature `getModules()` and selector `0xb2494df3`
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
    pub struct GetModulesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
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
    pub struct IsValidSignatureReturn(pub [u8; 4]);
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
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
    pub struct IsValidSignatureWithDataReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `simulate` function with signature `simulate(address,bytes)` and selector `0xbd61951d`
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
    pub struct SimulateReturn {
        pub response: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
