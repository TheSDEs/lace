//! A fast, extensible probabilistic cross-categorization engine.
//!
//!
//! Lace is a probabilistic cross-categorization engine written in rust with an
//! optional interface to python. Unlike traditional machine learning methods, which
//! learn some function mapping inputs to outputs, Lace learns a joint probability
//! distribution over your dataset, which enables users to...
//!
//! - predict or compute likelihoods of any number of features conditioned on any
//!   number of other features
//! - identify, quantify, and attribute uncertainty from variance in the data,
//!   epistemic uncertainty in the model, and missing features
//! - determine which variables are predictive of which others
//! - determine which records/rows are similar to which others on the whole or
//!   given a specific context
//! - simulate and manipulate synthetic data
//! - work natively with missing data and make inferences about missingness
//!   (missing not-at-random)
//! - work with continuous and categorical data natively, without transformation
//! - identify anomalies, errors, and inconsistencies within the data
//! - edit, backfill, and append data without retraining
//!
//! and more, all in one place, without any explicit model building.
//!
//!
//! # Design
//! Lace learns a probabilistic model of tabular data using cross-categorization.
//! The general steps to operation are
//!
//! * Create a [`prelude::Codebook`] which describes your data. One can be autogenerated but it is best to
//! check it before use.
//! * Create an [`prelude::Engine`] with your data and codebook.
//! * Train the [`prelude::Engine`] and monitor the model likelihood for convergence.
//! * Ask questions via the [`prelude::OracleT`] implementation of [`prelude::Engine`] to explore your data.
//!
//!
//! # Example
//!
//! (For a complete tutorial, see the [Lace Book](https://TODO))
//!
//! The following example uses the pre-trained `animals` example dataset.
//! Each row represents an animal and each column represents a feature of that animal.
//! The feature is present if the cell value is 1 and is absent if the value is 0.
//!
//! First, we create an oracle and import some `enum`s that allow us to call
//! out some of the row and column indices in plain English.
//!
//! ```rust
//! use lace::prelude::*;
//! use lace::examples::Example;
//!
//! let oracle = Example::Animals.oracle().unwrap();
//! ```
//! Let's ask about the statistical dependence between whether something swims
//! and is fast or has flippers. We expect that something swimming is more
//! indicative of whether it swims than whether something is fast, therefore we
//! expect the dependence between swims and flippers to be higher.
//!
//! ```rust
//! # use lace::prelude::*;
//! # use lace::examples::Example;
//! # let oracle = Example::Animals.oracle().unwrap();
//! let depprob_fast = oracle.depprob(
//!     "swims",
//!     "fast",
//! ).unwrap();
//!
//! let depprob_flippers = oracle.depprob(
//!     "swims",
//!     "flippers",
//! ).unwrap();
//!
//! assert!(depprob_flippers > depprob_fast);
//! ```
//!
//! We have the same expectation of mutual information. Mutual information
//! requires more input from the user. We need to know what type of mutual
//! information, and how many samples to take if we need to estimate the mutual
//! information.
//!
//! ```rust
//! # use lace::prelude::*;
//! # use lace::examples::Example;
//! # let oracle = Example::Animals.oracle().unwrap();
//! let mut rng = rand::thread_rng();
//!
//! let mi_fast = oracle.mi(
//!     "swims",
//!     "fast",
//!     1000,
//!     MiType::Iqr,
//! ).unwrap();
//!
//! let mi_flippers = oracle.mi(
//!     "swims",
//!     "flippers",
//!     1000,
//!     MiType::Iqr,
//! ).unwrap();
//!
//! assert!(mi_flippers > mi_fast);
//! ```
//!
//! We can likewise ask about the similarity between rows -- in this case,
//! animals.
//!
//! ```
//! # use lace::prelude::*;
//! # use lace::examples::Example;
//! # let oracle = Example::Animals.oracle().unwrap();
//! let wrt: Option<&[usize]> = None;
//! let rowsim_wolf = oracle.rowsim(
//!     "wolf",
//!     "chihuahua",
//!     wrt,
//!     RowSimilarityVariant::ViewWeighted,
//! ).unwrap();
//!
//! let rowsim_rat = oracle.rowsim(
//!     "rat",
//!     "chihuahua",
//!     wrt,
//!     RowSimilarityVariant::ViewWeighted,
//! ).unwrap();
//!
//! assert!(rowsim_rat > rowsim_wolf);
//! ```
//!
//! And we can add context to similarity.
//!
//! ```
//! # use lace::prelude::*;
//! # use lace::examples::Example;
//! # let oracle = Example::Animals.oracle().unwrap();
//! let context = vec!["swims"];
//! let rowsim_otter = oracle.rowsim(
//!     "beaver",
//!     "otter",
//!     Some(&context),
//!     RowSimilarityVariant::ViewWeighted,
//! ).unwrap();
//!
//! let rowsim_dolphin = oracle.rowsim(
//!     "beaver",
//!     "dolphin",
//!     Some(&context),
//!     RowSimilarityVariant::ViewWeighted,
//! ).unwrap();
//! ```
//!
//! # Feature flags
//! - `formats`: create `Engine`s and `Codebook`s from IPC, CSV, JSON, and
//!   Parquet data files
//! - `bencher`: Build benchmarking utilties
//! - `ctrc_handler`: enables and update handler than captures Ctrl+C
//!
#![warn(unused_extern_crates)]
#![warn(
    clippy::all,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::option_option,
    clippy::implicit_clone,
    clippy::perf
)]

#[cfg(feature = "bencher")]
pub mod bencher;

pub mod config;
pub mod data;
pub mod defaults;

#[cfg(feature = "examples")]
pub mod examples;

mod interface;
pub mod misc;
pub mod optimize;
pub mod prelude;

mod index;

pub use index::*;

pub use config::EngineUpdateConfig;

pub use interface::{
    update_handler, utils, AppendStrategy, BuildEngineError, Builder,
    ConditionalEntropyType, DatalessOracle, Engine, Given, HasData, HasStates,
    ImputeUncertaintyType, InsertDataActions, InsertMode, Metadata,
    MiComponents, MiType, Oracle, OracleT, OverwriteMode,
    PredictUncertaintyType, Row, RowSimilarityVariant, SupportExtension, Value,
    WriteMode,
};

pub mod error {
    pub use super::interface::error::*;
}

use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ParseError<T: Serialize + Debug + Clone + PartialEq + Eq>(T);

impl<T> std::string::ToString for ParseError<T>
where
    T: Serialize + Debug + Clone + PartialEq + Eq,
{
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}

pub use lace_cc::feature::FType;
pub use lace_cc::state::StateDiagnostics;
pub use lace_cc::transition::StateTransition;
pub use lace_data::{Category, Datum, SummaryStatistics};

pub mod consts {
    pub use lace_consts::*;
}

pub mod metadata {
    pub use lace_metadata::*;
}

pub mod codebook {
    pub use lace_codebook::*;
}

pub mod cc {
    pub use lace_cc::*;
}

pub mod stats {
    pub use lace_stats::*;
}
