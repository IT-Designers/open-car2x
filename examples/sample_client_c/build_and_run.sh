#!/bin/bash

cd `dirname "$0"`
rm -rf cmake-build-release && \
  mkdir cmake-build-release && \
  cd cmake-build-release && \
  cmake -DCMAKE_BUILD_TYPE=Release .. && \
  make -j$(nproc) && \
  (./sample_client --demo-vehicle&) && \
  ./sample_client --demo-fusion