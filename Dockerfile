# 1. AŞAMA: Build (Derleme)
FROM rust:alpine AS builder

# Gerekli sistem paketlerini yükle (OpenSSL vs.)
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app

# Sadece bağımlılıkları derlemek için trick
COPY Cargo.toml Cargo.lock ./
# Boş proje oluştur
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Gerçek kodları kopyala
COPY . .
# Önceki build'den kalan artifact'i temizle ve gerçek kodu derle
RUN touch src/main.rs
RUN cargo build --release

# 2. AŞAMA: Runtime (Çalıştırma)
FROM alpine:latest

RUN apk --no-cache add ca-certificates libgcc

WORKDIR /root/
COPY --from=builder /app/target/release/rust-restapi .

COPY .env .

EXPOSE 3000

CMD ["./rust-restapi"]