
It's been another year again, and folks [are][rep1] [writing][rep2] [about][rep3] [what][rep4] [happened][rep5] on their project this year,
and looking forward to next year. I did this [last year](/posts/rustdoc-json-2022/), and it was a super useful exercise for me
personally, and also a good resource of what's changed, without having to trawl through GitHub.


[rep1]: https://bytecodealliance.org/articles/wasmtime-and-cranelift-in-2023
[rep2]: https://slint.dev/blog/2023-in-review
[rep3]: https://www.eff.org/deeplinks/2023/12/fighting-european-threats-encryption-2023-year-review
[rep4]: https://wordpress.com/blog/2023/12/29/2023-year-in-review/
[rep5]: https://blog.thea.codes/my-2023/

## What's Rustdoc JSON

(note: You can skip this section if you're already familiar with it.)

Rustdoc JSON is a unstable feature for rustdoc that allows generating machine readable JSON output describing
the API of a crate (instead of the normal human readable HTML output). If you think of rustdoc like a compiler
from a crate to a description of its API [^rustdoc_as_compiler], this is an alternative target.


[^rustdoc_as_compiler]: I find this is the most helpful way to think about
    rustdoc. It's an alternative backend for rustc, albeit one that forks off
    much earlier in the compilation pipeline, and doesn't produce
    executables/libraries.

This allows tools to reason mechanically about an API in rust. It's the underlying data source for
[roogle](https://roogle.hkmatsumoto.com/),
[cargo-check-external-types](https://github.com/awslabs/cargo-check-external-types),
[pavex](https://github.com/LukeMathWalker/pavex/),
and many more.

## Format Changes

The most user-facing changes this year were changes to the JSON format itself. We made 5 of them:

24. [#106354][106354]: `Variant` was split into `Variant` and `VariantKind`, so the enum discriminant can always be reported. Previously, it could only be reported for a plain enum variant (i.e. one with no fields or braces).
25. [#109410][109410]: Support inherent associated types.
26. [#111427][111427]: Serialize all enums using external tagging. This changed the JSON representation of the data, but it's the same after deserialization into rust values. Doing this is more consistent [^enum_consistent], and allows (de)serializing to non-self-describing formats, such as postcard and bincode[^binary_format]. This can give a [significant performance improvement](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/.28De.29serialization.20speed.20of.20JSON.20docs/near/356983259). Special thanks to [Luca Palmieri](https://www.lpalmieri.com/) for the heroics in landing this, as it required changing every file in the test suite.
27. [#115078][115078]: Rename `Typedef` to `TypeAlias` to be more in line with standard terminology.
28. [#119246][119246]: Add `is_object_safe` field to `Trait`.

[^enum_consistent]: Previously we had an ad-hoc mix of [3 different](https://github.com/rust-lang/rust/issues/93667) ways of serializing enums to JSON.

[^binary_format]: To be clear, their are no plans for rustdoc itself to emit a binary format. However, it allows 3rd party tools to easily convert the JSON to some other format, that they themselves can load.

That makes this year significantly more stable than [last
year](https://alona.page/posts/rustdoc-json-2022/#format-changes), where we
changed the format 13 times.

## rustdoc-types release scare, and ownership transfer.

On the opposite end of the spectrum, the changes to the `rustdoc-types` crate shouldn't be user-visible at all
(touch wood), but are no less important for the long-term health of the project.

The canonical, upstream definition of the rustdoc-json format lives in
[src/rustdoc-json-types/](https://github.com/rust-lang/rust/tree/5a345b325b59370171d9a00c8f575fb177ead767/src/rustdoc-json-types)
of the rust repo. It's used as a dependency by `librustdoc`, and some in-tree
test tooling. However, it can't be directly used by 3rd party code, as it isn't packaged here at all.

To ease adoption, I created the
[`rustdoc-types`](https://crates.io/crates/rustdoc-types) crate. It's a
somewhat automated repackaging of the in-tree `rustdoc-json-types` crate onto
crates.io. Most consumers (AFAICT) of rustdoc-json do so via this crate.
However, despite its importance, it's a personal project. It lives in
[my github account](https://github.com/aDotInTheVoid/rustdoc-types/tree/4be3505d55aa502f7aec3f71383a7dec660bd177/),
and only I have the permissions to publish new versions to crates.io.

This is mostly transparent to users, who [can think of](https://youtu.be/OxQYyg_v3rw?t=991)
`rustdoc-types` as being the same as canonical in-tree representation of the format. However,
this relies on the shell script being run to update and publish the crate. Normally
this isn't a problem, as I tend to review all the changes to rustdoc-json, and am 
[automatically pinged](https://github.com/rust-lang/rust/blob/5a345b325b59370171d9a00c8f575fb177ead767/triagebot.toml#L533) when someone
makes a PR changing it.

However, we risked breaking this illusion with [#115078][115078]. It got into the merge queue the
night before I was about to leave for a week long camping trip. If it'd been merged while I
was in a field with no internet, a new version of `rustdoc-types` wouldn't be
published, and users would be broken. Fortunately for us, the bors queue was quick that
night, so I could publish the new version in morning, just before I left. However this was
a close call, and no-one's eager for it to happen again.

After some [discussion on
Zulip](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/call.20for.20rustdoc-types.20maintainers),
we decided the right thing to do would be to make `rustdoc-types` owned by the
Rustdoc Team (instead of me personally). This means that someone else would be
able to make releases if I can’t for whatever reason. It also provides
succession planning for when I inevitably stop working on rust at some point.

To do this, I’ve written and opened [an
RFC](https://github.com/rust-lang/rfcs/pull/3505), which contains motivation, as
well as the logistical details of the ownership transfer. Once this gets
merged, and the crate gets moved, we shouldn't have to worry about this
happening again.


## Good Chats on Zulip
This section is half so you can see how the sausage gets made with designing
stuff, and half so I can find these links easier in the future.

### The Metaformat, and documenting signatures that rely on nightly features

[Link](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/Rustdoc.20JSON.3A.20Experimental.20rustc.20features).
The core question here is "How much support should rustdoc JSON give to nightly language features", and "How should they be versioned".
The conclusion is that for now, we should version them like regular language features (to keep `format_version` as an unambiguous description of which schema was used to serialize).

The main idea that came out of this was the idea of the "metaformat". Rustdoc JSON
has had a number of different formats over the years, but the way they were
versioned and released has been the same. The idea here is that in addition to
thinking about the format we stabilize, we should also think how we can change the format
after stabilization. The metaformat refers to the design of a series of formats that use
the same mechanism for communicating changes [^current_metaformat].

[^current_metaformat]: That would make the current (and only so far) metaformat be "we have a field called `format_version` as the root of the JSON object, that is incremented on every change".

The conclusion is that the existing metaformat is fine for now, but probably not
suitable for stabilization. 

Changing the metaformat is much more disruptive to the ecosystem than changing the format.
This is because existing tools rely on the metaformat to
[detect if they're using the correct format](https://github.com/awslabs/cargo-check-external-types/blob/4bbf5a80fced7e11fdf855537b4202e225596f67/src/cargo.rs#L88-L101)
and even
[support multiple format versions at once](https://github.com/obi1kenobi/trustfall-rustdoc/blob/a9d7739b848d6bfc05f50ad7c179faec38e18144/src/parser.rs#L43-L70).
Therefore, we can be quite free to change the format, as the ecosystem is used to it,
and has mechanisms to minimize disruption. But a metaformat change would break all these
mechanisms, and would be much more unexpected.
Therefore, we should aim to only change metaformat once,
and to a metaformat that we believe we can stabilize.

### Stabilization Requirements

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
4. Ensure everything's fully documented.
5. Ensure `core` (and popular crates) produces correct output under `jsondoclint` [^jsondoclint]

The first two will require significant design work. The rest is clearer on how to do them, but may well also
throw wrench into the works. I don't want to speculate on a timeline, but I'd not hold my breath on all this getting done in
anything less than ~2 years.

[^2022_stab]: This latter one was in 2022, but it's still relevant today, so cut me some slack.

[^jsondoclint]: A testing tool to find dangling ID's and other invalid output in Rustdoc JSON output.

### cargo-semver-checks/trustfall Test Suite

Rustdoc JSON has a test suite that's built using [JSONPath](https://www.ietf.org/archive/id/draft-goessner-dispatch-jsonpath-00.html)
to write assertions about the contents of the JSON.
[Someone was wondering](https://rust-lang.zulipchat.com/#narrow/stream/266220-t-rustdoc/topic/.E2.9C.94.20Using.20cargo-semver-checks.20in.20rustdoc.20JSON.20tests.3A.20revisited)
if it made sense to complement this with [trustfall](https://github.com/obi1kenobi/trustfall) driven tests, potentially based on the cargo-semver-checks or trustfall-rustdoc-adaptor suites.

We concluded that this wouldn't be a good idea, as it would require all format
changes to also rewrite the trustfall code, which would add a significant
barrier. In addition, the higher-level invariant checks (that can be run on
every document, IE not asserts for specific items presence) can already be
written inside of `jsondoclint`, which is much simpler to understand and modify.

## Conclusion

??? IDK ???. Some stuff happened in 2023. Some of it was Rustdoc JSON related.
Does this post even need a conclusion? Probably.

If you have questions or comments on this post, I'd love to here them.
You can reply to me [on the Fediverse](https://social.treehouse.systems/@aDot/111675587239258225),
open a discussion on [GitHub](https://github.com/aDotInTheVoid/aDotInTheVoid.github.io/discussions),
or send me an [email](mailto:contact@alona.page)

*Thanks to [jyn](https://jyn.dev/) and [Predrag](https://predr.ag/) for their feedback on drafts of this post. Any and all mistakes are solely my own.*

[106354]: https://github.com/rust-lang/rust/pull/106354
[109410]: https://github.com/rust-lang/rust/pull/109410
[111427]: https://github.com/rust-lang/rust/pull/111427
[115078]: https://github.com/rust-lang/rust/pull/115078
[119246]: https://github.com/rust-lang/rust/pull/119246
