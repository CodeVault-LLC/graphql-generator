package file

import (
	"bufio"
	"fmt"
	"os"
)

func Write(file string, content string) error {
	typesFile, err := os.OpenFile(file, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return fmt.Errorf("failed to open types file: %w", err)
	}
	defer typesFile.Close()

	writer := bufio.NewWriter(typesFile)
	defer func() {
		if err := writer.Flush(); err != nil {
			fmt.Printf("Error flushing buffered writer: %v\n", err)
		}
	}()

	if _, err := writer.WriteString(content); err != nil {
		return fmt.Errorf("failed to write types to file: %w", err)
	}

	return nil
}
