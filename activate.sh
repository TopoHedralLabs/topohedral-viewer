#!/bin/zsh
export TOPOHEDRAL_VIEWER_HOME=$(pwd)
export RUSTDOCFLAGS="--html-in-header $(pwd)/docs/html/custom-header.html --document-private-items"
export TOPOHEDRAL_VIEWER_DEV=1
export TOPO_LOG=d2_rpc_test=trace
export PYTHONSTARTUP=$TOPOHEDRAL_VIEWER_HOME/startup.py