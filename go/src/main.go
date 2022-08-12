package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"techtonic/src/middleware"

	"github.com/gorilla/mux"
)

type MessageResponse struct {
	Message string `json:"message"`
}

type WordCount map[string]int

func jsonResponse(w http.ResponseWriter, body interface{}) {
	w.Header().Add("Content-Type", "application/json")
	json.NewEncoder(w).Encode(body)
}

func main() {
	router := mux.NewRouter()

	router.HandleFunc("/healthcheck", func(w http.ResponseWriter, r *http.Request) {
		jsonResponse(w, MessageResponse{Message: "healthy"})
	})

	router.HandleFunc("/count", func(w http.ResponseWriter, r *http.Request) {
		jsonResponse(w, WordCount{"Hello": 1, "world": 1})
	})

	// Allow the frontend to call this service
	router.Use(middleware.Cors)

	fmt.Println("Listening on port 8000...")
	log.Fatal(http.ListenAndServe(":8000", router))
}
