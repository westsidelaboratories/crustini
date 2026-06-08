#!/usr/bin/env bash
set -euo pipefail

APP=${1:-examples/bounce/app.flour}
bun run bake:example "$APP"
