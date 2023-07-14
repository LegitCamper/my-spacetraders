FROM debian:buster-slim

EXPOSE 80

COPY my-spacetraders .

CMD ["./my-spacetraders"]