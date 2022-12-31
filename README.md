# starling2

starling2 is a _very_ experimental side-project of its author to develop a
new version of [starling](https://github.com/MattWindsor91/starling-tool), an
automated proof tool for fine-grained concurrent algorithms.


# What is starling/starling2?

Starling (both versions) is a language for expressing proofs over concurrent
algorithms, and a tool for converting them into verification conditions
(which can then be discharged using [Z3](https://github.com/Z3Prover/z3), among
other solvers).  Starling1 was written for the author's PhD thesis, along with
Matt Parkinson, Mike Dodds, and Ben Simner.

This repository contains an effort to rewrite starling1 in Rust, along with a
new input language.


# Planned features

Currently, it can't do much of anything.  Should it get off the ground, slated
improvements include:

- a Pascal-style language that makes the intent of Starling proofs clearer
  (they are more like algorithmic pseudocode than implementations, and the C
  style language of starling1 muddied this somewhat);
- better tracking of errors, with traceability to the original program source;
- a refinement type system;
- (hopefully someday) code extraction;
- being Rewritten in Rust(tm),  starling2 _should_ be easier to maintain,
  install, and use than its F# predecessor.


# Further information

- The project's GitHub wiki has several design documents and other resources available.
- See the examples in the `examples/` directory.