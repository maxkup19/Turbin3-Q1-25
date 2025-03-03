#!/bin/bash

# Create output directory
mkdir -p diagrams/generated

# Generate PNG files from PlantUML files
for file in diagrams/*.puml; do
    filename=$(basename "$file" .puml)
    plantuml -tpng "$file" -o generated/
    echo "Generated $filename.png"
done
