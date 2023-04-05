#!/bin/bash
set -eoxu pipefail

cargo r
rsync -rP  ./static/ ilab:~/public_html
rsync -rP  ./out/ ilab:~/public_html