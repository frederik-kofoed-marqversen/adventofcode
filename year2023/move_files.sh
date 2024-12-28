#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <source_folder> <destination_folder> <number_of_days>"
    exit 1
fi

# Extract parameters
SOURCE_FOLDER=$1
DESTINATION_FOLDER=$2
NUM_DAYS=$3

# Create destination folder if it doesn't exist
if [ ! -d "$DESTINATION_FOLDER" ]; then
    mkdir -p "$DESTINATION_FOLDER"
fi

# Loop through the files and move them
for i in $(seq -f "%02g" 1 "$NUM_DAYS"); do
    DAY="day$i"
    if [ -d "$SOURCE_FOLDER/$DAY" ]; then
        mv "$SOURCE_FOLDER/$DAY/src/main.rs" "$DESTINATION_FOLDER/src/days/$DAY.rs"
        mv "$SOURCE_FOLDER/$DAY/input.data" "$DESTINATION_FOLDER/input/real/$DAY.txt"
        mv "$SOURCE_FOLDER/$DAY/test.data" "$DESTINATION_FOLDER/input/test/$DAY.txt"
        
        echo "Moved $DAY to $DESTINATION_FOLDER"
    else
        echo "Folder $DAY does not exist in $SOURCE_FOLDER"
    fi
done
