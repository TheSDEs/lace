<div align=center>
    <img src='assets/lace.svg' width='250px'/>
    <h3>Lace: A Probabilistic Machine Learning tool for Scientific Discovery</h3>
</div>

</br>
</br>

<div align="center">
  <img src="https://readthedocs.org/projects/pylace/badge/?version=latest&style=flat" alt="Read The Docs Badge"/>
  <a href="https://github.com/promised-ai/lace/actions/workflows/rust-build-test.yaml">
    <img src="https://github.com/promised-ai/lace/actions/workflows/rust-build-test.yaml/badge.svg?branch=master&event=push" alt="Rust: Build and Test"/>
  </a>
  <a href="https://github.com/promised-ai/lace/actions/workflows/python-build-test.yaml">
    <img src="https://github.com/promised-ai/lace/actions/workflows/python-build-test.yaml/badge.svg?branch=master&event=push" alt="Python: Build and Test"/>
  </a>
  <a href="https://crates.io/crates/lace">
    <img src="https://img.shields.io/crates/v/lace?logoColor=brightgreen" alt="Crates.io Latest Version"/>
  </a>
  <a href="https://pypi.org/project/pylace">
    <img src="https://img.shields.io/pypi/v/pylace?color=brightgreen" alt="PyPi Latest Version"/>
  </a>
</div>

<div align=center>
     <div>
        <strong>Documentation</strong>: 
        <a href='https://www.lace.dev/'>User guide</a> |
        <a href='https://docs.rs/lace/latest/lace/'>Rust API</a> |
        <a href='https://pylace.readthedocs.io/en/latest/'>Python API</a> |
     </div>
    <div>
        <strong>Installation</strong>: 
        <a href='#installation'>Rust</a> | 
        <a href='#installation'>Python</a> | 
        <a href='#installation'>CLI</a>
     </div>
    <div>
        <strong>Contents</strong>: 
        <a href='#the-problem'>Problem</a> | 
        <a href='#quick-start'>QUICK START</a> | 
        <a href='#license'>License</a>
     </div>
</div>

</br>
</br>

Lace is a probabilistic cross-categorization engine written in rust with an
optional interface to python. Unlike traditional machine learning methods, which
learn some function mapping inputs to outputs, Lace learns a joint probability
distribution over your dataset, which enables users to...

- predict or compute likelihoods of any number of features conditioned on any
  number of other features
- identify, quantify, and attribute uncertainty from variance in the data,
  epistemic uncertainty in the model, and missing features
- determine which variables are predictive of which others
- determine which records/rows are similar to which others on the whole or
  given a specific context
- simulate and manipulate synthetic data
- work natively with missing data and make inferences about missingness
  (missing not-at-random)
- work with continuous and categorical data natively, without transformation
- identify anomalies, errors, and inconsistencies within the data
- edit, backfill, and append data without retraining

and more, all in one place, without any explicit model building.

## The Problem

The goal of lace is to fill some of the massive chasm between standard machine
learning (ML) methods like deep learning and random forests, and statistical
methods like probabilistic programming languages. We wanted to develop a
machine that allows users to experience the joy of discovery, and indeed
optimizes for it.

### Short version

Standard, optimization-based ML methods don't help you learn about your data.
Probabilistic programming tools assume you already have learned a lot about
your data. Neither approach is optimized for what we think is the most
important part of data science: the science part: asking and answering questions.

### Long version

Standard ML methods are easy to use. You can throw data into a random forest
and start predicting with little thought. These methods attempt to learn a
function f(x) -> y that maps inputs x, to outputs y. This ease-of-use comes at
a cost. Generally f(x) does not reflect the reality of the process that
generated your data, but was instead chosen by whoever developed the approach
to be sufficiently expressive to better achieve the optimization goal. This
renders most standard ML completely uninterpretable and unable to yield sensible
uncertainty estimate.

On the other extreme you have probabilistic tools like probabilistic
programming languages (PPLs). A user specifies a model to a PPL in terms of a
hierarchy of probability distributions with parameters θ. The PPL then uses a
procedure (normally Markov Chain Monte Carlo) to learn about the *posterior*
distribution of the parameters given the data p(θ|x). PPLs are all about
interpretability and uncertainty quantification, but they place a number of
pretty steep requirements on the user. PPL users must specify the model
themselves from scratch, meaning they must know (or at least guess) the model.
PPL users must also know how to specify such a model in a way that is
compatible with the underlying inference procedure.

### Who should not use lace

There are a number of use cases for which Lace is not suited

- Non-tabular data such as images and text
- Highly optimizing specific predictions
    + Lace would rather over-generalize than over fit.

## Quick start

Install the CLI and pylace (requires [rust and
cargo](https://www.rust-lang.org/tools/install))

Lace requires rust.

To install the CLI:
```
$ cargo install --locked lace-cli
```

First, use the CLI to fit a model to your data

```console
$ lace run --csv satellites.csv -n 5000 -s 32 --seed 1337 satellites.lace 
```

Then load the model and start asking questions


```python
>>> from lace import Engine
>>> engine = Engine(metadata='satellites.lace')

# Predict the class of orbit given the satellite has a 75-minute
# orbital period and that it has a missing value of geosynchronous
# orbit longitude, and return epistemic uncertainty via Jensen-
# Shannon divergence.
>>> engine.predict(
...     'Class_of_Orbit',
...     given={
...         'Period_minutes': 75.0,
...         'longitude_radians_of_geo': None,
...     },
... )
('LEO', 0.023981898950561048)

# Find the top 10 most surprising (anomalous) orbital periods in
# the table
>>> engine.surprisal('Period_minutes') \
...     .sort('surprisal', reverse=True) \
...     .head(10)
shape: (10, 3)
┌─────────────────────────────────────┬────────────────┬───────────┐
│ index                               ┆ Period_minutes ┆ surprisal │
│ ---                                 ┆ ---            ┆ ---       │
│ str                                 ┆ f64            ┆ f64       │
╞═════════════════════════════════════╪════════════════╪═══════════╡
│ Wind (International Solar-Terres... ┆ 19700.45       ┆ 11.019368 │
│ Integral (INTErnational Gamma-Ra... ┆ 4032.86        ┆ 9.556746  │
│ Chandra X-Ray Observatory (CXO)     ┆ 3808.92        ┆ 9.477986  │
│ Tango (part of Cluster quartet, ... ┆ 3442.0         ┆ 9.346999  │
│ ...                                 ┆ ...            ┆ ...       │
│ Salsa (part of Cluster quartet, ... ┆ 3418.2         ┆ 9.338377  │
│ XMM Newton (High Throughput X-ra... ┆ 2872.15        ┆ 9.13493   │
│ Geotail (Geomagnetic Tail Labora... ┆ 2474.83        ┆ 8.981458  │
│ Interstellar Boundary EXplorer (... ┆ 0.22           ┆ 8.884579  │
└─────────────────────────────────────┴────────────────┴───────────┘
```

And similarly in rust:

```rust,noplayground
use lace::prelude::*;

fn main() {	
    let mut engine = Engine::load("satellites.lace").unrwap();
	
    // Predict the class of orbit given the satellite has a 75-minute
    // orbital period and that it has a missing value of geosynchronous
    // orbit longitude, and return epistemic uncertainty via Jensen-
    // Shannon divergence.
    engine.predict(
        "Class_of_Orbit",
        &Given::Conditions(vec![
            ("Period_minutes", Datum:Continuous(75.0)),
            ("Longitude_of_radians_geo", Datum::Missing),
        ]),
        Some(PredictUncertaintyType::JsDivergence),
        None,
    )
}
```

## License

Lace is licensed under the Business Source License v1.1, which restricts commercial use. See `LICENSE` for full details.

If you would like a license for use in commercial please contact `lace@redpoll.ai`

## Academic use

Lace is free for academic use. Please cite lace according the the `CITATION.cff` metadata.
