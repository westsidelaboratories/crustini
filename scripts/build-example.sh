#!/usr/bin/env bash
set -euo pipefail

APP=${1:-examples/hello-world}
bun run bake:example "$APP"
