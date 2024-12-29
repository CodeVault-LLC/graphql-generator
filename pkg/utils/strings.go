package utils

// Contains checks if a string is in a slice of strings
func Contains(s []string, e string) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}
