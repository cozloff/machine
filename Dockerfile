FROM nvidia/cuda:12.4.1-devel-ubuntu22.04 AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        cmake \
        libcli11-dev \
        libcurl4-openssl-dev \
        libsqlite3-dev \
        lsb-release \
        python3-pip \
        wget \
    && wget -q https://packages.apache.org/artifactory/arrow/ubuntu/apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb \
    && apt-get install -y --no-install-recommends ./apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb \
    && apt-get update \
    && apt-get install -y --no-install-recommends libarrow-dev libparquet-dev \
    && pip3 install --no-cache-dir nvidia-libnvcomp-cu12 \
    && rm -f apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb \
    && rm -rf /var/lib/apt/lists/*
ENV LD_LIBRARY_PATH=/usr/local/lib/python3.10/dist-packages/nvidia/libnvcomp/lib64:${LD_LIBRARY_PATH}
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
        lsb-release \
        python3-pip \
        wget \
    && wget -q https://packages.apache.org/artifactory/arrow/ubuntu/apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb \
    && apt-get install -y --no-install-recommends ./apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb \
    && apt-get update \
    && apt-get install -y --no-install-recommends libarrow-dev libparquet-dev \
    && pip3 install --no-cache-dir nvidia-libnvcomp-cu12 \
    && rm -f apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb \
    && rm -rf /var/lib/apt/lists/*
ENV LD_LIBRARY_PATH=/usr/local/lib/python3.10/dist-packages/nvidia/libnvcomp/lib64:${LD_LIBRARY_PATH}
WORKDIR /app
RUN mkdir -p data
COPY --from=build /src/build/machine /machine
COPY sql/ ./sql/
ENTRYPOINT ["/machine"]
