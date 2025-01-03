//! Context(blockchain) for the interpreter
use core::cell::Cell;

use crate::chain::ergo_box::ErgoBox;
use crate::{chain::context_extension::ContextExtension, ergo_tree::ErgoTreeVersion};
use bounded_vec::BoundedVec;
use ergo_chain_types::{Header, PreHeader};

/// BoundedVec type for Tx inputs, output_candidates and outputs
pub type TxIoVec<T> = BoundedVec<T, 1, { i16::MAX as usize }>;

/// Interpreter's context (blockchain state)
#[derive(Debug, Clone)]
pub struct Context<'ctx> {
    /// Current height
    pub height: u32,
    /// Box that contains the script we're evaluating (from spending transaction inputs)
    pub self_box: &'ctx ErgoBox,
    /// Spending transaction outputs
    pub outputs: &'ctx [ErgoBox],
    /// Spending transaction data inputs
    pub data_inputs: Option<TxIoVec<&'ctx ErgoBox>>,
    /// Spending transaction inputs
    pub inputs: TxIoVec<&'ctx ErgoBox>,
    /// Pre header of current block
    pub pre_header: PreHeader,
    /// Fixed number of last block headers in descending order (first header is the newest one)
    pub headers: [Header; 10],
    /// prover-defined key-value pairs, that may be used inside a script
    pub extension: ContextExtension,
    /// ergo tree version
    pub tree_version: Cell<ErgoTreeVersion>,
}

impl<'ctx> Context<'ctx> {
    /// Return a new Context with given context extension
    pub fn with_extension(self, ext: ContextExtension) -> Self {
        Context {
            extension: ext,
            ..self
        }
    }
    /// Activated script version corresponds to block version - 1
    pub fn activated_script_version(&self) -> ErgoTreeVersion {
        ErgoTreeVersion::from(self.pre_header.version.saturating_sub(1))
    }
    /// Version of ergotree being evaluated under context
    pub fn tree_version(&self) -> ErgoTreeVersion {
        self.tree_version.get()
    }
}

#[cfg(feature = "arbitrary")]
#[allow(clippy::unwrap_used)]
mod arbitrary {

    use super::*;
    use proptest::{collection::vec, option::of, prelude::*};

    impl Arbitrary for Context<'static> {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                0..i32::MAX as u32,
                any::<ErgoBox>(),
                vec(any::<ErgoBox>(), 1..3),
                vec(any::<ErgoBox>(), 1..3),
                of(vec(any::<ErgoBox>(), 1..3)),
                any::<PreHeader>(),
                any::<ContextExtension>(),
                any::<[Header; 10]>(),
            )
                .prop_map(
                    |(
                        height,
                        self_box,
                        outputs,
                        inputs,
                        data_inputs,
                        pre_header,
                        extension,
                        headers,
                    )| {
                        // Leak variables. Since this is only used for testing this is acceptable and avoids introducing a new type (ContextOwned)
                        Self {
                            height,
                            self_box: Box::leak(Box::new(self_box)),
                            outputs: Vec::leak(outputs),
                            data_inputs: data_inputs.map(|v| {
                                v.into_iter()
                                    .map(|i| &*Box::leak(Box::new(i)))
                                    .collect::<Vec<_>>()
                                    .try_into()
                                    .unwrap()
                            }),
                            inputs: inputs
                                .into_iter()
                                .map(|i| &*Box::leak(Box::new(i)))
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap(),
                            pre_header,
                            extension,
                            headers,
                            tree_version: Default::default(),
                        }
                    },
                )
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }
}

#[cfg(test)]
mod tests {}
