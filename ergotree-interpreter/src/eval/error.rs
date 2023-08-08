use bounded_vec::BoundedVecOutOfBounds;
use ergotree_ir::ergo_tree::ErgoTreeError;
use ergotree_ir::mir::constant::TryExtractFromError;
use ergotree_ir::serialization::SigmaParsingError;
use ergotree_ir::serialization::SigmaSerializationError;
use ergotree_ir::source_span::SourceSpan;
use sigma_ser::ScorexParsingError;
use sigma_ser::ScorexSerializationError;
use thiserror::Error;

use super::cost_accum::CostError;
use super::env::Env;

/// Interpreter errors
#[derive(Error, PartialEq, Eq, Debug, Clone)]
pub enum EvalError {
    /// AVL tree errors
    #[error("AvlTree: {0}")]
    AvlTree(String),
    /// Only boolean or SigmaBoolean is a valid result expr type
    #[error("Only boolean or SigmaBoolean is a valid result expr type")]
    InvalidResultType,
    /// Unexpected Expr encountered during the evaluation
    #[error("Unexpected Expr: {0}")]
    UnexpectedExpr(String),
    /// Error on cost calculation
    #[error("Error on cost calculation: {0:?}")]
    CostError(#[from] CostError),
    /// Unexpected value type
    #[error("Unexpected value type: {0:?}")]
    TryExtractFrom(#[from] TryExtractFromError),
    /// Not found (missing value, argument, etc.)
    #[error("Not found: {0}")]
    NotFound(String),
    /// Register id out of bounds
    #[error("{0}")]
    RegisterIdOutOfBounds(String),
    /// Unexpected value
    #[error("Unexpected value: {0}")]
    UnexpectedValue(String),
    /// Arithmetic exception error
    #[error("Arithmetic exception: {0}")]
    ArithmeticException(String),
    /// Misc error
    #[error("error: {0}")]
    Misc(String),
    /// Sigma serialization error
    #[error("Serialization error: {0}")]
    SigmaSerializationError(#[from] SigmaSerializationError),
    /// Sigma serialization parsing error
    #[error("Serialization parsing error: {0}")]
    SigmaParsingError(#[from] SigmaParsingError),
    /// ErgoTree error
    #[error("ErgoTree error: {0}")]
    ErgoTreeError(#[from] ErgoTreeError),
    /// Not yet implemented
    #[error("evaluation is not yet implemented: {0}")]
    NotImplementedYet(&'static str),
    /// Invalid item quantity for BoundedVec
    #[error("Invalid item quantity for BoundedVec: {0}")]
    BoundedVecError(#[from] BoundedVecOutOfBounds),
    /// Scorex serialization error
    #[error("Serialization error: {0}")]
    ScorexSerializationError(#[from] ScorexSerializationError),
    /// Scorex serialization parsing error
    #[error("Serialization parsing error: {0}")]
    ScorexParsingError(#[from] ScorexParsingError),
    /// Wrapped error with source span and environment
    #[error("eval error: {error}, details: {details:?}")]
    Wrapped {
        /// eval error
        error: Box<EvalError>,
        /// error details
        details: EvalErrorDetails,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EvalErrorDetails {
    /// source span
    source_span: SourceSpan,
    /// environment after evaluation
    env: Env,
    /// source code
    source: Option<String>,
}

impl EvalError {
    /// Wrap eval error with source span
    pub fn wrap(self, source_span: SourceSpan, env: Env) -> Self {
        EvalError::Wrapped {
            error: Box::new(self),
            details: EvalErrorDetails {
                source_span,
                env,
                source: None,
            },
        }
    }

    /// Wrap eval error with source code
    pub fn wrap_with_src(self, source: String) -> Self {
        match self {
            EvalError::Wrapped { error, details } => EvalError::Wrapped {
                error,
                details: EvalErrorDetails {
                    source_span: details.source_span,
                    env: details.env,
                    source: Some(source),
                },
            },
            e => EvalError::Wrapped {
                error: Box::new(e),
                details: EvalErrorDetails {
                    source_span: SourceSpan::empty(),
                    env: Env::empty(),
                    source: Some(source),
                },
            },
        }
    }
}

pub trait ExtResultEvalError<T> {
    fn enrich_err(self, span: SourceSpan, env: Env) -> Result<T, EvalError>;
}

impl<T> ExtResultEvalError<T> for Result<T, EvalError> {
    fn enrich_err(self, span: SourceSpan, env: Env) -> Result<T, EvalError> {
        self.map_err(|e| e.wrap(span, env))
    }
}
