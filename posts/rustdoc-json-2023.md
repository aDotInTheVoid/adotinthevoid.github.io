
It's been another year again, and folks [are][rep1] [writing][rep2] about what happened on their project this year,
and looking forward to next year. I did this [last year](/posts/rustdoc-json-2022/), and it was a super usefull excersise for me
personally, and also a good resourse of what's changed, without having to trawl though GitHub.


[rep1]: https://bytecodealliance.org/articles/wasmtime-and-cranelift-in-2023
[rep2]: https://slint.dev/blog/2023-in-review
[rep3]: https://www.eff.org/deeplinks/2023/12/fighting-european-threats-encryption-2023-year-review
[rep4]: https://wordpress.com/blog/2023/12/29/2023-year-in-review/

## What's Rustdoc JSON

(note: You can skip this section if your already framilar with it.)

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
[pavex](https://github.com/LukeMathWalker/pavex/),
and many more.

## Format Changes

The most user-facing changes this year were changes to the JSON format itself. We made 5 of them:

24. [#106354][106354]: `Variant` was split into `Variant` and `VariantKind`, so the enum discriminant can always be reported. Previously, it could only be reported for a plain enum variant (i.e. one with no fields or braces).
25. [#109410][109410]: Support inherent associated types.
26. [#111427][111427]: Serialize all enums using external tagging. This didn't change the information that was present, but how it's represented in JSON. Doing this is more consistant [^enum_consistant], and allows (de)serializing to non-self-describing formats, such as postcard and bincode[^binary_format]. This can give a [signifigant performance improvement](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/.28De.29serialization.20speed.20of.20JSON.20docs/near/356983259). Special thanks to [Luca Palmieri](https://www.lpalmieri.com/) for the heroics in landing this, as it required changing every file in the test suite.
27. [#115078][115078]: Rename `Typedef` to `TypeAlias` to be more in line with stardard terminology.
28. [#119246][119246]: Add `is_object_safe` field to `Trait`.

[^enum_consistant]: Previously we had an ad-hoc mix of [3 different](https://github.com/rust-lang/rust/issues/93667) ways of serializing enums to JSON.

[^binary_format]: To be clear, their are no plans for rustdoc itself to emit a binary format. However, it allows 3rd party tools to easily convert the JSON to some other format, that they themselves can load.

That makes this year signifigatly more stable than [last
year](https://alona.page/posts/rustdoc-json-2022/#format-changes), where we
changed the format 13 times.

## rustdoc-types release scare, and ownership transfer.

On the oposite end of the spectrum, the changes to the `rustdoc-types` crate shouldn't be user-visible at all
(touch wood), but are no less important for the long-term health of the project.

The canonical, upstream definition of the rustdoc-json format lives in
[src/rustdoc-json-types/](https://github.com/rust-lang/rust/tree/5a345b325b59370171d9a00c8f575fb177ead767/src/rustdoc-json-types)
of the rust repo. It's used as a dependency by `librustdoc`, and some in-tree
test tooling. However, it can't be directly used by 3rd party code, as it isn't packaged here at all.

To ease adoption, I created the
[`rustdoc-types`](https://crates.io/crates/rustdoc-types) crate. It's a
somewhat automated repackaging of the in-tree `rustdoc-json-types` crate onto
crates.io. Most consumers (AFAIKT) of rustdoc-json do so via this crate.
However, despite it's importance, it's a personal project. It lives in
[my github account](https://github.com/aDotInTheVoid/rustdoc-types/tree/4be3505d55aa502f7aec3f71383a7dec660bd177/),
and only I have permission's to publish new versions crates.io.

This mostly tranparent to users, who [can think of](https://youtu.be/OxQYyg_v3rw?t=991)
`rustdoc-types` as being the same as canonical in-tree representation of the format. However,
this relies on the shell script being run to update and publish the crate. Normally
this isn't a problem, as I tend to review all the changes to rustdoc-json, and am 
[automaticly pinged](https://github.com/rust-lang/rust/blob/5a345b325b59370171d9a00c8f575fb177ead767/triagebot.toml#L533) when someone
makes a PR changing it.

However, we risked breaking this illusion with [#115078][115078]. It got into the merge queue the
night before I was about to leave for a week long camping trip. If it'd been merged while I
was in a field with no internet, a new version of `rustdoc-types` wouldn't be
published, and users would be broken. Fortunatly for us, the bors queue was quick that
night, so I could publish the new version in morning, just before I left. However this was
a close call, and no-one's eager for it to happen again.

After some [discussion on
Zulip](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/call.20for.20rustdoc-types.20maintainers),
we decided the right think to do would be to make `rustdoc-types` owned by the
Rustdoc Team (instead of me personaly). This means that someone else would be
able to make relases if I can't for whatever reason. It also provides succession
planing for when I inevitably stop working on rust at some point.

To do this, I've written an oppened [an
RFC](https://github.com/rust-lang/rfcs/pull/3505), which contains motivation, as
well as the logistical details of the ownership transfer. Once this get's
merged, and the crate get's moved, we shouldn't have to worry about this
happening again.


## Good Chats on Zulip

This section is half so you can see how the saugsage get's made with designing
stuff, and half so I can find these links easier in the future.

### The Metaformat, and documenting signatures that relly on nightly features

[Link](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Rustdoc.20JSON.3A.20Experimental.20rustc.20features).
The core question here is "How much support should rustdoc JSON give to nightly language features", and "How should they be versioned".
The conclusion is that for now, we should version them like regular language features (to keep )

The main idea that came out of this was the idea of the "metaformat". Rustdoc JSON
has had a number of different formats over the years, but they way they were
versioned and realased has been the same. The idea here is that in addition to
thinking about the format we stabilize, we should also think about the format of
how we change the format after stabilization (hence the term metaformat).

The conclusion is that the existing format is fine for now, but probably not
suitable for stabilization. However, changing the metaformat is much more
expensive from to ecosystem than changing the format (within the same
metaformat) as existing tools relly on the metaformat to detect if they'll
understand the format. Therefor, we should aim to only change metaformat once,
and to a metaformat that we believe we can stabilize.

### Stablilization Requirements

We also talked a 
[couple](https://rust-lang.zulipchat.com/#narrow/stream/266220-rustdoc/topic/Rustdoc.20JSON.3A.20Stabilization.20criteria)
of
[times](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Long.20Term.20Rustdoc.20JSON.20Stability/near/386100109)
[^2022_stab]
about the path to stabilization. The core blockers are

1. Long term metaformat, that allows adding new language concepts (that don't exist yet) without breaking users.
    - Also involved in this: How to deal with nightly only language items.
2. Reliable cross-crate ID lookup. (See the first part of [this issue](https://github.com/rust-lang/rust/issues/106697)) for details.
3. Move `rustdoc-types` into T-Rustdoc ownership.
4. Ensure everything's fully documentated.
5. Ensure `core` (and poplular crates) produces correct output under `jsondoclint` [^jsondoclint]

The first two will require signifigant design work. The rest is clearer on how to do them, but may also
throw a fork in the road. I don't want to speculate on a timeline, but I'd not hold my breath on all this getting done in
anything less than ~2 years.

[^2022_stab]: This latter one was in 2022, but it's still relevant today, so cut me some slack.

[^jsondoclint]: A testing tool to find dangling ID's and other invalid output in Rustdoc JSON output.

### cargo-semver-checks/trustfall Test Suite

Rustdoc JSON has a test suite that's built using [JSONPath](https://www.ietf.org/archive/id/draft-goessner-dispatch-jsonpath-00.html)
to write assertion's about the contents of the JSON.
[Someone was woundering](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/.E2.9C.94.20Using.20cargo-semver-checks.20in.20rustdoc.20JSON.20tests.3A.20revisited)
if it made sense to complement this with [trustfall](https://github.com/obi1kenobi/trustfall) driven tests, potentially based on the cargo-semver-checks or trustfall-rustdoc adaptor.

We concluded that this wouldn't be a good idea, as it would require all format
changes to also rewrite the trustfall code, which would add a signifigant
barrier. In addition, the higher-level invariant checks (that can be run on
every document, IE not asserts for specific items presense) can already be
written inside of `jsondoclint`, which is much simpler to understand and modify.

## Conclusion

??? IDK ???. Some stuff happened in 2023. Some of it was Rustdoc JSON related.
Does this post even need a conclussion? Probably.

*Thanks to [jyn](https://jyn.dev/) and [Predrag](https://predr.ag/) for their feedback on drafts of this post. Any and all mistakes are solely my own.*

[106354]: https://github.com/rust-lang/rust/pull/106354
[109410]: https://github.com/rust-lang/rust/pull/109410
[111427]: https://github.com/rust-lang/rust/pull/111427
[115078]: https://github.com/rust-lang/rust/pull/115078
[119246]: https://github.com/rust-lang/rust/pull/119246
