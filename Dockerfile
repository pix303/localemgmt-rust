# syntax=docker/dockerfile:1
# Create a stage for building the application.

ARG RUST_VERSION=1.75.0
FROM rust:${RUST_VERSION}-slim-bullseye AS build
WORKDIR /app

COPY . .
RUN set -e
RUN cargo build --locked --release
RUN cp -r ./target/release/ /bin/server


################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
FROM debian:bullseye-slim AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin

# Expose the port that the application listens on.
EXPOSE 3030

# What the container should run when it is started.
CMD ["/bin/api"]
