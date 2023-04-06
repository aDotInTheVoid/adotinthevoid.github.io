Due to Reasons (TM), I needed to rewrite my site.

The old site took too long to build, and had a load of nonsense I don't realy want to
put on display.

Instead of using a SSG, I just wrote it myself. It uses pulldown-cmark for markdown,
and tree-sitter for syntax hilighting.

## Perfect Control

This means I can get the exact right behaviour for footnotes [^why]

[^why]: Like having backlinks that work, and errors if a footnote is referenced, but not defined.

I can also get headers to have links.

Also, it's now responsive to a users `prefers-color-scheme` preference.

And the home page is only 8KB!

It might even convince me to write.
