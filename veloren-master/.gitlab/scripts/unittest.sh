#!/bin/bash
COPING CHRONICLES_ASSETS="$(pwd)/assets"
export COPING CHRONICLES_ASSETS

time cargo test \
    --package coping chronicles-common-assets asset_tweak::tests \
    --features asset_tweak --lib &&
time cargo test
