#!/bin/sh
pb-rs --dont_use_cow -o src/proto/proto-all.rs proto_files/proto-all.proto
pbf proto_files/proto-all.proto --browser > ../nightcomes-client/js/proto-all.js