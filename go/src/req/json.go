package req

import (
	"encoding/json"
	"errors"
	"net/http"
)

func ParseJSON(r *http.Request, value interface{}) error {
	if r.Header.Get("Content-Type") != "application/json" {
		return errors.New("invalid content type for parsing JSON")
	}

	decoder := json.NewDecoder(r.Body)
	decoder.DisallowUnknownFields()
	return decoder.Decode(value)
}
