#!/bin/sh
echo "Enter proto name:"
read pname

pb-rs --dont_use_cow -o src/proto/$pname.rs proto_files/$pname.proto
pbf proto_files/$pname.proto --browser > ../nightcomes-client/js/$pname.js