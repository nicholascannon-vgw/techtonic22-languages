package res

import (
	"encoding/json"
	"net/http"
)

type ResponseFunc func(w http.ResponseWriter, body interface{})

func JSON(w http.ResponseWriter, body interface{}) {
	w.Header().Add("Content-Type", "application/json")
	json.NewEncoder(w).Encode(body)
}

func Status(w http.ResponseWriter, statusCode int) {
	w.WriteHeader(statusCode)
}
