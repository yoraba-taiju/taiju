.PHONY: all get run clean cl

all:
	gofmt -w src/
	GOPATH=$(shell pwd):${GOPATH} GOBIN=$(shell pwd)/bin go install -v -gcflags -N ./...

get:
	GOPATH=$(shell pwd):${GOPATH} GOBIN=$(shell pwd)/bin go get github.com/go-gl/glfw

run:
	bin/taiju

clean:
	rm -Rf pkg bin

cl:
	find . -name *.go | xargs wc -l
