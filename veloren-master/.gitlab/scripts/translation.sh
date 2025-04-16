#!/bin/bash
COPING CHRONICLES_ASSETS="$(pwd)/assets"
export COPING CHRONICLES_ASSETS

time cargo run --bin i18n_csv --features="stat"
