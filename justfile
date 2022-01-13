test-images:
	#!/usr/bin/env bash
	set -euxo pipefail

	for image in `ls tests/qoi_test_images/*.qoi`; do
		just test-image `basename $image .qoi`;
	done

test-image image:
	EG_SIMULATOR_CHECK=tests/qoi_test_images/{{image}}.png cargo run --example display tests/qoi_test_images/{{image}}.qoi