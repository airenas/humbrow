#########################################################################################
## docker will invoke this file from ../.. dir in order to access code
#########################################################################################
FROM rust:1.67-buster as builder

ARG BUILD_VERSION=0.1

WORKDIR /src/

COPY ./ /src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
      CARGO_APP_VERSION=$BUILD_VERSION cargo build --release
#########################################################################################
FROM ultrafunk/undetected-chromedriver:3.20-chrome108 as runner

ARG BUILD_VERSION=0.1

LABEL org.opencontainers.image.version=$BUILD_VERSION \
      org.opencontainers.image.authors="airenass@gmail.com" \
      name="humbrow" \
      version=$BUILD_VERSION \
      release=$BUILD_VERSION \
      summary="This image is used to host the humbrow container" \
      description="This image is used to host the humbrow container" 

WORKDIR /app
RUN mkdir -p /app

COPY --from=builder /src/target/release/humbrow /app/

COPY scripts/agent.py /app/scripts/
COPY scripts/cookie.py /app/scripts/

CMD ["./humbrow"]
