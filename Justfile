b: build
build:
    cargo r --
    cp -r static/* out

s: serve
serve: build
    miniserve --route-prefix ~ne321/s2/ ./out/ --index index.html

deploy-s2: build
    rsync -rP ./out/ ilab:~/public_html/s2