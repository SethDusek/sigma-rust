use crate::eval::Env;
use ergotree_ir::mir::global_vars::GlobalVars;
use ergotree_ir::mir::value::Value;
use ergotree_ir::serialization::SigmaSerializable;

use super::EvalContext;
use super::EvalError;
use super::Evaluable;

impl Evaluable for GlobalVars {
    fn eval(&self, _env: &mut Env, ectx: &mut EvalContext) -> Result<Value, EvalError> {
        match self {
            GlobalVars::Height => Ok((ectx.ctx.height as i32).into()),
            GlobalVars::SelfBox => Ok(ectx.ctx.self_box.into()),
            GlobalVars::Outputs => Ok(ectx.ctx.outputs.clone().into()),
            GlobalVars::Inputs => Ok(ectx.ctx.inputs.as_vec().clone().into()),
            GlobalVars::MinerPubKey => {
                Ok(ectx.ctx.pre_header.miner_pk.sigma_serialize_bytes()?.into())
            }
            GlobalVars::GroupGenerator => Ok(ergo_chain_types::ec_point::generator().into()),
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::eval::context::Context;
    use crate::eval::tests::eval_out;
    use ergo_chain_types::EcPoint;
    use ergoscript_compiler::compiler::compile_expr;
    use ergoscript_compiler::script_env::ScriptEnv;
    use ergotree_ir::chain::ergo_box::ErgoBox;
    use sigma_test_util::force_any_val;

    use super::*;

    #[test]
    fn eval_height() {
        let ctx = force_any_val::<Context>();
        let expr = compile_expr("HEIGHT", ScriptEnv::new()).unwrap();
        assert_eq!(eval_out::<i32>(&expr, &ctx), ctx.height as i32);
    }

    #[test]
    fn eval_self_box() {
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<&ErgoBox>(&GlobalVars::SelfBox.into(), &ctx),
            ctx.self_box
        );
    }

    #[test]
    fn eval_outputs() {
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<Vec<Arc<ErgoBox>>>(&GlobalVars::Outputs.into(), &ctx),
            ctx.outputs
        );
    }

    #[test]
    fn eval_inputs() {
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<Vec<Arc<ErgoBox>>>(&GlobalVars::Inputs.into(), &ctx),
            *ctx.inputs.as_vec()
        );
    }

    #[test]
    fn eval_group_generator() {
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<EcPoint>(&GlobalVars::GroupGenerator.into(), ctx),
            ergo_chain_types::ec_point::generator()
        );
    }
}
