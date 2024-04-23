##
# Project Title
#
# @file
# @version 0.1

build:
	wasm-pack build --target web

up:
	miniserve . --index "index.html"


# end
