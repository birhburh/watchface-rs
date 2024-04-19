#!/bin/bash

# just to not lost
# if installed watchface-js globaly you should delete it, somehow it is not working otherwise
# npm uninstall --verbose -g . && npm run build && npm install --verbose -g .

# Fix:
# ls ~/Downloads/trang_final_en_mm_dd_bobmod.bin ~/Downloads/lamachin08_en_alarm.bin ~/Downloads/haha16.bin ~/Downloads/haha15_en.bin ~/Downloads/haha14.bin | xargs -n1 ./check.sh
# ls ~/Downloads/haha17.bin ~/Downloads/mod_analog_fhb6_blue_ru.bin ~/Downloads/mod_analog_fhb6_yellow_en.bin | xargs -n1 ./check.sh

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <path_to_watchface_bin>"
    exit 1
fi

# filename=$(basename $1)
# watchface_name=${filename%.*}
# if ! [ -f ${watchface_name}_extracted/preview.png ]; then
#     echo "Watchface bin: $1"
#     echo $watchface_name
# fi
# exit

echo "Watchface bin: $1"
filename=$(basename $1)
watchface_name=${filename%.*}
rm -rf ${watchface_name}_extracted && wfjs readBin -m miband5 -i $1
cargo run $1

# code --diff ${watchface_name}_extracted/watchface.json ${watchface_name}_rs_extracted/watchface.json
# code ${watchface_name}_rs_extracted/watchface.json

compare -metric PSNR ${watchface_name}_extracted/preview.png ${watchface_name}_rs_extracted/preview.png ${watchface_name}_rs_extracted/preview_diff.png && true
convert ${watchface_name}_extracted/preview.png ${watchface_name}_rs_extracted/preview_diff.png ${watchface_name}_rs_extracted/preview.png +append ${watchface_name}_rs_extracted/preview_concat.png
code ${watchface_name}_rs_extracted/preview_concat.png

# code ${watchface_name}_rs_extracted/preview.png
