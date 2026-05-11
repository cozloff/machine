FROM alpine:latest AS build
RUN apk add --no-cache g++ musl-dev cmake make sqlite-dev sqlite-static ca-certificates
WORKDIR /src
COPY main.cpp .
RUN g++ -O2 -static -o machine main.cpp -lsqlite3

FROM scratch
COPY --from=build /src/machine /machine
ENTRYPOINT ["/machine"]
