#!/bin/bash -e

test_executable_basepath="target/debug/"
test_executable="fifo_server"

server_port=8080


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
  if pkill -0 ${test_executable}
  then
    pkill -10 ${test_executable} || true
    sleep .1
    pkill -9 ${test_executable} || true
  fi
}


function testCommand()
{
  local command=$1
  local expected_result=$2

  local send_cmd="nc -N localhost ${server_port}"

  echo -e "${command}" | ${send_cmd} | diff - <(echo -ne "${expected_result}")
}


function runTest
{
  logInfo "Running tests.."

  set -x

  # Error case tests
  testCommand "blah"     "Error: Invalid command"
  testCommand "BLAH"     "Error: Invalid command"
  testCommand "PuB"      "Error: Invalid command"
  testCommand "gEt"      "Error: Invalid command"
  testCommand "PuB one"  "Error: Invalid command"
  testCommand "gEt 1"    "Error: Invalid command"

  testCommand "PUB"      "Error: No arguments provided"
  testCommand "GET"      "Error: No arguments provided"
  testCommand "PUB "     "Error: No arguments provided"
  testCommand "PUB   "   "Error: No arguments provided"

  testCommand "GET blah" "Error: Invalid argument"
  testCommand "GET -1"   "Error: Invalid argument"

  # Happy flow tests
  testCommand "GET 0"
  testCommand "GET 5"

  testCommand "PUB one"
  testCommand "GET 1"    "one "

  testCommand "PUB 1"
  testCommand "GET 1"    "1 "

  testCommand "PUB 1 2"
  testCommand "GET 1"    "1 "
  testCommand "GET 1"    "2 "

  testCommand "PUB 1 2"
  testCommand "GET 2"    "1 2 "

  testCommand "PUB 1 2"
  testCommand "GET 5"    "1 2 "

  set +x
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
