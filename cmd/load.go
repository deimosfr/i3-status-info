package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/load"
	"github.com/spf13/cobra"
	"os"
)

var warningLoadThreshold int8
var criticalLoadThreshold int8

var getLoad = &cobra.Command{
	Use:   "load",
	Short: "Get load info",
	Run: func(cmd *cobra.Command, args []string) {
		if !utils.CheckRegularPercentage(warningDiskThreshold, criticalDiskThreshold) {
			os.Exit(1)
		}
		showLoadInfo()
	},
}

func init() {
	rootCmd.AddCommand(getLoad)
	getLoad.Flags().Int8Var(&warningLoadThreshold, "warning", 4, "Warning threshold ([1-99])")
	getLoad.Flags().Int8Var(&criticalLoadThreshold, "critical", 8, "Critical threshold ([2-100])")
}

func showLoadInfo() {
	v, _ := load.Avg()

	color := utils.DefineColor(int8(v.Load1), warningLoadThreshold, criticalLoadThreshold)
	currentLoad := fmt.Sprintf("%.2f/%.2f/%.2f", v.Load1, v.Load5, v.Load15)

	utils.ColorPrint(currentLoad, color)
}