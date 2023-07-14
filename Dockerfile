FROM debian:buster-slim

EXPOSE 80

COPY target/release/interface spacetraders.bin

CMD ["./my-spacetraders.bin"]