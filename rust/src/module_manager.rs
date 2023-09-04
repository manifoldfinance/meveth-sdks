pub use module_manager::*;
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
pub mod module_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("disableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("disableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevModule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("enableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("execTransactionFromModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModule",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "execTransactionFromModuleReturnData",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModuleReturnData",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
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
                    ::std::borrow::ToOwned::to_owned("getModulesPaginated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getModulesPaginated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pageSize"),
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
                                    name: ::std::borrow::ToOwned::to_owned("array"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("next"),
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
                    ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DisabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DisabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleFailure",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static MODULEMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x08\x94\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81c-\x9A\xD5=\x14a\0zWP\x80cF\x87!\xA7\x14a\0uW\x80cR)\x07?\x14a\0pW\x80ca\x0BY%\x14a\0kW\x80c\xCC/\x84R\x14a\0fWc\xE0\t\xCF\xDE\x14a\0aW`\0\x80\xFD[a\x05JV[a\x04MV[a\x02}V[a\x01\xFBV[a\x01\xD5V[4a\0\xCEW` 6`\x03\x19\x01\x12a\0\xCEW`\x01`\x01`\xA0\x1B\x03\x80a\0\x9Ca\0\xD1V[\x16\x91\x82`\x01\x14\x15\x92\x83a\0\xB6W[\x83\x15\x15`\x80R` `\x80\xF3[`@\x92\x93P\x81R\x80` R T\x16\x15\x15\x81\x80\x80a\0\xAAV[\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xE7WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xE7WV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x01>W`@RV[a\x01\x02V[`d5\x90`\x02\x82\x10\x15a\0\xE7WV[`\x80`\x03\x19\x82\x01\x12a\0\xE7Wa\x01fa\0\xD1V[\x91`$5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D5\x81\x81\x11a\0\xE7W\x82`#\x82\x01\x12\x15a\0\xE7W\x80`\x04\x015\x91\x82\x11a\x01>Wa\x01\xA8`\x1F\x83\x01`\x1F\x19\x16` \x01a\x01\x18V[\x92\x82\x84R`$\x83\x83\x01\x01\x11a\0\xE7W\x81`\0\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R\x90a\x01\xD2a\x01CV[\x90V[4a\0\xE7W` a\x01\xF1a\x01\xE86a\x01RV[\x92\x91\x90\x91a\x06\xB8V[`@Q\x90\x15\x15\x81R\xF3[4a\0\xE7Wa\x02\x0Ca\x01\xE86a\x01RV[`@Q` \x91\x82=\x83\x01\x01`@R=\x82R=`\0\x84\x84\x01>`@Q\x92\x83\x91\x15\x15\x82R`@\x81\x83\x01R\x82Q\x90\x81`@\x84\x01R`\0\x93[\x82\x85\x10a\x02dWPP``\x92P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xF3[\x84\x81\x01\x82\x01Q\x86\x86\x01``\x01R\x93\x81\x01\x93\x85\x93Pa\x02AV[4a\0\xE7W` 6`\x03\x19\x01\x12a\0\xE7Wa\x02\x96a\0\xD1V[a\x02\x9Ea\x08(V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x15\x15\x80a\x03\xEBW[a\x02\xBB\x90a\x06\x84V[`\0R`\0` R`@`\0 T\x16a\x03\xBEW`\x01`\0\x90\x81R` R\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x90a\x03\xB9\x90a\x03ka\x033\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x03P\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[`\x01`\0\x90\x81R` Ra\x03\x9F\x81\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}a\x03PV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[P`\x01\x81\x14\x15a\x02\xB2V[\x90\x92\x91\x92`@\x82\x01`@\x83R\x81Q\x80\x91R``\x83\x01` \x80\x93\x01\x91`\0[\x84\x82\x82\x10a\x040WPPP`\x01`\x01`\xA0\x1B\x03\x91P\x94\x16\x91\x01RV[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04\x14V[4a\0\xE7W`@6`\x03\x19\x01\x12a\0\xE7Wa\x04fa\0\xD1V[`$5\x90a\x04{a\x04v\x83a\x07\xC1V[a\x01\x18V[\x82\x81R`\x1F\x19a\x04\x8A\x84a\x07\xC1V[\x016` \x83\x017`\0\x92`\x01`\x01`\xA0\x1B\x03\x80\x93\x16\x84R\x83` Ra\x04\xBA`@\x85 `\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[\x83\x81\x16\x80\x15\x15\x90\x81a\x05>W[P\x80a\x055W[\x15a\x05\x1EWa\x05\x12a\x03&\x82a\x04\xF9a\x05\x18\x94a\x04\xEB\x8A\x89a\x07\xD9V[\x90`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x94a\x08\x03V[\x93a\x04\xBAV[\x82\x85\x81Ra\x051`@Q\x92\x83\x92\x83a\x03\xF6V[\x03\x90\xF3[P\x81\x85\x10a\x04\xCEV[`\x01\x91P\x14\x158a\x04\xC7V[4a\0\xE7W`@6`\x03\x19\x01\x12a\0\xE7Wa\x05ca\0\xD1V[a\x05ka\0\xECV[\x90a\x05ta\x08(V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x90\x81\x15\x15\x80a\x06yW[a\x05\x92\x90a\x06\x84V[\x80\x83\x16`\0R`\0` R`@`\0 T\x16\x03a\x06LW\x81a\x06\x0Fa\x03\xB9\x92a\x03Pa\x05\xF5a\x03&\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x97`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x03\x9Fa\x06/\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x90\xFD[P`\x01\x82\x14\x15a\x05\x89V[\x15a\x06\x8BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x91\x92\x90\x92`\x013\x14\x15\x80a\x07[W[\x15a\x07.Wa\x06\xD7\x93Z\x93a\x07{V[\x90\x81\x15a\x07\x06W3\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8`\0\x80\xA2V[3\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u`\0\x80\xA2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x90\xFD[P3`\0R`\0` R`\x01`\x01`\xA0\x1B\x03`@`\0 T\x16\x15\x15a\x06\xC7V[\x93\x90\x93`\x02\x84\x10\x15a\x07\xABW`\0\x94\x85\x94`\x01\x03a\x07\x9FWP` \x83Q\x93\x01\x91\xF4\x90V[\x90` \x84Q\x94\x01\x92\xF1\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01>W`\x05\x1B` \x01\x90V[\x80Q\x82\x10\x15a\x07\xEDW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x08\x12W`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[03\x03a\x081WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 ,u\xCB\x17\xDAf\x05\xFA'\xF1QPu65\xCC\xE9%Wbk\xB6K\x94D\xB0F\x8A\xD1\x89\x11\xABdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MODULEMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81c-\x9A\xD5=\x14a\0zWP\x80cF\x87!\xA7\x14a\0uW\x80cR)\x07?\x14a\0pW\x80ca\x0BY%\x14a\0kW\x80c\xCC/\x84R\x14a\0fWc\xE0\t\xCF\xDE\x14a\0aW`\0\x80\xFD[a\x05JV[a\x04MV[a\x02}V[a\x01\xFBV[a\x01\xD5V[4a\0\xCEW` 6`\x03\x19\x01\x12a\0\xCEW`\x01`\x01`\xA0\x1B\x03\x80a\0\x9Ca\0\xD1V[\x16\x91\x82`\x01\x14\x15\x92\x83a\0\xB6W[\x83\x15\x15`\x80R` `\x80\xF3[`@\x92\x93P\x81R\x80` R T\x16\x15\x15\x81\x80\x80a\0\xAAV[\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xE7WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xE7WV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x01>W`@RV[a\x01\x02V[`d5\x90`\x02\x82\x10\x15a\0\xE7WV[`\x80`\x03\x19\x82\x01\x12a\0\xE7Wa\x01fa\0\xD1V[\x91`$5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D5\x81\x81\x11a\0\xE7W\x82`#\x82\x01\x12\x15a\0\xE7W\x80`\x04\x015\x91\x82\x11a\x01>Wa\x01\xA8`\x1F\x83\x01`\x1F\x19\x16` \x01a\x01\x18V[\x92\x82\x84R`$\x83\x83\x01\x01\x11a\0\xE7W\x81`\0\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R\x90a\x01\xD2a\x01CV[\x90V[4a\0\xE7W` a\x01\xF1a\x01\xE86a\x01RV[\x92\x91\x90\x91a\x06\xB8V[`@Q\x90\x15\x15\x81R\xF3[4a\0\xE7Wa\x02\x0Ca\x01\xE86a\x01RV[`@Q` \x91\x82=\x83\x01\x01`@R=\x82R=`\0\x84\x84\x01>`@Q\x92\x83\x91\x15\x15\x82R`@\x81\x83\x01R\x82Q\x90\x81`@\x84\x01R`\0\x93[\x82\x85\x10a\x02dWPP``\x92P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xF3[\x84\x81\x01\x82\x01Q\x86\x86\x01``\x01R\x93\x81\x01\x93\x85\x93Pa\x02AV[4a\0\xE7W` 6`\x03\x19\x01\x12a\0\xE7Wa\x02\x96a\0\xD1V[a\x02\x9Ea\x08(V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x15\x15\x80a\x03\xEBW[a\x02\xBB\x90a\x06\x84V[`\0R`\0` R`@`\0 T\x16a\x03\xBEW`\x01`\0\x90\x81R` R\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x90a\x03\xB9\x90a\x03ka\x033\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x03P\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[`\x01`\0\x90\x81R` Ra\x03\x9F\x81\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}a\x03PV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[P`\x01\x81\x14\x15a\x02\xB2V[\x90\x92\x91\x92`@\x82\x01`@\x83R\x81Q\x80\x91R``\x83\x01` \x80\x93\x01\x91`\0[\x84\x82\x82\x10a\x040WPPP`\x01`\x01`\xA0\x1B\x03\x91P\x94\x16\x91\x01RV[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04\x14V[4a\0\xE7W`@6`\x03\x19\x01\x12a\0\xE7Wa\x04fa\0\xD1V[`$5\x90a\x04{a\x04v\x83a\x07\xC1V[a\x01\x18V[\x82\x81R`\x1F\x19a\x04\x8A\x84a\x07\xC1V[\x016` \x83\x017`\0\x92`\x01`\x01`\xA0\x1B\x03\x80\x93\x16\x84R\x83` Ra\x04\xBA`@\x85 `\x01`\x01`\xA0\x1B\x03\x90T\x16\x90V[\x83\x81\x16\x80\x15\x15\x90\x81a\x05>W[P\x80a\x055W[\x15a\x05\x1EWa\x05\x12a\x03&\x82a\x04\xF9a\x05\x18\x94a\x04\xEB\x8A\x89a\x07\xD9V[\x90`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x94a\x08\x03V[\x93a\x04\xBAV[\x82\x85\x81Ra\x051`@Q\x92\x83\x92\x83a\x03\xF6V[\x03\x90\xF3[P\x81\x85\x10a\x04\xCEV[`\x01\x91P\x14\x158a\x04\xC7V[4a\0\xE7W`@6`\x03\x19\x01\x12a\0\xE7Wa\x05ca\0\xD1V[a\x05ka\0\xECV[\x90a\x05ta\x08(V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x90\x81\x15\x15\x80a\x06yW[a\x05\x92\x90a\x06\x84V[\x80\x83\x16`\0R`\0` R`@`\0 T\x16\x03a\x06LW\x81a\x06\x0Fa\x03\xB9\x92a\x03Pa\x05\xF5a\x03&\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x97`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x03\x9Fa\x06/\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x90\xFD[P`\x01\x82\x14\x15a\x05\x89V[\x15a\x06\x8BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x91\x92\x90\x92`\x013\x14\x15\x80a\x07[W[\x15a\x07.Wa\x06\xD7\x93Z\x93a\x07{V[\x90\x81\x15a\x07\x06W3\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8`\0\x80\xA2V[3\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u`\0\x80\xA2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x90\xFD[P3`\0R`\0` R`\x01`\x01`\xA0\x1B\x03`@`\0 T\x16\x15\x15a\x06\xC7V[\x93\x90\x93`\x02\x84\x10\x15a\x07\xABW`\0\x94\x85\x94`\x01\x03a\x07\x9FWP` \x83Q\x93\x01\x91\xF4\x90V[\x90` \x84Q\x94\x01\x92\xF1\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01>W`\x05\x1B` \x01\x90V[\x80Q\x82\x10\x15a\x07\xEDW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x08\x12W`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[03\x03a\x081WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 ,u\xCB\x17\xDAf\x05\xFA'\xF1QPu65\xCC\xE9%Wbk\xB6K\x94D\xB0F\x8A\xD1\x89\x11\xABdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MODULEMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ModuleManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ModuleManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ModuleManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ModuleManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ModuleManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ModuleManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ModuleManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MODULEMANAGER_ABI.clone(),
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
                MODULEMANAGER_ABI.clone(),
                MODULEMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `disableModule` (0xe009cfde) function
        pub fn disable_module(
            &self,
            prev_module: ::ethers::core::types::Address,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 9, 207, 222], (prev_module, module))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableModule` (0x610b5925) function
        pub fn enable_module(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 11, 89, 37], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModule` (0x468721a7) function
        pub fn exec_transaction_from_module(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 135, 33, 167], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModuleReturnData` (0x5229073f) function
        pub fn exec_transaction_from_module_return_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([82, 41, 7, 63], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModulesPaginated` (0xcc2f8452) function
        pub fn get_modules_paginated(
            &self,
            start: ::ethers::core::types::Address,
            page_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([204, 47, 132, 82], (start, page_size))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isModuleEnabled` (0x2d9ad53d) function
        pub fn is_module_enabled(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 154, 213, 61], module)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DisabledModule` event
        pub fn disabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnabledModule` event
        pub fn enabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleFailure` event
        pub fn execution_from_module_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleSuccess` event
        pub fn execution_from_module_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleSuccessFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ModuleManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ModuleManager<M> {
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
    #[ethevent(name = "DisabledModule", abi = "DisabledModule(address)")]
    pub struct DisabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "EnabledModule", abi = "EnabledModule(address)")]
    pub struct EnabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "ExecutionFromModuleFailure",
        abi = "ExecutionFromModuleFailure(address)"
    )]
    pub struct ExecutionFromModuleFailureFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "ExecutionFromModuleSuccess",
        abi = "ExecutionFromModuleSuccess(address)"
    )]
    pub struct ExecutionFromModuleSuccessFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ModuleManagerEvents {
        DisabledModuleFilter(DisabledModuleFilter),
        EnabledModuleFilter(EnabledModuleFilter),
        ExecutionFromModuleFailureFilter(ExecutionFromModuleFailureFilter),
        ExecutionFromModuleSuccessFilter(ExecutionFromModuleSuccessFilter),
    }
    impl ::ethers::contract::EthLogDecode for ModuleManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DisabledModuleFilter::decode_log(log) {
                return Ok(ModuleManagerEvents::DisabledModuleFilter(decoded));
            }
            if let Ok(decoded) = EnabledModuleFilter::decode_log(log) {
                return Ok(ModuleManagerEvents::EnabledModuleFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleFailureFilter::decode_log(log) {
                return Ok(
                    ModuleManagerEvents::ExecutionFromModuleFailureFilter(decoded),
                );
            }
            if let Ok(decoded) = ExecutionFromModuleSuccessFilter::decode_log(log) {
                return Ok(
                    ModuleManagerEvents::ExecutionFromModuleSuccessFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ModuleManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DisabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DisabledModuleFilter> for ModuleManagerEvents {
        fn from(value: DisabledModuleFilter) -> Self {
            Self::DisabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<EnabledModuleFilter> for ModuleManagerEvents {
        fn from(value: EnabledModuleFilter) -> Self {
            Self::EnabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleFailureFilter>
    for ModuleManagerEvents {
        fn from(value: ExecutionFromModuleFailureFilter) -> Self {
            Self::ExecutionFromModuleFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleSuccessFilter>
    for ModuleManagerEvents {
        fn from(value: ExecutionFromModuleSuccessFilter) -> Self {
            Self::ExecutionFromModuleSuccessFilter(value)
        }
    }
    ///Container type for all input parameters for the `disableModule` function with signature `disableModule(address,address)` and selector `0xe009cfde`
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
    #[ethcall(name = "disableModule", abi = "disableModule(address,address)")]
    pub struct DisableModuleCall {
        pub prev_module: ::ethers::core::types::Address,
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `enableModule` function with signature `enableModule(address)` and selector `0x610b5925`
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
    #[ethcall(name = "enableModule", abi = "enableModule(address)")]
    pub struct EnableModuleCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
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
        name = "execTransactionFromModule",
        abi = "execTransactionFromModule(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
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
        name = "execTransactionFromModuleReturnData",
        abi = "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleReturnDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
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
        name = "getModulesPaginated",
        abi = "getModulesPaginated(address,uint256)"
    )]
    pub struct GetModulesPaginatedCall {
        pub start: ::ethers::core::types::Address,
        pub page_size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    #[ethcall(name = "isModuleEnabled", abi = "isModuleEnabled(address)")]
    pub struct IsModuleEnabledCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ModuleManagerCalls {
        DisableModule(DisableModuleCall),
        EnableModule(EnableModuleCall),
        ExecTransactionFromModule(ExecTransactionFromModuleCall),
        ExecTransactionFromModuleReturnData(ExecTransactionFromModuleReturnDataCall),
        GetModulesPaginated(GetModulesPaginatedCall),
        IsModuleEnabled(IsModuleEnabledCall),
    }
    impl ::ethers::core::abi::AbiDecode for ModuleManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DisableModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableModule(decoded));
            }
            if let Ok(decoded)
                = <EnableModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableModule(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecTransactionFromModule(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleReturnDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecTransactionFromModuleReturnData(decoded));
            }
            if let Ok(decoded)
                = <GetModulesPaginatedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetModulesPaginated(decoded));
            }
            if let Ok(decoded)
                = <IsModuleEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsModuleEnabled(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ModuleManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DisableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetModulesPaginated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsModuleEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ModuleManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DisableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecTransactionFromModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetModulesPaginated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsModuleEnabled(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DisableModuleCall> for ModuleManagerCalls {
        fn from(value: DisableModuleCall) -> Self {
            Self::DisableModule(value)
        }
    }
    impl ::core::convert::From<EnableModuleCall> for ModuleManagerCalls {
        fn from(value: EnableModuleCall) -> Self {
            Self::EnableModule(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleCall> for ModuleManagerCalls {
        fn from(value: ExecTransactionFromModuleCall) -> Self {
            Self::ExecTransactionFromModule(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleReturnDataCall>
    for ModuleManagerCalls {
        fn from(value: ExecTransactionFromModuleReturnDataCall) -> Self {
            Self::ExecTransactionFromModuleReturnData(value)
        }
    }
    impl ::core::convert::From<GetModulesPaginatedCall> for ModuleManagerCalls {
        fn from(value: GetModulesPaginatedCall) -> Self {
            Self::GetModulesPaginated(value)
        }
    }
    impl ::core::convert::From<IsModuleEnabledCall> for ModuleManagerCalls {
        fn from(value: IsModuleEnabledCall) -> Self {
            Self::IsModuleEnabled(value)
        }
    }
    ///Container type for all return fields from the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
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
    pub struct ExecTransactionFromModuleReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
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
    pub struct ExecTransactionFromModuleReturnDataReturn {
        pub success: bool,
        pub return_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
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
    pub struct GetModulesPaginatedReturn {
        pub array: ::std::vec::Vec<::ethers::core::types::Address>,
        pub next: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    pub struct IsModuleEnabledReturn(pub bool);
}
