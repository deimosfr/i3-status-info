package utils

import "fmt"

func ColorPrint(message string, color string) {
	if color == "" {
		fmt.Println(message)
		fmt.Println(message)
	}
}