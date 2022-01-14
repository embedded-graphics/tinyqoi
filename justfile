target_dir := "target"

test-images:
    #!/usr/bin/env bash
    set -euxo pipefail

    for image in `ls tests/qoi_test_images/*.qoi`; do
        just test-image `basename $image .qoi`;
    done

test-image image:
    EG_SIMULATOR_CHECK=tests/qoi_test_images/{{image}}.png cargo run --example display tests/qoi_test_images/{{image}}.qoi

#----------------------
# README.md generation
# ---------------------

# Generate README.md
generate-readme: (_build-readme)
    cp {{target_dir}}/README.md README.md

# Check if README.md is up to date
@check-readme: (_build-readme)
    diff -q {{target_dir}}/README.md README.md || ( \
        echo -e "\033[1;31mError:\033[0m README.md needs to be regenerated."; \
        echo -e "       Run 'just generate-readme' to regenerate.\n"; \
        exit 1 \
    )

# Builds README.md
_build-readme:
    #!/usr/bin/env bash
    set -e -o pipefail
    mkdir -p {{target_dir}}/readme
    echo "Building README.md"
    cargo readme | sed -E -f filter_readme.sed > {{target_dir}}/README.md
