#!/usr/bin/env bash
cargo fix --allow-dirty && cargo test -- --nocapture
