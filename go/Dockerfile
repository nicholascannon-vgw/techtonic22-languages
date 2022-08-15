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

# Compile app to binary
COPY src/ src/
RUN go build ./src/main.go

FROM scratch AS runtime

COPY --from=build /usr/src/app/main .
CMD [ "/main" ]
