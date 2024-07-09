[![Build Status](https://travis-ci.com/krisvanrens/rust-workshop-accu2019.svg?branch=master)](https://travis-ci.com/krisvanrens/rust-workshop-accu2019)

# ACCU 2019 Rust workshop

ACCU 2019 Rust workshop excercise.


# Assignment description

The assigment was two-fold; first a parser for simple commands had to be created, which would then be used in the second exercise to create a server application that uses the parser.


## The parser

The parser accepts a string as a command with arguments.
This command string is then translated by the parser to a strong-typed return value representing the command and its arguments.

The following commands must be implemented:

* `PUB <arg1:string> [arg2:string] [arg3:string] .. [argN:string]`
* `GET <arg:number(positive)>`

If the command string is empty, invalid or ill-formed, an error value must be returned.


## The server

The server application binds to a socket address and starts listening for clients.
A client can issue commands as described in the section about the parser.
The server holds an internal string-queue with which it implements a FIFO buffer.

The parser translates the client commands to strong-typed values which the server handles as follows:

* On a `PUB` command, the server will place all provided arguments in the back of a string-queue,
* On a `GET` command, the server will echo back the provided argument number of items from the front of the queue. The items will be removed from the queue,
* On an empty, invalid or ill-formed command, the server will report the error back to the client, i.e. a string starting with `ERROR:`.


# // TODO

* Fix handling of line breaks: `\n`, `\r` and `\r\n`.
* Use command-line arguments to select binding port.
* Make `server::handle_get` return the stream as well, to allow for more uniform implementation.
* Build `server::handle_client` around the `std::io::Read` trait, to be able to test and implement separately from the I/O handling.
