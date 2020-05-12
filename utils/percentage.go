package utils

import (
	"fmt"
)

func CheckRegularPercentage(warningThreshold int8, criticalThreshold int8) bool {
	if warningThreshold > 99 || warningThreshold < 1 {
		fmt.Println("Warning threshold should be set between 1 and 99")
		return false
	}
	if criticalThreshold < 2 || criticalThreshold > 100 {
		fmt.Println("Critical threshold should be set between 2 and 100")
		return false
	}
	if criticalThreshold < warningThreshold {
		fmt.Printf("Warning threshold (%d) can't be greater than critical threshold (%d)\n",
			warningThreshold, criticalThreshold)
		return false
	}
	return true
}