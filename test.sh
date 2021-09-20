#!/bin/sh
TEST_FILE=`mktemp "pipeviewer.test.XXXXXXXX"`
TEST_DUPE=`mktemp "pipeviewer.dupe.XXXXXXXX"`
trap "rm -- '${TEST_FILE}' '${TEST_DUPE}'" EXIT

cargo build --release
head -c 100m /dev/urandom > "${TEST_FILE}" 
cargo run --release --quiet < "${TEST_FILE}" > "${TEST_DUPE}"
diff "${TEST_FILE}" "${TEST_DUPE}"
