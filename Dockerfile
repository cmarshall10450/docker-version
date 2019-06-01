FROM scratch

ADD target/x86_64-unknown-linux-musl/release/docker-version /opt/

WORKDIR project

ENTRYPOINT ["/opt/docker-version"]