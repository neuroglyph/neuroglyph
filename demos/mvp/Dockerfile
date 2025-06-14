# SPDX-License-Identifier: Apache-2.0
FROM alpine:latest

# Install dependencies
RUN apk add --no-cache \
    git \
    bash \
    tree \
    build-base

# First build gitmind in the container
WORKDIR /build
COPY ../../c/ /build/c/
RUN cd /build/c && make clean && make && cp gitmind /usr/local/bin/

# Create demo user
RUN adduser -D demouser
USER demouser
WORKDIR /home/demouser

# Copy demo setup script and content
COPY --chown=demouser:demouser setup-demo.sh /home/demouser/
COPY --chown=demouser:demouser ../vault /home/demouser/vault-template/

RUN chmod +x /home/demouser/setup-demo.sh

ENTRYPOINT ["/bin/bash"]