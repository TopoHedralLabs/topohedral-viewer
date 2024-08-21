#!/bin/zsh
export TOPOHEDRAL_VIEWER_HOME=$(pwd)
export RUSTDOCFLAGS="--html-in-header $(pwd)/docs/html/custom-header.html --document-private-items"
export TOPOHEDRAL_VIEWER_DEV=1
export TOPO_LOG=all=debug,update_uniform=trace
export PYTHONSTARTUP=$TOPOHEDRAL_VIEWER_HOME/startup.py