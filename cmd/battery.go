package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/spf13/cobra"
	"github.com/xellio/tools/acpi"
	"os"
)

var warningBatteryThreshold int8
var criticalBatteryThreshold int8

var getBattery = &cobra.Command{
	Use:   "battery",
	Short: "Get Battery info",
	Run: func(cmd *cobra.Command, args []string) {
		if !utils.CheckReversePercentage(warningBatteryThreshold, criticalBatteryThreshold) {
			os.Exit(1)
		}
		showBatteryInfo()
	},
}

func init() {
	rootCmd.AddCommand(getBattery)
	getBattery.Flags().Int8Var(&warningBatteryThreshold, "warning", 50, "Warning threshold ([2-100])")
	getBattery.Flags().Int8Var(&criticalBatteryThreshold, "critical", 30, "Critical threshold ([1-99])")
}

func showBatteryInfo() {
	color := ""
	batteryNumber := 0

	batInfo, err := acpi.Battery()
	if err != nil {
		fmt.Println(err)
	}

	// Need this because of non idempotency on returned batteries
	if len(batInfo) > 1 {
		for batteryCurrentNumber := range batInfo {
			if batInfo[batteryCurrentNumber].Level != 0 {
				batteryNumber = batteryCurrentNumber
				break
			}
		}
	}

	batteryLevel := batInfo[batteryNumber].Level
	batteryStatus := batInfo[batteryNumber].Status

	icon := iconSelector(batteryLevel, batteryStatus)
	if batInfo[batteryNumber].Status == "Discharging" {
		color = utils.DefineReverseColor(int8(batteryLevel), warningBatteryThreshold, criticalBatteryThreshold)
	}

	printable := fmt.Sprintf("%s %d%%", icon, batteryLevel)
	utils.ColorPrint(printable, color)
}

func iconSelector(currentPercentage int, status string) string {
	var icon string
	if status == "Discharging" {
		if currentPercentage < 20 {
			icon=""
		} else if currentPercentage < 40 {
			icon=""
		} else if currentPercentage < 60 {
			icon=""
		} else if currentPercentage < 85 {
			icon=""
		} else {
			icon=""
		}
	} else {
		if currentPercentage < 20 {
			icon=""
		} else if currentPercentage < 40 {
			icon=""
		} else if currentPercentage < 60 {
			icon=""
		} else if currentPercentage < 85 {
			icon=""
		} else {
			icon=""
		}
	}
	return icon
}