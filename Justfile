b: build
build:
    cargo r

s: serve
serve: build
    miniserve --route-prefix ~ne321 ./out/ --index index.html

