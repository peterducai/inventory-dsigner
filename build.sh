#!/bin/bash

cargo build
chmod +x target/debug/inventory-dsigner
./target/debug/inventory-dsigner