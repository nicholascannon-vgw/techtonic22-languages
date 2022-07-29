package res

import "net/http"

// Writes the HTTP status to the response. This must be called first!
func Status(w http.ResponseWriter, statusCode int) {
	w.WriteHeader(statusCode)
}
