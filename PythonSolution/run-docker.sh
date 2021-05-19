#!/bin/bash
set -e

docker run -p 80:80 azgrom/suboffsetapi:latest $*
