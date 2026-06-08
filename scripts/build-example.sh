#!/usr/bin/env bash
set -euo pipefail

APP=${1:-examples/bounce/app.crst}
cargo run -p crustini -- build "$APP"
