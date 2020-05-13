package utils

import "fmt"

func ColorPrint(message string, color string) {
	fmt.Println(message)
	fmt.Println(message)
	if color != "" {
		fmt.Println(color)
	}
}

func DefineColor(currentPercent int8, warningThreshold int8, criticalThreshold int8) string {
	if currentPercent > criticalThreshold {
		return "#FF0000"
	} else if currentPercent > warningThreshold {
		return "#FFFC00"
	}
	return ""
}

func DefineReverseColor(currentPercent int8, warningThreshold int8, criticalThreshold int8) string {
	if currentPercent < criticalThreshold {
		return "#FF0000"
	} else if currentPercent < warningThreshold {
		return "#FFFC00"
	}
	return ""
}