.PHONY: all get cl

all:
	echo ""

get:

clean:
	rm -Rf pkg bin

cl:
	find . -name *.go | xargs wc -l
