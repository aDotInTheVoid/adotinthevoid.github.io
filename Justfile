b: build
build:
    cargo r -- --drafts
    cp -r static/* out

s: serve
serve: build
    miniserve --route-prefix ~ne321 ./out/ --index index.html

