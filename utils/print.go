package utils

import "fmt"

func ColorPrint(message string, color string) {
	fmt.Println(message)
	fmt.Println(message)
	if color != "" {
		fmt.Println(color)
	}
}

func DefineColor(currentPercent int8, warningCpuThreshold int8, criticalCpuThreshold int8) string {
	if currentPercent > criticalCpuThreshold {
		return "#FF0000"
	} else if currentPercent > warningCpuThreshold {
		return "#FFFC00"
	}
	return ""
}