use std::convert::TryInto;

use crate::eval::EvalError;

use ergotree_ir::chain::ergo_box::ErgoBox;
use ergotree_ir::mir::constant::TryExtractInto;
use ergotree_ir::mir::value::Value;
use ergotree_ir::reference::Ref;

use super::EvalFn;

pub(crate) static VALUE_EVAL_FN: EvalFn = |_env, _ctx, obj, _args| {
    Ok(Value::Long(
        obj.try_extract_into::<Ref<'_, ErgoBox>>()?.value.as_i64(),
    ))
};

pub(crate) static GET_REG_EVAL_FN: EvalFn = |_env, _ctx, obj, args| {
    let reg_id = args
        .get(0)
        .cloned()
        .ok_or_else(|| EvalError::NotFound("register index is missing".to_string()))?
        .try_extract_into::<i8>()?;
    let reg_id = reg_id.try_into().map_err(|e| {
        EvalError::RegisterIdOutOfBounds(format!(
            "register index {reg_id} is out of bounds: {:?} ",
            e
        ))
    })?;

    Ok(Value::Opt(Box::new(
        obj.try_extract_into::<Ref<'_, ErgoBox>>()?
            .get_register(reg_id)
            .map_err(|e| {
                EvalError::NotFound(format!(
                    "Error getting the register id {reg_id} with error {e:?}"
                ))
            })?
            .map(|c| Value::from(c.v)),
    )))
};

pub(crate) static TOKENS_EVAL_FN: EvalFn = |_env, _ctx, obj, _args| {
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
    use ergotree_ir::mir::expr::Expr;
    use ergotree_ir::mir::global_vars::GlobalVars;
    use ergotree_ir::mir::property_call::PropertyCall;
    use ergotree_ir::types::sbox;
    use sigma_test_util::force_any_val;

    use crate::eval::context::Context;
    use crate::eval::tests::eval_out;

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
}
