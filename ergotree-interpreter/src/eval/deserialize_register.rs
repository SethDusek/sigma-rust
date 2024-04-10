use std::convert::TryInto;

use ergotree_ir::chain::ergo_box::RegisterId;
use ergotree_ir::mir::constant::TryExtractInto;
use ergotree_ir::mir::deserialize_register::DeserializeRegister;
use ergotree_ir::mir::expr::Expr;
use ergotree_ir::mir::value::Value;
use ergotree_ir::serialization::SigmaSerializable;
use ergotree_ir::types::stype::SType;

use crate::eval::env::Env;
use crate::eval::EvalContext;
use crate::eval::EvalError;
use crate::eval::Evaluable;

impl Evaluable for DeserializeRegister {
    fn eval(&self, env: &mut Env, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        let reg_id: RegisterId = self.reg.try_into().map_err(|e| {
            EvalError::RegisterIdOutOfBounds(format!("register index is out of bounds: {:?} ", e))
        })?;
        match ctx.ctx.self_box.get_register(reg_id) {
            Ok(Some(c)) => {
                if c.tpe != SType::SColl(SType::SByte.into()) {
                    Err(EvalError::UnexpectedExpr(format!(
                        "DeserializeRegister: expected register {} value {} to have type SColl(SByte), got {:?}",
                        reg_id, c, c.tpe
                    )))
                } else {
                    let bytes = c.v.try_extract_into::<Vec<u8>>()?;
                    let expr = Expr::sigma_parse_bytes(bytes.as_slice())?;
                    if expr.tpe() != self.tpe {
                        let pretty_expr = expr.to_string_pretty();
                        Err(EvalError::UnexpectedExpr(format!("DeserializeRegister: expected register {reg_id} deserialized expr {pretty_expr} to have type {:?}, got {:?}", self.tpe, expr.tpe())))
                    } else {
                        expr.eval(env, ctx)
                    }
                }
            }
            Ok(None) => match &self.default {
                Some(default_expr) => eval_default(&self.tpe, default_expr, env, ctx),
                None => Err(EvalError::NotFound(format!(
                    "DeserializeRegister: register {reg_id} is empty"
                ))),
            },
            Err(e) => match &self.default {
                Some(default_expr) => eval_default(&self.tpe, default_expr, env, ctx),
                None => Err(EvalError::NotFound(format!(
                    "DeserializeRegister: failed to get the register id {reg_id} with error: {e:?}"
                ))),
            },
        }
    }
}

fn eval_default<'ctx>(
    deserialize_reg_tpe: &SType,
    default_expr: &Expr,
    env: &mut Env,
    ctx: &'ctx mut EvalContext,
) -> Result<Value<'ctx>, EvalError> {
    if &default_expr.tpe() != deserialize_reg_tpe {
        Err(EvalError::UnexpectedExpr(format!(
            "DeserializeRegister: expected default expr to have type {:?}, got {:?}",
            deserialize_reg_tpe,
            default_expr.tpe()
        )))
    } else {
        default_expr.eval(env, ctx)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(feature = "arbitrary")]
#[cfg(test)]
mod tests {

    use ergotree_ir::chain::ergo_box::ErgoBox;
    use ergotree_ir::chain::ergo_box::NonMandatoryRegisters;
    use ergotree_ir::mir::bin_op::BinOp;
    use ergotree_ir::mir::bin_op::RelationOp;
    use ergotree_ir::mir::constant::Constant;
    use ergotree_ir::mir::expr::Expr;
    use ergotree_ir::mir::global_vars::GlobalVars;
    use ergotree_ir::serialization::SigmaSerializable;
    use ergotree_ir::types::stype::SType;
    use sigma_test_util::force_any_val;

    use crate::eval::context::Context;
    use crate::eval::tests::try_eval_out;

    use super::*;

    fn make_ctx_with_self_box(self_box: ErgoBox) -> Context<'static> {
        let ctx = force_any_val::<Context>();
        Context {
            height: 0u32,
            self_box: Box::leak(Box::new(self_box)),
            ..ctx
        }
    }

    #[test]
    fn eval() {
        // SInt
        let inner_expr: Expr = BinOp {
            kind: RelationOp::NEq.into(),
            left: Box::new(GlobalVars::Height.into()),
            right: Box::new(1i32.into()),
        }
        .into();
        let reg_value: Constant = inner_expr.sigma_serialize_bytes().unwrap().into();
        let b = force_any_val::<ErgoBox>()
            .with_additional_registers(vec![reg_value].try_into().unwrap());
        // expected SBoolean
        let expr: Expr = DeserializeRegister {
            reg: 4,
            tpe: SType::SBoolean,
            default: None,
        }
        .into();
        let ctx = make_ctx_with_self_box(b);
        assert!(try_eval_out::<bool>(&expr, &ctx).unwrap());
    }

    #[test]
    fn eval_reg_is_empty() {
        let b =
            force_any_val::<ErgoBox>().with_additional_registers(NonMandatoryRegisters::empty());
        // no default provided
        let expr: Expr = DeserializeRegister {
            reg: 5,
            tpe: SType::SBoolean,
            default: None,
        }
        .into();
        let ctx = make_ctx_with_self_box(b.clone());
        assert!(try_eval_out::<Value>(&expr, &ctx).is_err());

        // default with wrong type provided
        let expr: Expr = DeserializeRegister {
            reg: 5,
            tpe: SType::SInt,
            default: Some(Box::new(true.into())),
        }
        .into();
        let ctx = make_ctx_with_self_box(b.clone());
        assert!(try_eval_out::<i32>(&expr, &ctx).is_err());

        // default provided
        let expr: Expr = DeserializeRegister {
            reg: 5,
            tpe: SType::SInt,
            default: Some(Box::new(1i32.into())),
        }
        .into();
        let ctx = make_ctx_with_self_box(b);
        assert_eq!(try_eval_out::<i32>(&expr, &ctx).unwrap(), 1i32);
    }

    #[test]
    fn eval_reg_wrong_type() {
        // SInt, expected SColl(SByte)
        let reg_value: Constant = 1i32.into();
        let b = force_any_val::<ErgoBox>()
            .with_additional_registers(vec![reg_value].try_into().unwrap());
        let expr: Expr = DeserializeRegister {
            reg: 4,
            tpe: SType::SBoolean,
            default: None,
        }
        .into();
        let ctx = make_ctx_with_self_box(b);
        assert!(try_eval_out::<Value>(&expr, &ctx).is_err());
    }

    #[test]
    fn evaluated_expr_wrong_type() {
        // SInt
        let inner_expr: Expr = 1i32.into();
        let reg_value: Constant = inner_expr.sigma_serialize_bytes().unwrap().into();
        let b = force_any_val::<ErgoBox>()
            .with_additional_registers(vec![reg_value].try_into().unwrap());
        // expected SBoolean
        let expr: Expr = DeserializeRegister {
            reg: 4,
            tpe: SType::SBoolean,
            default: None,
        }
        .into();
        let ctx = make_ctx_with_self_box(b);
        assert!(try_eval_out::<bool>(&expr, &ctx).is_err());
    }
}
