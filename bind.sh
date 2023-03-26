#!/bin/bash


forge bind -c lib/v3-core/contracts -b crates/bindings/ --crate-name bindings --overwrite --single-file
echo "Generated bindings for v3-core" 