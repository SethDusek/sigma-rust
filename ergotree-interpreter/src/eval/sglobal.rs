use alloc::{string::ToString, sync::Arc};

use crate::eval::EvalError;

use ergotree_ir::mir::value::{CollKind, NativeColl, Value};

use super::EvalFn;
use crate::eval::Vec;
use ergo_chain_types::ec_point::generator;
use ergotree_ir::bigint256::BigInt256;
use ergotree_ir::types::stype::SType;

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
        _ => {
            return Err(EvalError::UnexpectedValue(format!(
                "fromBigEndianBytes: expected first argument to be byte array, got {:?}",
                bytes_val
            )))
        }
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
            let value = bytes
                .iter()
                .fold(0i16, |acc, &x| (acc << 8) | (x as u8 as i16));
            Ok(Value::Short(value))
        }
        SType::SInt => {
            if bytes.len() != 4 {
                return Err(EvalError::UnexpectedValue(
                    "To deserialize Int with fromBigEndianBytes, exactly four bytes should be provided".to_string(),
                ));
            }
            let value = bytes
                .iter()
                .fold(0i32, |acc, &x| (acc << 8) | (x as u8 as i32));
            Ok(Value::Int(value))
        }
        SType::SLong => {
            if bytes.len() != 8 {
                return Err(EvalError::UnexpectedValue(
                    "To deserialize Long with fromBigEndianBytes, exactly eight bytes should be provided".to_string(),
                ));
            }
            let value = bytes
                .iter()
                .fold(0i64, |acc, &x| (acc << 8) | (x as u8 as i64));
            Ok(Value::Long(value))
        }
        SType::SBigInt => {
            if bytes.len() > 32 {
                return Err(EvalError::UnexpectedValue(
                    "BigInt value doesn't fit into 32 bytes in fromBigEndianBytes".to_string(),
                ));
            }
            let bytes_vec: Vec<u8> = bytes.iter().map(|&x| x as u8).collect();
            Ok(Value::BigInt(
                BigInt256::from_be_slice(&bytes_vec).ok_or_else(|| {
                    EvalError::UnexpectedValue("Failed to convert to BigInt256".to_string())
                })?,
            ))
        }
        _ => Err(EvalError::UnexpectedValue(format!(
            "Unsupported type provided in fromBigEndianBytes: {:?}",
            type_val
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

    use crate::eval::tests::{eval_out, eval_out_wo_ctx};
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

    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn test_bigendian_bytes_roundtrip(
            v_byte in any::<i8>(),
            v_short in any::<i16>(),
            v_int in any::<i32>(),
            v_long in any::<i64>()
        ) {
            {
                let bytes = vec![v_byte];

                let type_args = std::iter::once((STypeVar::t(), SType::SByte)).collect();
                let expr: Expr = MethodCall::with_type_args(
                    Expr::Global,
                    sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
                    vec![bytes.into()],
                    type_args,
                )
                .unwrap()
                .into();
                assert_eq!(eval_out_wo_ctx::<i8>(&expr), v_byte);
            }

            {
                let bytes = vec![(v_short >> 8) as i8, v_short as i8];

                let type_args = std::iter::once((STypeVar::t(), SType::SShort)).collect();
                let expr: Expr = MethodCall::with_type_args(
                    Expr::Global,
                    sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
                    vec![bytes.into()],
                    type_args,
                )
                .unwrap()
                .into();
                assert_eq!(eval_out_wo_ctx::<i16>(&expr), v_short);
            }

            {
                let bytes = vec![
                    (v_int >> 24) as i8,
                    (v_int >> 16) as i8,
                    (v_int >> 8) as i8,
                    v_int as i8
                ];

                let type_args = std::iter::once((STypeVar::t(), SType::SInt)).collect();
                let expr: Expr = MethodCall::with_type_args(
                    Expr::Global,
                    sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
                    vec![bytes.into()],
                    type_args,
                )
                .unwrap()
                .into();
                assert_eq!(eval_out_wo_ctx::<i32>(&expr), v_int);
            }

            {
                let bytes = vec![
                    (v_long >> 56) as i8,
                    (v_long >> 48) as i8,
                    (v_long >> 40) as i8,
                    (v_long >> 32) as i8,
                    (v_long >> 24) as i8,
                    (v_long >> 16) as i8,
                    (v_long >> 8) as i8,
                    v_long as i8
                ];

                let type_args = std::iter::once((STypeVar::t(), SType::SLong)).collect();
                let expr: Expr = MethodCall::with_type_args(
                    Expr::Global,
                    sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
                    vec![bytes.clone().into()],
                    type_args,
                )
                .unwrap()
                .into();
                assert_eq!(eval_out_wo_ctx::<i64>(&expr), v_long);

                let original_long = ((bytes[0] as i64) << 56) |
                                  (((bytes[1] as i64) & 0xFF) << 48) |
                                  (((bytes[2] as i64) & 0xFF) << 40) |
                                  (((bytes[3] as i64) & 0xFF) << 32) |
                                  (((bytes[4] as i64) & 0xFF) << 24) |
                                  (((bytes[5] as i64) & 0xFF) << 16) |
                                  (((bytes[6] as i64) & 0xFF) << 8) |
                                  ((bytes[7] as i64) & 0xFF);
                assert_eq!(original_long, v_long);
            }
        }

        #[test]
        fn test_bigint_roundtrip(v_long in any::<i64>()) {
            let bytes = vec![
                (v_long >> 56) as i8,
                (v_long >> 48) as i8,
                (v_long >> 40) as i8,
                (v_long >> 32) as i8,
                (v_long >> 24) as i8,
                (v_long >> 16) as i8,
                (v_long >> 8) as i8,
                v_long as i8
            ];

            let type_args = std::iter::once((STypeVar::t(), SType::SBigInt)).collect();
            let expr: Expr = MethodCall::with_type_args(
                Expr::Global,
                sglobal::FROM_BIGENDIAN_BYTES_METHOD.clone().with_concrete_types(&type_args),
                vec![bytes.into()],
                type_args,
            )
            .unwrap()
            .into();
            assert_eq!(eval_out_wo_ctx::<BigInt256>(&expr), BigInt256::from(v_long));
        }
    }
}
