# Concurrent Line Counter with TCP Server

## Overview
This application counts the number of lines in a file concurrently and
provides a TCP server for handling client requests.

## Features
- Concurrent file processing for line counting
- TCP server to accept file paths and return line counts
- Comprehensive error handling and performance optimization

## Usage
1. Build and run the application:
```sh
cargo run
```

## Changes
1. Fn counting lines is modified to use optimal amount of parallelism for linf counting only.
2. I attempted to implement parallel file reader that spawns threads, splits file into no overlapping chunks and all are processed concurrently. Unlikely, I couldn't make it work correctly due to arbitrary length of lines which input file may contain.