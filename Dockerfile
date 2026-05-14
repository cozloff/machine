FROM nvidia/cuda:12.4.1-devel-ubuntu22.04 AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        cmake \
        libcli11-dev \
        libcurl4-openssl-dev \
        libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY CMakeLists.txt ./
COPY src/ ./src/
RUN cmake -S . -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build

FROM nvidia/cuda:12.4.1-runtime-ubuntu22.04
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libcurl4 \
        libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
RUN mkdir -p data
COPY --from=build /src/build/machine /machine
COPY sql/ ./sql/
ENTRYPOINT ["/machine"]
