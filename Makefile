.PHONY: all get cl

all:
	echo ""

get:

cl:
	find . -name *.go | xargs wc -l
