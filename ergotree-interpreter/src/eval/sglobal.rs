use std::sync::Arc;

use crate::eval::EvalError;

use ergotree_ir::mir::value::{CollKind, NativeColl, Value};

use ergo_chain_types::ec_point::generator;
use ergotree_ir::bigint256::BigInt256;
use ergotree_ir::types::stype::SType;
use super::EvalFn;

fn helper_xor(x: &[i8], y: &[i8]) -> Arc<[i8]> {
    x.iter().zip(y.iter()).map(|(x1, x2)| *x1 ^ *x2).collect()
}

pub(crate) static GROUP_GENERATOR_EVAL_FN: EvalFn = |_mc, _env, _ctx, obj, _args| {
    if obj != Value::Global {
        return Err(EvalError::UnexpectedValue(format!(
            "sglobal.groupGenerator expected obj to be Value::Global, got {:?}",
            obj
        )));
    }
    Ok(Value::from(generator()))
};

pub(crate) static XOR_EVAL_FN: EvalFn = |_mc, _env, _ctx, obj, args| {
    if obj != Value::Global {
        return Err(EvalError::UnexpectedValue(format!(
            "sglobal.xor expected obj to be Value::Global, got {:?}",
            obj
        )));
    }
    let right_v = args
        .first()
        .cloned()
        .ok_or_else(|| EvalError::NotFound("xor: missing right arg".to_string()))?;
    let left_v = args
        .get(1)
        .cloned()
        .ok_or_else(|| EvalError::NotFound("xor: missing left arg".to_string()))?;

    match (left_v.clone(), right_v.clone()) {
        (
            Value::Coll(CollKind::NativeColl(NativeColl::CollByte(l_byte))),
            Value::Coll(CollKind::NativeColl(NativeColl::CollByte(r_byte))),
        ) => {
            let xor = helper_xor(&l_byte, &r_byte);
            Ok(CollKind::NativeColl(NativeColl::CollByte(xor)).into())
        }
        _ => Err(EvalError::UnexpectedValue(format!(
            "expected Xor input to be byte array, got: {0:?}",
            (left_v, right_v)
        ))),
    }
};

pub(crate) static SGLOBAL_FROM_BIGENDIAN_BYTES_EVAL_FN: EvalFn = |mc, _env, _ctx, obj, args| {
    if obj != Value::Global {
        return Err(EvalError::UnexpectedValue(format!(
            "sglobal.fromBigEndianBytes expected obj to be Value::Global, got {:?}",
            obj
        )));
    }

    let bytes_val = args
        .first()
        .cloned()
        .ok_or_else(|| EvalError::NotFound("fromBigEndianBytes: missing bytes arg".to_string()))?;
    let type_val = mc.tpe().t_range.clone();

    let bytes = match bytes_val {
        Value::Coll(CollKind::NativeColl(NativeColl::CollByte(bytes))) => bytes,
        _ => return Err(EvalError::UnexpectedValue(format!(
            "fromBigEndianBytes: expected first argument to be byte array, got {:?}",
            bytes_val
        ))),
    };

    match *type_val {
        SType::SByte => {
            if bytes.len() != 1 {
                return Err(EvalError::UnexpectedValue(
                    "To deserialize Byte with fromBigEndianBytes, exactly one byte should be provided".to_string(),
                ));
            }
            Ok(Value::Byte(bytes[0]))
        }
        SType::SShort => {
            if bytes.len() != 2 {
                return Err(EvalError::UnexpectedValue(
                    "To deserialize Short with fromBigEndianBytes, exactly two bytes should be provided".to_string(),
                ));
            }
            let b0 = bytes[0] as i16;
            let b1 = bytes[1] as i16;
            Ok(Value::Short(((b0 & 0xFF) << 8 | (b1 & 0xFF)) as i16))
        }
        SType::SInt => {
            if bytes.len() != 4 {
                return Err(EvalError::UnexpectedValue(
                    "To deserialize Int with fromBigEndianBytes, exactly four bytes should be provided".to_string(),
                ));
            }
            let bytes_vec: Vec<u8> = bytes.iter().map(|&x| x as u8).collect();
            let int_bytes: [u8; 4] = bytes_vec.try_into().map_err(|_| EvalError::UnexpectedValue(
                "Invalid byte array length for Int".to_string(),
            ))?;
            Ok(Value::Int(i32::from_be_bytes(int_bytes)))
        }
        SType::SLong => {
            if bytes.len() != 8 {
                return Err(EvalError::UnexpectedValue(
                    "To deserialize Long with fromBigEndianBytes, exactly eight bytes should be provided".to_string(),
                ));
            }
            let bytes_vec: Vec<u8> = bytes.iter().map(|&x| x as u8).collect();
            let long_bytes: [u8; 8] = bytes_vec.try_into().map_err(|_| EvalError::UnexpectedValue(
                "Invalid byte array length for Long".to_string(),
            ))?;
            Ok(Value::Long(i64::from_be_bytes(long_bytes)))
        }
        SType::SBigInt => {
            if bytes.len() > 32 {
                return Err(EvalError::UnexpectedValue(
                    "BigInt value doesn't fit into 32 bytes in fromBigEndianBytes".to_string(),
                ));
            }
            let bytes_vec: Vec<u8> = bytes.iter().map(|&x| x as u8).collect();
            let big_int = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes_vec);
            Ok(Value::BigInt(BigInt256::try_from(big_int).map_err(|e|
                EvalError::UnexpectedValue(format!("Failed to convert to BigInt256: {:?}", e))
            )?))
        }
        SType::SUnit => {
            if !bytes.is_empty() {
                return Err(EvalError::UnexpectedValue(
                    "To deserialize Unit with fromBigEndianBytes, empty byte array should be provided".to_string(),
                ));
            }
            Ok(Value::Unit)
        }
        _ => Err(EvalError::UnexpectedValue(format!(
            "Unsupported type provided in fromBigEndianBytes: {:?}", type_val
        ))),
    }
};

#[allow(clippy::unwrap_used)]
#[cfg(test)]
#[cfg(feature = "arbitrary")]
mod tests {
    use ergo_chain_types::EcPoint;
    use ergotree_ir::bigint256::BigInt256;
    use ergotree_ir::mir::expr::Expr;
    use ergotree_ir::mir::method_call::MethodCall;
    use ergotree_ir::mir::property_call::PropertyCall;

    use crate::eval::tests::eval_out;
    use ergotree_ir::chain::context::Context;
    use ergotree_ir::types::sglobal;
    use ergotree_ir::types::stype::SType;
    use ergotree_ir::types::stype_param::STypeVar;
    use sigma_test_util::force_any_val;

    #[test]
    fn eval_group_generator() {
        let expr: Expr = PropertyCall::new(Expr::Global, sglobal::GROUP_GENERATOR_METHOD.clone())
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<EcPoint>(&expr, &ctx),
            ergo_chain_types::ec_point::generator()
        );
    }

    #[test]
    fn eval_xor() {
        let left = vec![1_i8, 1, 0, 0];
        let right = vec![0_i8, 1, 0, 1];
        let expected_xor = vec![1_i8, 0, 0, 1];

        let expr: Expr = MethodCall::new(
            Expr::Global,
            sglobal::XOR_METHOD.clone(),
            vec![right.into(), left.into()],
        )
        .unwrap()
        .into();
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<Vec<i8>>(&expr, &ctx).as_slice(),
            expected_xor.as_slice()
        );
    }

    #[test]
    fn eval_from_bigendian_bytes() {
        let type_args = std::iter::once((STypeVar::t(), SType::SInt)).collect();

        let bytes = vec![0_i8, 0, 0, 1];
        let expr: Expr = MethodCall::new(
            Expr::Global,
            sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
            vec![bytes.into()],
        )
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        assert_eq!(eval_out::<i32>(&expr, &ctx), 1);
    }

    #[test]
    fn eval_from_bigendian_bytes_short() {
        let type_args = std::iter::once((STypeVar::t(), SType::SShort)).collect();

        let bytes = vec![0_i8, 1];
        let expr: Expr = MethodCall::new(
            Expr::Global,
            sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
            vec![bytes.into()],
        )
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        assert_eq!(eval_out::<i16>(&expr, &ctx), 1);
    }

    #[test]
    fn eval_from_bigendian_bytes_long() {
        let type_args = std::iter::once((STypeVar::t(), SType::SLong)).collect();

        let bytes = vec![0_i8, 0, 0, 0, 0, 0, 0, 1];
        let expr: Expr = MethodCall::new(
            Expr::Global,
            sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
            vec![bytes.into()],
        )
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        assert_eq!(eval_out::<i64>(&expr, &ctx), 1);
    }

    #[test]
    fn eval_from_bigendian_bytes_wrong_answer() {
        let type_args = std::iter::once((STypeVar::t(), SType::SInt)).collect();

        let bytes = vec![0_i8, 0, 0, 1];
        let expr: Expr = MethodCall::new(
            Expr::Global,
            sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
            vec![bytes.into()],
        )
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        assert_ne!(eval_out::<i32>(&expr, &ctx), 2);
    }

    #[test]
    fn eval_from_bigendian_bytes_bigint() {
        let type_args = std::iter::once((STypeVar::t(), SType::SBigInt)).collect();

        let bytes = vec![0_i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let expr: Expr = MethodCall::new(
            Expr::Global,
            sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
            vec![bytes.into()],
        )
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        let expected_bigint = num_bigint::BigInt::from(1);
        assert_eq!(eval_out::<BigInt256>(&expr, &ctx), BigInt256::try_from(expected_bigint).unwrap());
    }

}
