#!/bin/bash -e

test_executable="fifo_server"

echo "Test if the executable is running.."
pgrep ${test_executable}

# TODO: Add tests..

echo "Test if the executable is still running and stop it.."
pkill -10 ${test_executable}

echo "Done."
