# Build the app
FROM golang:1.16 AS build
WORKDIR /usr/src/app

# Env vars for compilation
ENV GOOS=linux
ENV CGO_ENABLED=0
ENV GOARCH=amd64

# Download app dependencies
COPY go.mod .
COPY go.sum .
RUN go mod download

# Copy code and compile app to native binary
COPY src/ src/
RUN go build ./src/main.go

# Read more about scratch here: https://hub.docker.com/_/scratch
FROM scratch AS runtime

COPY --from=build /usr/src/app/main .
CMD [ "/main" ]
