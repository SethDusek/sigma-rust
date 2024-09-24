use std::convert::TryInto;

use crate::eval::EvalError;

use ergotree_ir::chain::ergo_box::ErgoBox;
use ergotree_ir::mir::constant::TryExtractInto;
use ergotree_ir::mir::value::Value;
use ergotree_ir::reference::Ref;
use ergotree_ir::types::stype::SType;

use super::EvalFn;

pub(crate) static VALUE_EVAL_FN: EvalFn = |_mc, _env, _ctx, obj, _args| {
    Ok(Value::Long(
        obj.try_extract_into::<Ref<'_, ErgoBox>>()?.value.as_i64(),
    ))
};

pub(crate) static GET_REG_EVAL_FN: EvalFn = |mc, _env, _ctx, obj, args| {
    #[allow(clippy::unwrap_used)]
    let reg_id: i8 = args
        .first()
        .cloned()
        .ok_or_else(|| EvalError::NotFound("register index is missing".to_string()))?
        .try_extract_into::<i32>()?
        .try_into()
        .map_err(|e| {
            EvalError::RegisterIdOutOfBounds(format!("register index is out of bounds: {:?} ", e))
        })?;
    let reg_id = reg_id.try_into().map_err(|e| {
        EvalError::RegisterIdOutOfBounds(format!(
            "register index {reg_id} is out of bounds: {:?} ",
            e
        ))
    })?;

    let reg_val_opt = obj
        .try_extract_into::<Ref<'_, ErgoBox>>()?
        .get_register(reg_id)
        .map_err(|e| {
            EvalError::NotFound(format!(
                "Error getting the register id {reg_id} with error {e:?}"
            ))
        })?;
    // Return type of getReg[T] is always Option[T]
    #[allow(clippy::unreachable)]
    let SType::SOption(expected_type) = &*mc.tpe().t_range
    else {
        unreachable!()
    };
    match reg_val_opt {
        Some(constant) if constant.tpe == **expected_type => {
            Ok(Value::Opt(Box::new(Some(constant.v.into()))))
        }
        Some(constant) => Err(EvalError::UnexpectedValue(format!(
            "Expected register {reg_id} to be of type {}, got {}",
            expected_type, constant.tpe
        ))),
        None => Ok(Value::Opt(Box::new(None))),
    }
};

pub(crate) static TOKENS_EVAL_FN: EvalFn = |_mc, _env, _ctx, obj, _args| {
    let res: Value = obj
        .try_extract_into::<Ref<'_, ErgoBox>>()?
        .tokens_raw()
        .into();
    Ok(res)
};

#[allow(clippy::unwrap_used)]
#[cfg(test)]
#[cfg(feature = "arbitrary")]
mod tests {
    use ergotree_ir::mir::constant::Constant;
    use ergotree_ir::mir::expr::Expr;
    use ergotree_ir::mir::global_vars::GlobalVars;
    use ergotree_ir::mir::method_call::MethodCall;
    use ergotree_ir::mir::property_call::PropertyCall;
    use ergotree_ir::mir::value::Value;
    use ergotree_ir::types::sbox;
    use ergotree_ir::types::stype::SType;
    use ergotree_ir::types::stype_param::STypeVar;
    use sigma_test_util::force_any_val;

    use crate::eval::context::Context;
    use crate::eval::tests::{eval_out, try_eval_out};

    #[test]
    fn eval_box_value() {
        let expr: Expr = PropertyCall::new(GlobalVars::SelfBox.into(), sbox::VALUE_METHOD.clone())
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        assert_eq!(eval_out::<i64>(&expr, &ctx), ctx.self_box.value.as_i64());
    }

    #[test]
    fn eval_box_tokens() {
        let expr: Expr = PropertyCall::new(GlobalVars::SelfBox.into(), sbox::TOKENS_METHOD.clone())
            .unwrap()
            .into();
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<Vec<(Vec<i8>, i64)>>(&expr, &ctx),
            ctx.self_box.tokens_raw()
        );
    }

    // Attempt to extract SigmaProp from register of type SByte
    #[test]
    fn eval_reg_out() {
        let type_args = std::iter::once((STypeVar::t(), SType::SLong)).collect();
        let expr: Expr = MethodCall::with_type_args(
            GlobalVars::SelfBox.into(),
            sbox::GET_REG_METHOD.clone().with_concrete_types(&type_args),
            vec![Constant::from(0i32).into()],
            type_args,
        )
        .unwrap()
        .into();
        let ctx = force_any_val::<Context>();
        try_eval_out::<Value>(&expr, &ctx).unwrap();
    }

    // Attempt to extract SigmaProp from register of type SLong
    #[test]
    fn eval_reg_out_wrong_type() {
        let type_args = std::iter::once((STypeVar::t(), SType::SSigmaProp)).collect();
        let expr: Expr = MethodCall::with_type_args(
            GlobalVars::SelfBox.into(),
            sbox::GET_REG_METHOD.clone().with_concrete_types(&type_args),
            vec![Constant::from(0i32).into()],
            type_args,
        )
        .unwrap()
        .into();
        let ctx = force_any_val::<Context>();
        assert!(try_eval_out::<Value>(&expr, &ctx).is_err());
    }
}
