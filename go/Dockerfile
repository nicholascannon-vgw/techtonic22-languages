FROM golang:1.16-alpine AS build
WORKDIR /usr/src/app

ENV GOOS=linux
ENV CGO_ENABLED=0

COPY go.mod .
COPY go.sum .
RUN go mod download

COPY src/ src/
RUN go build ./src/main.go

FROM scratch AS runtime
COPY --from=build /usr/src/app/main .
CMD [ "/main" ]
