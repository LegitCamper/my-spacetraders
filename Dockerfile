FROM debian:buster-slim

EXPOSE 80

COPY target/release/automation spacetraders.bin

CMD ["./my-spacetraders.bin"]