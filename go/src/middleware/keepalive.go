package middleware

import "net/http"

func DisableKeepAlive(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Connection", "close")
		next.ServeHTTP(w, r)
	})
}
