b: build
build:
    cargo r -- --drafts
    cp static/* out

s: serve
serve: build
    miniserve --route-prefix ~ne321 ./out/ --index index.html

