FROM debian:buster-slim

EXPOSE 80

COPY my-spacetraders.bin my-spacetraders.bin

CMD ["./my-spacetraders.bin"]