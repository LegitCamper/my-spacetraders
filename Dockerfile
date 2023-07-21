FROM debian:buster-slim

COPY target/release/automation spacetraders.bin

CMD ["./my-spacetraders.bin"]