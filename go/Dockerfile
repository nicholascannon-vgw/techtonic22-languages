FROM golang:1.16 AS build
WORKDIR /usr/src/app

ENV GOOS=linux
ENV CGO_ENABLED=0
ENV GOARCH=amd64

COPY go.mod .
COPY go.sum .
RUN go mod download

COPY . .
RUN go build ./src/main.go

FROM scratch

COPY --from=build /usr/src/app/main .

CMD [ "/main" ]
