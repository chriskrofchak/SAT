#!/bin/bash

for FILE in ./tests/*; do
    cat $FILE | cargo run;
done
