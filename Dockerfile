# 1. This tells docker to use the Rust official image
FROM rust:1.60 as build

RUN update-ca-certificates

# Create appuser
ENV USER=filler
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /filler

COPY ./ .

RUN cargo build --release

###
 # Final Build
###
FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

WORKDIR /filler

ENV PORT=7878
COPY --from=build /filler/target/release/main ./
COPY --from=build /filler/assets ./assets
COPY --from=build /filler/public ./public

USER filler:filler

CMD ["/filler/main"]
