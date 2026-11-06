#!/usr/bin/env bash
# Apify CLI - install with: npm install -g apify-cli
set -euo pipefail
apify call data_foundry/aso-app-rank-tracker --input-file input.json --output-dataset
