#!/bin/zsh
export TOPOHEDRAL_VIEWER_HOME=$(pwd)
export RUSTDOCFLAGS="--html-in-header $(pwd)/docs/html/custom-header.html --document-private-items"
export TOPOHEDRAL_VIEWER_DEV=1
export RUST_LOG=topohedral_viewer=info
export PYTHONSTARTUP=$TOPOHEDRAL_VIEWER_HOME/startup.py