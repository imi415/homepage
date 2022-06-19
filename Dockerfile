FROM rust:1 AS backend_builder

COPY . /src

WORKDIR /src

RUN cargo build --release


FROM node:lts-alpine AS frontend_builder

COPY . /src

WORKDIR /src

RUN npm install
RUN npx webpack --mode=production


FROM debian:buster
RUN mkdir /app

WORKDIR /app

COPY --from=backend_builder /src/target/release/homepage /app
COPY --from=frontend_builder /src/dist /app

CMD ["./homepage"]
