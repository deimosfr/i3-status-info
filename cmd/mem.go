package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/mem"
	"github.com/spf13/cobra"
	"os"
)

var warningMemThreshold int8
var criticalMemThreshold int8

var getMemory = &cobra.Command{
	Use:   "mem",
	Short: "Get memory info",
	Run: func(cmd *cobra.Command, args []string) {
		if !utils.CheckRegularPercentage(warningMemThreshold, criticalMemThreshold) {
			os.Exit(1)
		}
		showMemoryInfo()
	},
}

func init() {
	rootCmd.AddCommand(getMemory)
	getMemory.Flags().Int8Var(&warningMemThreshold, "warning", 60, "Warning threshold ([1-99]%)")
	getMemory.Flags().Int8Var(&criticalMemThreshold, "critical", 80, "Critical threshold ([2-100]%)")
}

func showMemoryInfo() {
	v, _ := mem.VirtualMemory()

	color := utils.DefineColor(int8(v.UsedPercent), warningMemThreshold, criticalCpuThreshold)
	printable := fmt.Sprintf("%.1fG", float64(v.Used) / 1024 / 1024 / 1024)

	utils.ColorPrint(printable, color)
}