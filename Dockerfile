FROM alpine:latest AS build
RUN apk add --no-cache g++ musl-dev cmake make sqlite-dev curl-dev ca-certificates
WORKDIR /src
COPY src/ ./src/
RUN g++ -O2 -o machine src/main.cpp src/pubchem.cpp -lsqlite3 -lcurl

FROM alpine:latest
RUN apk add --no-cache sqlite-libs libcurl libstdc++ ca-certificates
WORKDIR /app
RUN mkdir -p data
COPY --from=build /src/machine /machine
ENTRYPOINT ["/machine"]
