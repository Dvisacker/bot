/**

Generated by the following Solidity interface...
```solidity
interface GetERC20TokenDataBatchRequest {
    constructor(address[] tokens);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "tokens",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "nonpayable"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod GetERC20TokenDataBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50604051610b38380380610b388339818101604052810190610031919061074e565b5f815167ffffffffffffffff81111561004d5761004c6105b8565b5b60405190808252806020026020018201604052801561008657816020015b610073610555565b81526020019060019003908161006b5790505b5090505f5b8251811015610505575f8382815181106100a8576100a7610795565b5b602002602001015190506100c18161053360201b60201c565b156100cc57506104fa565b6100d4610555565b81815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250505f808373ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516101b79190610814565b5f604051808303815f8787f1925050503d805f81146101f1576040519150601f19603f3d011682016040523d82523d5f602084013e6101f6565b606091505b509150915081801561020857505f8151115b1561022e578080602001905181019061022191906108cc565b8360200181905250610237565b505050506104fa565b5f808573ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516102e39190610814565b5f604051808303815f8787f1925050503d805f811461031d576040519150601f19603f3d011682016040523d82523d5f602084013e610322565b606091505b50915091508115610395575f6020825103610383578180602001905181019061034b9190610946565b90505f81148061035b575060ff81115b1561036c57505050505050506104fa565b80866040019060ff16908160ff168152505061038f565b505050505050506104fa565b506103a0565b5050505050506104fa565b5f808773ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f18160ddd000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161044c9190610814565b5f604051808303815f8787f1925050503d805f8114610486576040519150601f19603f3d011682016040523d82523d5f602084013e61048b565b606091505b509150915081801561049e575060208151145b156104c557808060200190518101906104b79190610946565b8760600181815250506104d2565b50505050505050506104fa565b868a8a815181106104e6576104e5610795565b5b602002602001018190525050505050505050505b80600101905061008b565b505f816040516020016105189190610b17565b60405160208183030381529060405290506020810180590381f35b5f808273ffffffffffffffffffffffffffffffffffffffff163b149050919050565b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff168152602001606081526020015f60ff1681526020015f81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6105ee826105a8565b810181811067ffffffffffffffff8211171561060d5761060c6105b8565b5b80604052505050565b5f61061f610593565b905061062b82826105e5565b919050565b5f67ffffffffffffffff82111561064a576106496105b8565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6106888261065f565b9050919050565b6106988161067e565b81146106a2575f80fd5b50565b5f815190506106b38161068f565b92915050565b5f6106cb6106c684610630565b610616565b905080838252602082019050602084028301858111156106ee576106ed61065b565b5b835b81811015610717578061070388826106a5565b8452602084019350506020810190506106f0565b5050509392505050565b5f82601f830112610735576107346105a4565b5b81516107458482602086016106b9565b91505092915050565b5f602082840312156107635761076261059c565b5b5f82015167ffffffffffffffff8111156107805761077f6105a0565b5b61078c84828501610721565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f6107ee826107c2565b6107f881856107cc565b93506108088185602086016107d6565b80840191505092915050565b5f61081f82846107e4565b915081905092915050565b5f80fd5b5f67ffffffffffffffff821115610848576108476105b8565b5b610851826105a8565b9050602081019050919050565b5f61087061086b8461082e565b610616565b90508281526020810184848401111561088c5761088b61082a565b5b6108978482856107d6565b509392505050565b5f82601f8301126108b3576108b26105a4565b5b81516108c384826020860161085e565b91505092915050565b5f602082840312156108e1576108e061059c565b5b5f82015167ffffffffffffffff8111156108fe576108fd6105a0565b5b61090a8482850161089f565b91505092915050565b5f819050919050565b61092581610913565b811461092f575f80fd5b50565b5f815190506109408161091c565b92915050565b5f6020828403121561095b5761095a61059c565b5b5f61096884828501610932565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6109a38161067e565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f6109cd826109a9565b6109d781856109b3565b93506109e78185602086016107d6565b6109f0816105a8565b840191505092915050565b5f60ff82169050919050565b610a10816109fb565b82525050565b610a1f81610913565b82525050565b5f608083015f830151610a3a5f86018261099a565b5060208301518482036020860152610a5282826109c3565b9150506040830151610a676040860182610a07565b506060830151610a7a6060860182610a16565b508091505092915050565b5f610a908383610a25565b905092915050565b5f602082019050919050565b5f610aae82610971565b610ab8818561097b565b935083602082028501610aca8561098b565b805f5b85811015610b055784840389528151610ae68582610a85565b9450610af183610a98565b925060208a01995050600181019050610acd565b50829750879550505050505092915050565b5f6020820190508181035f830152610b2f8184610aa4565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x0B88\x03\x80a\x0B8\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x07NV[_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0MWa\0La\x05\xB8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x86W\x81` \x01[a\0sa\x05UV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x05\x05W_\x83\x82\x81Q\x81\x10a\0\xA8Wa\0\xA7a\x07\x95V[[` \x02` \x01\x01Q\x90Pa\0\xC1\x81a\x053` \x1B` \x1CV[\x15a\0\xCCWPa\x04\xFAV[a\0\xD4a\x05UV[\x81\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x01\xB7\x91\x90a\x08\x14V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x01\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01\xF6V[``\x91P[P\x91P\x91P\x81\x80\x15a\x02\x08WP_\x81Q\x11[\x15a\x02.W\x80\x80` \x01\x90Q\x81\x01\x90a\x02!\x91\x90a\x08\xCCV[\x83` \x01\x81\x90RPa\x027V[PPPPa\x04\xFAV[_\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x02\xE3\x91\x90a\x08\x14V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\"V[``\x91P[P\x91P\x91P\x81\x15a\x03\x95W_` \x82Q\x03a\x03\x83W\x81\x80` \x01\x90Q\x81\x01\x90a\x03K\x91\x90a\tFV[\x90P_\x81\x14\x80a\x03[WP`\xFF\x81\x11[\x15a\x03lWPPPPPPPa\x04\xFAV[\x80\x86`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\x8FV[PPPPPPPa\x04\xFAV[Pa\x03\xA0V[PPPPPPa\x04\xFAV[_\x80\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04L\x91\x90a\x08\x14V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\x8BV[``\x91P[P\x91P\x91P\x81\x80\x15a\x04\x9EWP` \x81Q\x14[\x15a\x04\xC5W\x80\x80` \x01\x90Q\x81\x01\x90a\x04\xB7\x91\x90a\tFV[\x87``\x01\x81\x81RPPa\x04\xD2V[PPPPPPPPa\x04\xFAV[\x86\x8A\x8A\x81Q\x81\x10a\x04\xE6Wa\x04\xE5a\x07\x95V[[` \x02` \x01\x01\x81\x90RPPPPPPPPP[\x80`\x01\x01\x90Pa\0\x8BV[P_\x81`@Q` \x01a\x05\x18\x91\x90a\x0B\x17V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14\x90P\x91\x90PV[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01_`\xFF\x16\x81R` \x01_\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x05\xEE\x82a\x05\xA8V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\rWa\x06\x0Ca\x05\xB8V[[\x80`@RPPPV[_a\x06\x1Fa\x05\x93V[\x90Pa\x06+\x82\x82a\x05\xE5V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06JWa\x06Ia\x05\xB8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x06\x88\x82a\x06_V[\x90P\x91\x90PV[a\x06\x98\x81a\x06~V[\x81\x14a\x06\xA2W_\x80\xFD[PV[_\x81Q\x90Pa\x06\xB3\x81a\x06\x8FV[\x92\x91PPV[_a\x06\xCBa\x06\xC6\x84a\x060V[a\x06\x16V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x06\xEEWa\x06\xEDa\x06[V[[\x83[\x81\x81\x10\x15a\x07\x17W\x80a\x07\x03\x88\x82a\x06\xA5V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x06\xF0V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x075Wa\x074a\x05\xA4V[[\x81Qa\x07E\x84\x82` \x86\x01a\x06\xB9V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x07cWa\x07ba\x05\x9CV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x80Wa\x07\x7Fa\x05\xA0V[[a\x07\x8C\x84\x82\x85\x01a\x07!V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x07\xEE\x82a\x07\xC2V[a\x07\xF8\x81\x85a\x07\xCCV[\x93Pa\x08\x08\x81\x85` \x86\x01a\x07\xD6V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x08\x1F\x82\x84a\x07\xE4V[\x91P\x81\x90P\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08HWa\x08Ga\x05\xB8V[[a\x08Q\x82a\x05\xA8V[\x90P` \x81\x01\x90P\x91\x90PV[_a\x08pa\x08k\x84a\x08.V[a\x06\x16V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x08\x8CWa\x08\x8Ba\x08*V[[a\x08\x97\x84\x82\x85a\x07\xD6V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x08\xB3Wa\x08\xB2a\x05\xA4V[[\x81Qa\x08\xC3\x84\x82` \x86\x01a\x08^V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x08\xE1Wa\x08\xE0a\x05\x9CV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xFEWa\x08\xFDa\x05\xA0V[[a\t\n\x84\x82\x85\x01a\x08\x9FV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\t%\x81a\t\x13V[\x81\x14a\t/W_\x80\xFD[PV[_\x81Q\x90Pa\t@\x81a\t\x1CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t[Wa\tZa\x05\x9CV[[_a\th\x84\x82\x85\x01a\t2V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\t\xA3\x81a\x06~V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\t\xCD\x82a\t\xA9V[a\t\xD7\x81\x85a\t\xB3V[\x93Pa\t\xE7\x81\x85` \x86\x01a\x07\xD6V[a\t\xF0\x81a\x05\xA8V[\x84\x01\x91PP\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\n\x10\x81a\t\xFBV[\x82RPPV[a\n\x1F\x81a\t\x13V[\x82RPPV[_`\x80\x83\x01_\x83\x01Qa\n:_\x86\x01\x82a\t\x9AV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra\nR\x82\x82a\t\xC3V[\x91PP`@\x83\x01Qa\ng`@\x86\x01\x82a\n\x07V[P``\x83\x01Qa\nz``\x86\x01\x82a\n\x16V[P\x80\x91PP\x92\x91PPV[_a\n\x90\x83\x83a\n%V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\n\xAE\x82a\tqV[a\n\xB8\x81\x85a\t{V[\x93P\x83` \x82\x02\x85\x01a\n\xCA\x85a\t\x8BV[\x80_[\x85\x81\x10\x15a\x0B\x05W\x84\x84\x03\x89R\x81Qa\n\xE6\x85\x82a\n\x85V[\x94Pa\n\xF1\x83a\n\x98V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\n\xCDV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0B/\x81\x84a\n\xA4V[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040525f80fdfea26469706673582212206de7f13688923ce68787b47e6e9899fc80242550b212d6eef07d84a1cf4ee51364736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 m\xE7\xF16\x88\x92<\xE6\x87\x87\xB4~n\x98\x99\xFC\x80$%P\xB2\x12\xD6\xEE\xF0}\x84\xA1\xCFN\xE5\x13dsolcC\0\x08\x1A\x003",
    );
    /**Constructor`.
```solidity
constructor(address[] tokens);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.tokens,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tokens: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GetERC20TokenDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetERC20TokenDataBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetERC20TokenDataBatchRequestInstance<T, P, N> {
        GetERC20TokenDataBatchRequestInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<GetERC20TokenDataBatchRequestInstance<T, P, N>>,
    > {
        GetERC20TokenDataBatchRequestInstance::<T, P, N>::deploy(provider, tokens)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        GetERC20TokenDataBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, tokens)
    }
    /**A [`GetERC20TokenDataBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetERC20TokenDataBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetERC20TokenDataBatchRequestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetERC20TokenDataBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetERC20TokenDataBatchRequestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetERC20TokenDataBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetERC20TokenDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetERC20TokenDataBatchRequestInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::Result<GetERC20TokenDataBatchRequestInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, tokens);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { tokens },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> GetERC20TokenDataBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> GetERC20TokenDataBatchRequestInstance<T, P, N> {
            GetERC20TokenDataBatchRequestInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetERC20TokenDataBatchRequestInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetERC20TokenDataBatchRequestInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
