use super::{data::DataSerializer, sigma_byte_writer::SigmaByteWrite};
use crate::mir::constant::Constant;
use crate::serialization::types::TypeCode;
use crate::serialization::SigmaSerializeResult;
use crate::serialization::{
    sigma_byte_reader::SigmaByteRead, SigmaParsingError, SigmaSerializable,
};
use crate::types::stype::SType;

impl<'ctx> Constant<'ctx> {
    /// Parse constant from byte stream. This function should be used instead of
    /// `sigma_parse` when type code is already read for look-ahead
    pub fn parse_with_type_code<R: SigmaByteRead>(
        r: &mut R,
        t_code: TypeCode,
    ) -> Result<Self, SigmaParsingError> {
        let tpe = SType::parse_with_type_code(r, t_code)?;
        let v = DataSerializer::sigma_parse(&tpe, r)?;
        Ok(Constant { tpe, v })
    }
}
impl SigmaSerializable for Constant<'_> {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> SigmaSerializeResult {
        self.tpe.sigma_serialize(w)?;
        DataSerializer::sigma_serialize(&self.v, w)
    }

    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SigmaParsingError> {
        // for reference see http://github.com/ScorexFoundation/sigmastate-interpreter/blob/25251c1313b0131835f92099f02cef8a5d932b5e/sigmastate/src/main/scala/sigmastate/serialization/DataSerializer.scala#L84-L84
        let t_code = TypeCode::sigma_parse(r)?;
        Self::parse_with_type_code(r, t_code)
    }
}

#[cfg(test)]
#[cfg(feature = "arbitrary")]
#[allow(clippy::panic, clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::mir::constant::arbitrary::ArbConstantParams;
    use crate::serialization::sigma_serialize_roundtrip;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn ser_roundtrip(v in any_with::<Constant>(ArbConstantParams::AnyWithDepth(4))) {
            prop_assert_eq![sigma_serialize_roundtrip(&v), v];
        }

        #[test]
        fn ser_roundtrip_sbox(v in any_with::<Constant>(ArbConstantParams::Exact(SType::SBox))) {
            prop_assert_eq![sigma_serialize_roundtrip(&v), v];
        }
    }

    #[test]
    fn parse_register_coll_box_issue_695_incorrect_method_id_and_missing_type_spec() {
        // regression test for messed up method id (AvlTree.update <-> remove)
        // and missing type specialization for ProperyCall
        // see https://github.com/ergoplatform/sigma-rust/issues/695
        let constant_bytes_str = "0c63028092f401104904000e200137c91882b759ad46a20e39aa4d035ce32525dc76d021ee643e71d09446400f04020e20f6ff8b7210015545d4b3ac5fc60c908092d035a1a16155c029e8d511627c7a2c0e20efc4f603dea6041286a89f5bd516ac96ea5b25da4f08d76c6927e01d61b22adf040204000402040004000402040c044c04010404040404020e20f5918eb4b0283c669bdd8a195640766c19e40a693a6697b775b08e09052523d40e20767caa80b98e496ad8a9f689c4410ae453327f0f95e95084c0ae206350793b7704000402040004020412040005809bee0204000400040004000402040404000402041205d00f040304000402040204420580897a0e20012aec95af24812a01775de090411ba70a648fe859013f896ca2a1a95882ce5f040204040400041004100402041005000402040004100400040004000400040004100410040204100402040205000404040404020402040404040100d80dd601db6501fed602b27201730000d6037301d604b27201730200d605dc640be4c6720204640283020e73037304e4e3000ed606e4c6a70410d607b27206730500d608b2a5730600d609e4c672080410d60ab27209730700d60be3044005d60ce4720bd60d8c720c01d196830301938cb2db63087202730800017203938cb2db6308720473090001b4e4b27205730a00730b730c95ed947207720a937207730dd80cd60eb27201730e00d60fdb6308a7d610e4c6a70511d611720bd612720cd613b47210730fb17210d6148c721202d615b2a5731000d616dc640be4c6720e04640283020e73117312e4e3010ed617b2db63087215731300d6188cb2720f73140002d6197cb4e4b272167315007316731796830401938cb2db6308720e7318000172039683080193c27208c2a792c1720899c1a7731993b2db63087208731a00b2720f731b0093b27209731c00b27206731d0093e4c672080511721093e4c672080664e4c6a7066493720a9591b27210731e009d9cb2e4c672040511731f007cb4e4b27205732000732173227323720d7324edafdb0c0e7213d9011a049593721a720d93b27213721a00721490b27213721a00721491b17213720d91db6903db6503feb272107325009683040193cbc27215b4e4b272167326007327732892c172157329938c721701732a928c72170295927218721972187219d802d60ee4c6a70511d60fe4c6720805119594720e720fd809d610b2a4732b00d611e4c6b2a4732c00050ed612adb4db0c0e7211732d9db17211732ed90112047cb472119c7212732f9c9a721273307331d613b072127332d90113599a8c7213018c721302d614e4c6a70664d615e4c67210050ed616dc640a7214027215e4e3010ed617e67216d618e4e3020e96830801927cb4e4dc640ae4c672040464028cb2db6308721073330001e4e3030e73347335721393c27208c2a792c17208c1a793b2db63087208733600b2db6308a7733700937209720693b2720f733800b2720e733900957217d802d619e47216d61aadb4db0c0e7219733a9db17219733bd9011a047cb472199c721a733c9c9a721a733d733e9683020193b2720f733f009a99b2720e734000b0721a7341d9011b599a8c721b018c721b02721393b4720f7342b1720faddc0c1db4720e7343b1720e01addc0c1d721a017212d9011b59998c721b028c721b01d9011b599a8c721b018c721b029683020193b2720f7344009ab2720e734500721393b4720f7346b1720faddc0c1db4720e7347b1720e017212d90119599a8c7219018c72190293db6401e4c672080664db6401957217e4dc640d72140283013c0e0e8602721572117218e4dc640c72140283013c0e0e86027215721172187348efc13b02010b4858ce0425ed4748d0d3a59f2dbf874166a2caaf734655ac5e3f88a68cdd01012aec95af24812a01775de090411ba70a648fe859013f896ca2a1a95882ce5f904e0310020001110400000000644ec61f485b98eb87153f7c57db4f5ecd75556fddbc403b41acf8441fde8e1609000720003b6fd893695e655e17804641c3ed2074731ff34281dafb0e5b50688d0627717300c0843d10230400040204000402040604040500050004000e200137c91882b759ad46a20e39aa4d035ce32525dc76d021ee643e71d09446400f04000e20010b4858ce0425ed4748d0d3a59f2dbf874166a2caaf734655ac5e3f88a68cdd0400040204080400040204040502040604080400040004020402040004020e20c7c537e6c635930ecb4ace95a54926b3ab77698d9f4922f0b1c58ea87156483b0400040204420404040205000502d80ed601db6501fed602b27201730000d603b27201730100d604e4c672030410d605e4c6a70411d606b27205730200d607b27205730300d608b27205730400d609b27205730500d60a9172097306d60be4c6a7050c63d60cb1720bd60db1a5d60ed9010e0c63b0dc0c0f720e01d9011063db630872107307d90110414d0e9a8c7210018c8c72100202d196830701938cb2db6308720273080001730996830301938cb2db63087203730a0001730b937eb27204730c00057206937eb27204730d0005720792db6903db6503fe720895720ad804d60fe4c6a7050c63d610b2a5b1720f00d611e4c672100411d612b27205730e009683090192c17210c1a793db63087210db6308a793b27211730f00720693b27211731000720793b27211731100997209731293b272117313009a7208721293b27211731400721293e4c67210050c63720f93c27210c2a7efaea5d9010f63aedb6308720fd901114d0e938c7211018cb2db6308a773150001afdc0c1d720b01b4a5731699720c7317d9010f3c636393c48c720f01c48c720f0293720d9a9a720c95720a731873199593cbc2b2a599720d731a00b4e4b2dc640be4c6720204640283010e731be4e3000e731c00731d731e731f732093da720e01a49ada720e01a595720a73217322efc13b0100b44a84993674c57c4fc23c6c1bb221470463e4e711b2260ffd8ed01f1aab420102110500020000000c6301c0843d0008cd02e4cb952261186ec0fd2dc4c2baa8dbfd9c8f6012c5efa9f702f9450a58fe221eefc13b01012aec95af24812a01775de090411ba70a648fe859013f896ca2a1a95882ce5fc0843d00c9ffc2d2fd948bd3217a245eab2c9d829abd0cd3b41a8069006ff2bb38b06e2d00cb93d676f11faa9b7204a875fdb34d55619fc0e933ae956af8c7d8ce4c9e20ca00";
        let constant_bytes = base16::decode(constant_bytes_str).unwrap();
        let c_res = Constant::sigma_parse_bytes(&constant_bytes);
        if let Err(e) = &c_res {
            println!("Failed to parse constant: {}", e);
        }
        assert!(c_res.is_ok());
        assert_eq!(c_res.unwrap().tpe, SType::SColl(Box::new(SType::SBox)));
    }

    #[test]
    fn test_parse_r5_in_698() {
        let constant_bytes_str = "0c63028092f401104904000e200137c91882b759ad46a20e39aa4d035ce32525dc76d021ee643e71d09446400f04020e20f6ff8b7210015545d4b3ac5fc60c908092d035a1a16155c029e8d511627c7a2c0e20efc4f603dea6041286a89f5bd516ac96ea5b25da4f08d76c6927e01d61b22adf040204000402040004000402040c044c04010404040404020e20f5918eb4b0283c669bdd8a195640766c19e40a693a6697b775b08e09052523d40e20767caa80b98e496ad8a9f689c4410ae453327f0f95e95084c0ae206350793b7704000402040004020412040005809bee0204000400040004000402040404000402041205d00f040304000402040204420580897a0e20012aec95af24812a01775de090411ba70a648fe859013f896ca2a1a95882ce5f040204040400041004100402041005000402040004100400040004000400040004100410040204100402040205000404040404020402040404040100d80dd601db6501fed602b27201730000d6037301d604b27201730200d605dc640be4c6720204640283020e73037304e4e3000ed606e4c6a70410d607b27206730500d608b2a5730600d609e4c672080410d60ab27209730700d60be3044005d60ce4720bd60d8c720c01d196830301938cb2db63087202730800017203938cb2db6308720473090001b4e4b27205730a00730b730c95ed947207720a937207730dd80cd60eb27201730e00d60fdb6308a7d610e4c6a70511d611720bd612720cd613b47210730fb17210d6148c721202d615b2a5731000d616dc640be4c6720e04640283020e73117312e4e3010ed617b2db63087215731300d6188cb2720f73140002d6197cb4e4b272167315007316731796830401938cb2db6308720e7318000172039683080193c27208c2a792c1720899c1a7731993b2db63087208731a00b2720f731b0093b27209731c00b27206731d0093e4c672080511721093e4c672080664e4c6a7066493720a9591b27210731e009d9cb2e4c672040511731f007cb4e4b27205732000732173227323720d7324edafdb0c0e7213d9011a049593721a720d93b27213721a00721490b27213721a00721491b17213720d91db6903db6503feb272107325009683040193cbc27215b4e4b272167326007327732892c172157329938c721701732a928c72170295927218721972187219d802d60ee4c6a70511d60fe4c6720805119594720e720fd809d610b2a4732b00d611e4c6b2a4732c00050ed612adb4db0c0e7211732d9db17211732ed90112047cb472119c7212732f9c9a721273307331d613b072127332d90113599a8c7213018c721302d614e4c6a70664d615e4c67210050ed616dc640a7214027215e4e3010ed617e67216d618e4e3020e96830801927cb4e4dc640ae4c672040464028cb2db6308721073330001e4e3030e73347335721393c27208c2a792c17208c1a793b2db63087208733600b2db6308a7733700937209720693b2720f733800b2720e733900957217d802d619e47216d61aadb4db0c0e7219733a9db17219733bd9011a047cb472199c721a733c9c9a721a733d733e9683020193b2720f733f009a99b2720e734000b0721a7341d9011b599a8c721b018c721b02721393b4720f7342b1720faddc0c1db4720e7343b1720e01addc0c1d721a017212d9011b59998c721b028c721b01d9011b599a8c721b018c721b029683020193b2720f7344009ab2720e734500721393b4720f7346b1720faddc0c1db4720e7347b1720e017212d90119599a8c7219018c72190293db6401e4c672080664db6401957217e4dc640d72140283013c0e0e8602721572117218e4dc640c72140283013c0e0e86027215721172187348e3893c02010b4858ce0425ed4748d0d3a59f2dbf874166a2caaf734655ac5e3f88a68cdd01012aec95af24812a01775de090411ba70a648fe859013f896ca2a1a95882ce5f904e0310020401110400000000644ec61f485b98eb87153f7c57db4f5ecd75556fddbc403b41acf8441fde8e160900072000d35f8400db49e16a8185956c1fce96819bd407f8597a65120fb6bc02ebbc7f5e00c0843d10230400040204000402040604040500050004000e200137c91882b759ad46a20e39aa4d035ce32525dc76d021ee643e71d09446400f04000e20010b4858ce0425ed4748d0d3a59f2dbf874166a2caaf734655ac5e3f88a68cdd0400040204080400040204040502040604080400040004020402040004020e20c7c537e6c635930ecb4ace95a54926b3ab77698d9f4922f0b1c58ea87156483b0400040204420404040205000502d80ed601db6501fed602b27201730000d603b27201730100d604e4c672030410d605e4c6a70411d606b27205730200d607b27205730300d608b27205730400d609b27205730500d60a9172097306d60be4c6a7050c63d60cb1720bd60db1a5d60ed9010e0c63b0dc0c0f720e01d9011063db630872107307d90110414d0e9a8c7210018c8c72100202d196830701938cb2db6308720273080001730996830301938cb2db63087203730a0001730b937eb27204730c00057206937eb27204730d0005720792db6903db6503fe720895720ad804d60fe4c6a7050c63d610b2a5b1720f00d611e4c672100411d612b27205730e009683090192c17210c1a793db63087210db6308a793b27211730f00720693b27211731000720793b27211731100997209731293b272117313009a7208721293b27211731400721293e4c67210050c63720f93c27210c2a7efaea5d9010f63aedb6308720fd901114d0e938c7211018cb2db6308a773150001afdc0c1d720b01b4a5731699720c7317d9010f3c636393c48c720f01c48c720f0293720d9a9a720c95720a731873199593cbc2b2a599720d731a00b4e4b2dc640be4c6720204640283010e731be4e3000e731c00731d731e731f732093da720e01a49ada720e01a595720a73217322e3893c0100b44a84993674c57c4fc23c6c1bb221470463e4e711b2260ffd8ed01f1aab420102110504020090ea9db2f261000c6301000008cd02e4cb952261186ec0fd2dc4c2baa8dbfd9c8f6012c5efa9f702f9450a58fe221ee3893c01012aec95af24812a01775de090411ba70a648fe859013f896ca2a1a95882ce5fa08d06000909d9bf168f897d64f00458fc2294adcf89ac0b6e5718cf1199edaa0afc2b2700813096f27f9aedcedda6e766f429c87ecfed43e168c025c4bb2723bb89ff73b400";
        let constant_bytes = base16::decode(constant_bytes_str).unwrap();
        let c_res = Constant::sigma_parse_bytes(&constant_bytes);
        //dbg!(&c_res);
        assert!(c_res.is_err());
        assert!(matches!(c_res, Err(SigmaParsingError::ValueOutOfBounds(_))));
    }
}
