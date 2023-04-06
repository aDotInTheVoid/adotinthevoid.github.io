b: build
build:
    cargo r
    cp static/* out

s: serve
serve: build
    miniserve --route-prefix ~ne321 ./out/ --index index.html

