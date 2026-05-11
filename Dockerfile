FROM alpine:latest AS build
RUN apk add --no-cache g++ musl-dev cmake make sqlite-dev curl-dev ca-certificates
WORKDIR /src
COPY main.cpp pubchem.cpp pubchem.h ./
RUN g++ -O2 -o machine main.cpp pubchem.cpp -lsqlite3 -lcurl

FROM alpine:latest
RUN apk add --no-cache sqlite-libs libcurl libstdc++ ca-certificates
WORKDIR /app
RUN mkdir -p data
COPY --from=build /src/machine /machine
ENTRYPOINT ["/machine"]
