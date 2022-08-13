package main

import (
	"fmt"
	"log"
	"net/http"
	"strings"
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
		res.Status(w, 200)
		res.JSON(w, MessageResponse{Message: "healthy"})
	})

	router.HandleFunc("/count", func(w http.ResponseWriter, r *http.Request) {
		payload := WordCountBody{}
		req.ParseJSON(r, &payload)

		wordCount := map[string]int{}

		words := strings.Split(payload.Text, " ")
		for _, word := range words {
			if strings.TrimSpace(word) == "" {
				continue
			}
			if strings.Contains(word, ",") {
				word = strings.ReplaceAll(word, ",", "")
			}
			if strings.Contains(word, "!") {
				word = strings.ReplaceAll(word, "!", "")
			}

			count := wordCount[word]
			wordCount[word] = count + 1
		}

		res.Status(w, 200)
		res.JSON(w, wordCount)
	})

	// Allow the frontend to call this service
	router.Use(middleware.Cors)

	fmt.Println("Listening on port 8000...")
	err := http.ListenAndServe(":8000", router)
	log.Fatal(err)
}
