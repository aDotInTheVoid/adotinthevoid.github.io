
It's been another year again, and folks [are][rep1] [writing][rep2] about what happened on their project this year,
and looking forward to next year. I did this [last year](/posts/rustdoc-json-2022/), and it was a super usefull excersise for me
personally, and also a good resourse of what's changed, without having to trawl though GitHub.

## What's Rustdoc JSON

(note: You can skip this section if your already framilar)

Rustdoc JSON is a unstable feature for rustdoc that allows generating machine readable JSON output descibing
the API of a crate (instead of the normal human readable HTML output). If you think of rustdoc like a compiller
from a crate to a description of it's API [^rustdoc_as_compiller], this is an alternative target.


[^rustdoc_as_compiller]: I find this is the most helpful way to think about
    rustdoc. It's an alternative backend for rustc, albeit one that forks off
    much earlier in the compilation pipeline, and doesn't produce
    executables/libraries.

This allows tools to reason mechanicly about an API in rust. It's the underlying data source for
[roogle](https://roogle.hkmatsumoto.com/),
[cargo-check-external-types](https://github.com/awslabs/cargo-check-external-types),
[pavex](https://github.com/LukeMathWalker/pavex/)
, and many more.

## Format Changes

24. [#106354][106354]: `Variant` was split into `Variant` and `VariantKind`, so the enum discriminant can always be reported. Previously, it could only be reported for a plain enum variant (i.e. one with no fields or braces).
25. [#109410][109410]: Support inherent associated types.


## `rustdoc-types` release scare, and ownership transfer.

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/call.20for.20rustdoc-types.20maintainers/near/409769736
https://github.com/aDotInTheVoid/rustdoc-types/issues/25
https://github.com/rust-lang/rfcs/pull/3505


[rep1]: https://bytecodealliance.org/articles/wasmtime-and-cranelift-in-2023
[rep2]: https://slint.dev/blog/2023-in-review

[106354]: https://github.com/rust-lang/rust/pull/106354



## Good Chat's on Zulip

This section is half so you can see how the saugsage get's made with designing stuff, and half so I can find
these links easier in the future.

### The Metaformat, and documenting signatures that relly on nightly features

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Rustdoc.20JSON.3A.20Experimental.20rustc.20features

### Stablilization

https://rust-lang.zulipchat.com/#narrow/stream/266220-rustdoc/topic/Rustdoc.20JSON.3A.20Stabilization.20criteria

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Long.20Term.20Rustdoc.20JSON.20Stability/near/386100109


### CSC Test Suite

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/.E2.9C.94.20Using.20cargo-semver-checks.20in.20rustdoc.20JSON.20tests.3A.20revisited

### Thinking about resigning

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Resignation.20Musings/near/382779775

### Sealed Traits

https://social.treehouse.systems/@predrag@hachyderm.io/111638365150320222

### The Metaformat, and documenting signatures that relly on nightly features

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Rustdoc.20JSON.3A.20Experimental.20rustc.20features

### Stablilization

https://rust-lang.zulipchat.com/#narrow/stream/266220-rustdoc/topic/Rustdoc.20JSON.3A.20Stabilization.20criteria

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Long.20Term.20Rustdoc.20JSON.20Stability/near/386100109


### CSC Test Suite

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/.E2.9C.94.20Using.20cargo-semver-checks.20in.20rustdoc.20JSON.20tests.3A.20revisited

### Thinking about resigning

https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Resignation.20Musings/near/382779775


## In Conclus


<!-- TODO: More of these, maybe -->