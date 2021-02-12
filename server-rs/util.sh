#!/bin/bash

echo How many migrations do you have?
read migrations

for i in {0..migrations}
    do
        diesel migration revert
    done

diesel migration run
