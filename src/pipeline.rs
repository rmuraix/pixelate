//! Typed, functional-style composition utilities for filters.
//!
//! Example: chain invert (RGB→RGB) then halftone (RGB→grayscale).
//! ```no_run
//! use pixelate::filters::{Filter, InvertFilter, HalftoneFilter};
//! use pixelate::pipeline::Pipeline;
//! # let img = image::RgbImage::new(1,1);
//! let pipe = Pipeline::new(InvertFilter).then(HalftoneFilter);
//! let out = pipe.apply(&img);
//! ```
use crate::filters::Filter;

/// A pipeline wrapping a filter `F: Filter<I, O>`.
pub struct Pipeline<I, O, F>
where
    F: Filter<I, O>,
{
    f: F,
    _in: std::marker::PhantomData<I>,
    _out: std::marker::PhantomData<O>,
}

impl<I, O, F> Pipeline<I, O, F>
where
    F: Filter<I, O>,
{
    /// Create a new pipeline from a single filter.
    pub fn new(f: F) -> Self {
        Self {
            f,
            _in: Default::default(),
            _out: Default::default(),
        }
    }

    /// Apply the pipeline to the given input, producing an output value.
    pub fn apply(&self, input: &I) -> O {
        self.f.apply(input)
    }

    /// Compose this pipeline with another filter, producing a longer pipeline.
    pub fn then<N, G>(self, g: G) -> Pipeline<I, N, Compose<F, G, I, O, N>>
    where
        G: Filter<O, N>,
    {
        Pipeline::new(Compose {
            a: self.f,
            b: g,
            _i: Default::default(),
            _m: Default::default(),
            _o: Default::default(),
        })
    }
}

/// Two filters composed end-to-end: `O = B(A(I))`.
pub struct Compose<A, B, I, M, O>
where
    A: Filter<I, M>,
    B: Filter<M, O>,
{
    a: A,
    b: B,
    _i: std::marker::PhantomData<I>,
    _m: std::marker::PhantomData<M>,
    _o: std::marker::PhantomData<O>,
}

impl<A, B, I, M, O> Filter<I, O> for Compose<A, B, I, M, O>
where
    A: Filter<I, M>,
    B: Filter<M, O>,
{
    /// Apply the first filter, then feed its output to the second filter.
    fn apply(&self, input: &I) -> O {
        let mid = self.a.apply(input);
        self.b.apply(&mid)
    }
}
