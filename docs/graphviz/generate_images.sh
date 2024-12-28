#!/bin/bash
./delete_images.sh
find . -name "*.dot" -exec ./generate_image.sh {} \;
