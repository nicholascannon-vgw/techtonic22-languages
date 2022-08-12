package main

import (
	"fmt"
	"log"
	"net/http"
	"techtonic/src/middleware"
	"techtonic/src/res"

	"github.com/gorilla/mux"
)

type MessageResponse struct {
	Message string `json:"message"`
}

type WordCount map[string]int

func main() {
	router := mux.NewRouter()

	router.HandleFunc("/healthcheck", func(w http.ResponseWriter, r *http.Request) {
		res.JSON(w, MessageResponse{Message: "healthy"})
		res.Status(w, 200)
	})

	router.HandleFunc("/count", func(w http.ResponseWriter, r *http.Request) {
		res.JSON(w, WordCount{"Hello": 1, "world": 1})
		res.Status(w, 200)
	})

	// Allow the frontend to call this service
	router.Use(middleware.Cors)

	fmt.Println("Listening on port 8000...")
	log.Fatal(http.ListenAndServe(":8000", router))
}
