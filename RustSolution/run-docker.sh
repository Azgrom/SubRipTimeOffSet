#!/bin/bash
set -e

docker run -p 1111:1111 azgrom/subrip_web_api:latest $*
