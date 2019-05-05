#!/bin/bash -e

test_executable_basepath="target/debug/"
test_executable="fifo_server"


function logInfo()
{
  echo -e "\e[93m[I] $1\e[0m"
}


function logError()
{
  echo -e "\e[31m[E] $1\e[0m"
}


function destructor()
{
  pkill -10 ${test_executable} || true
  sleep .1
  pkill -9 ${test_executable} || true
}


function runTest
{
  logInfo "// TODO: Tests.."
}


function main()
{
  trap destructor EXIT

  logInfo "Start the executable under test '${test_executable}'.."
  nohup ${test_executable_basepath}${test_executable} &> server.stdout.log &

  logInfo "Test if the executable is running.."
  pkill -0 ${test_executable}

  runTest

  logInfo "Test if the executable is still running and stop it.."
  pkill -10 ${test_executable}

  logInfo "Done."
}


main
