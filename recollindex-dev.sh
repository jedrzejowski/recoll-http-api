#!/bin/bash

VOLUME=/home/adam/Pobrane

docker run --rm \
  -v $VOLUME:/input:ro \
  -v "$(pwd)/volume:/output" recoll recollindex "${@}"