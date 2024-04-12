use crate::eval::Env;
use ergotree_ir::mir::global_vars::GlobalVars;
use ergotree_ir::mir::value::Value;
use ergotree_ir::reference::Ref;
use ergotree_ir::serialization::SigmaSerializable;

use super::EvalContext;
use super::EvalError;
use super::Evaluable;

impl Evaluable for GlobalVars {
    fn eval<'ctx>(
        &self,
        _env: &mut Env,
        ectx: &EvalContext<'ctx>,
    ) -> Result<Value<'ctx>, EvalError> {
        match self {
            GlobalVars::Height => Ok((ectx.ctx.height as i32).into()),
            GlobalVars::SelfBox => Ok(Value::CBox(Ref::from(ectx.ctx.self_box))),
            GlobalVars::Outputs => Ok(ectx
                .ctx
                .outputs
                .iter()
                .map(Ref::Borrowed)
                .collect::<Vec<_>>()
                .into()),
            GlobalVars::Inputs => Ok(ectx
                .ctx
                .inputs
                .iter()
                .map(Ref::Borrowed)
                .collect::<Vec<_>>()
                .into()),
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
            &*eval_out::<Ref<'_, ErgoBox>>(&GlobalVars::SelfBox.into(), &ctx),
            ctx.self_box
        );
    }

    #[test]
    fn eval_outputs() {
        let ctx = force_any_val::<Context>();

        eval_out::<Vec<Ref<'_, ErgoBox>>>(&GlobalVars::Outputs.into(), &ctx)
            .iter()
            .zip(ctx.outputs)
            .for_each(|(a, b)| assert_eq!(&**a, b));
    }

    #[test]
    fn eval_inputs() {
        let ctx = force_any_val::<Context>();

        eval_out::<Vec<Ref<'_, ErgoBox>>>(&GlobalVars::Inputs.into(), &ctx)
            .iter()
            .zip(ctx.inputs)
            .for_each(|(&a, b)| assert_eq!(&*a, b));
    }

    #[test]
    fn eval_group_generator() {
        let ctx = force_any_val::<Context>();
        assert_eq!(
            eval_out::<EcPoint>(&GlobalVars::GroupGenerator.into(), &ctx),
            ergo_chain_types::ec_point::generator()
        );
    }
}
