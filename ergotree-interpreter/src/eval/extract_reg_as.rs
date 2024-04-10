use std::convert::TryInto;

use ergotree_ir::chain::ergo_box::ErgoBox;
use ergotree_ir::mir::constant::TryExtractInto;
use ergotree_ir::mir::extract_reg_as::ExtractRegisterAs;
use ergotree_ir::mir::value::Value;

use crate::eval::env::Env;
use crate::eval::EvalContext;
use crate::eval::EvalError;
use crate::eval::Evaluable;

impl Evaluable for ExtractRegisterAs {
    fn eval(&self, env: &mut Env, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        let ir_box = (&self.input.eval(env, ctx)?).try_extract_into::<&ErgoBox>()?;
        let id = self.register_id.try_into().map_err(|e| {
            EvalError::RegisterIdOutOfBounds(format!(
                "register index {} is out of bounds: {:?} ",
                self.register_id, e
            ))
        })?;
        let reg_val_opt = ir_box.get_register(id).map_err(|e| {
            EvalError::NotFound(format!(
                "Error getting the register id {id} with error {e:?}"
            ))
        })?;
        Ok(Value::Opt(Box::new(reg_val_opt.map(|c| Value::from(c.v)))))
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::context::Context;
    use crate::eval::tests::eval_out;
    use ergotree_ir::mir::expr::Expr;
    use ergotree_ir::mir::global_vars::GlobalVars;
    use ergotree_ir::mir::option_get::OptionGet;
    use ergotree_ir::mir::unary_op::OneArgOpTryBuild;
    use ergotree_ir::types::stype::SType;
    use sigma_test_util::force_any_val;

    #[test]
    fn eval_box_get_reg_r0() {
        let get_reg_expr: Expr = ExtractRegisterAs::new(
            GlobalVars::SelfBox.into(),
            0,
            SType::SOption(SType::SLong.into()),
        )
        .unwrap()
        .into();
        let option_get_expr: Expr = OptionGet::try_build(get_reg_expr).unwrap().into();
        let ctx = force_any_val::<Context>();
        let v = eval_out::<i64>(&option_get_expr, &ctx);
        assert_eq!(v, ctx.self_box.value.as_i64());
    }
}
