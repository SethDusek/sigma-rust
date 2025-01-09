use crate::ergo_tree::ErgoTreeVersion;
use crate::serialization::types::TypeCode;

use super::sfunc::SFunc;
use super::smethod::MethodId;
use super::smethod::SMethodDesc;
use super::stype::SType;
use crate::types::smethod::SMethod;
use crate::types::stype_companion::STypeCompanion;
use crate::types::stype_param::STypeVar;
use alloc::vec;
use alloc::vec::Vec;
use lazy_static::lazy_static;

/// SGlobal type code
pub const TYPE_CODE: TypeCode = TypeCode::SGLOBAL;
/// SGlobal type name
pub static TYPE_NAME: &str = "Global";

/// groupGenerator property
pub const GROUP_GENERATOR_METHOD_ID: MethodId = MethodId(1);
/// "xor" predefined function
pub const XOR_METHOD_ID: MethodId = MethodId(2);
/// "fromBigEndianBytes" predefined function
pub const FROM_BIGENDIAN_BYTES_METHOD_ID: MethodId = MethodId(5);
/// serialize function added in v6.0
pub const SERIALIZE_METHOD_ID: MethodId = MethodId(3);
/// Global.powHit function
pub const POW_HIT_METHOD_ID: MethodId = MethodId(8);

lazy_static! {
    /// Global method descriptors
    pub(crate) static ref METHOD_DESC: Vec<&'static SMethodDesc> =
        vec![&GROUP_GENERATOR_METHOD_DESC, &XOR_METHOD_DESC, &SERIALIZE_METHOD_DESC, &FROM_BIGENDIAN_BYTES_METHOD_DESC, &POW_HIT_METHOD_DESC];
}

lazy_static! {
    static ref GROUP_GENERATOR_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: GROUP_GENERATOR_METHOD_ID,
        name: "groupGenerator",
        tpe: SFunc {
            t_dom: vec![SType::SGlobal],
            t_range: SType::SGroupElement.into(),
            tpe_params: vec![],
        },
        explicit_type_args: vec![],
        min_version: ErgoTreeVersion::V0
    };
     /// GLOBAL.GroupGenerator
    pub static ref GROUP_GENERATOR_METHOD: SMethod = SMethod::new(STypeCompanion::Global, GROUP_GENERATOR_METHOD_DESC.clone(),);

}

lazy_static! {
    static ref XOR_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: XOR_METHOD_ID,
        name: "xor",
        tpe: SFunc {
            t_dom: vec![
                SType::SGlobal,
                SType::SColl(SType::SByte.into()),
                SType::SColl(SType::SByte.into()),
            ],
            t_range: SType::SColl(SType::SByte.into()).into(),
            tpe_params: vec![],
        },
        explicit_type_args: vec![],
        min_version: ErgoTreeVersion::V0
    };
     /// GLOBAL.xor
    pub static ref XOR_METHOD: SMethod = SMethod::new(STypeCompanion::Global, XOR_METHOD_DESC.clone(),);

}

lazy_static! {
    static ref FROM_BIGENDIAN_BYTES_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: FROM_BIGENDIAN_BYTES_METHOD_ID,
        name: "fromBigEndianBytes",
        tpe: SFunc {
            t_dom: vec![SType::SGlobal, SType::SColl(SType::SByte.into())],
            t_range: SType::STypeVar(STypeVar::t()).into(),
            tpe_params: vec![],
        },
        explicit_type_args: vec![STypeVar::t()],
        min_version: ErgoTreeVersion::V3
    };
    /// GLOBAL.fromBigEndianBytes
    pub static ref FROM_BIGENDIAN_BYTES_METHOD: SMethod = SMethod::new(STypeCompanion::Global, FROM_BIGENDIAN_BYTES_METHOD_DESC.clone(),);

    static ref SERIALIZE_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: SERIALIZE_METHOD_ID,
        name: "serialize",
        tpe: SFunc {
            t_dom: vec![
                SType::SGlobal,
                STypeVar::t().into()
            ],
            t_range: SType::SColl(SType::SByte.into()).into(),
            tpe_params: vec![],
        },
        explicit_type_args: vec![],
        min_version: ErgoTreeVersion::V3
    };
    /// GLOBAL.serialize
    pub static ref SERIALIZE_METHOD: SMethod = SMethod::new(STypeCompanion::Global, SERIALIZE_METHOD_DESC.clone(),);

    static ref POW_HIT_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: POW_HIT_METHOD_ID,
        name: "powHit",
        tpe: SFunc {
            t_dom: vec![
                SType::SGlobal,
                SType::SInt,
                SType::SColl(SType::SByte.into()),
                SType::SColl(SType::SByte.into()),
                SType::SColl(SType::SByte.into()),
                SType::SInt,
            ],
            t_range: SType::SBoolean.into(),
            tpe_params: vec![],
        },
        explicit_type_args: vec![],
        min_version: ErgoTreeVersion::V3
    };
    /// Global.powHit
    pub static ref POW_HIT_METHOD: SMethod = SMethod::new(STypeCompanion::Global, POW_HIT_METHOD_DESC.clone());
}
