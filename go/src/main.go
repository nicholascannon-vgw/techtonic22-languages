package main

import (
	"fmt"
	"log"
	"net/http"
	"techtonic/src/middleware"
	"techtonic/src/req"
	"techtonic/src/res"

	"github.com/gorilla/mux"
)

type MessageResponse struct {
	Message string `json:"message"`
}

type WordCountBody struct {
	Text string `json:"text"`
}

func main() {
	router := mux.NewRouter()

	router.HandleFunc("/healthcheck", func(w http.ResponseWriter, r *http.Request) {
		res.Status(w, 200) // Note: status must come before JSON
		res.JSON(w, MessageResponse{Message: "healthy"})
	})

	router.HandleFunc("/count", func(w http.ResponseWriter, r *http.Request) {
		payload := WordCountBody{}
		req.ParseJSON(r, &payload)

		wordCounts := map[string]uint{}

		// TODO: Count words here...

		res.Status(w, 200)
		res.JSON(w, wordCounts)
	})

	router.Use(middleware.Cors) // Allow the frontend to call this service
	router.Use(middleware.DisableKeepAlive)

	fmt.Println("Listening on port 8000...")
	err := http.ListenAndServe(":8000", router)
	log.Fatal(err)
}
