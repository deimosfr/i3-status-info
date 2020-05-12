package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/cpu"
	"github.com/spf13/cobra"
	"os"
)

var warningCpuThreshold int8
var criticalCpuThreshold int8

var getCpu = &cobra.Command{
	Use:   "cpu",
	Short: "Get CPU info",
	Run: func(cmd *cobra.Command, args []string) {
		if !utils.CheckRegularPercentage(warningCpuThreshold, criticalCpuThreshold) {
			os.Exit(1)
		}
		showCpuInfo()
	},
}

func init() {
	rootCmd.AddCommand(getCpu)
	getCpu.Flags().Int8Var(&warningCpuThreshold, "warning", 60, "Warning threshold ([1-99])")
	getCpu.Flags().Int8Var(&criticalCpuThreshold, "critical", 80, "Critical threshold ([2-100])")
}

func showCpuInfo() {
	v, _ := cpu.Percent(0, false)

	color := utils.DefineColor(int8(v[0]), warningCpuThreshold, criticalCpuThreshold)
	printable := fmt.Sprintf("%.1f%%", v[0])

	utils.ColorPrint(printable, color)
}