#!/bin/bash

for filename in ./tests/* do
    cat filename | cargo run
    echo '\n'
done
