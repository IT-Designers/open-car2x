#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

(\
    cd "$SCRIPT_DIR/asn1c" && \
    (find ../asn1c_patches -iname \*.diff -exec sh -c 'cat {} | patch -s -p1'  \;); \
    (test -f configure || autoreconf -iv) && \
    ./configure && \
    make -j$(nproc) \
)
