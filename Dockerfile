FROM alpine:latest AS build
RUN apk add --no-cache g++ musl-dev cmake make sqlite-dev curl-dev ca-certificates
WORKDIR /src
COPY CMakeLists.txt ./
COPY src/ ./src/
RUN cmake -S . -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build

FROM alpine:latest
RUN apk add --no-cache sqlite-libs libcurl libstdc++ ca-certificates
WORKDIR /app
RUN mkdir -p data
COPY --from=build /src/build/machine /machine
COPY sql/ ./sql/
ENTRYPOINT ["/machine"]
