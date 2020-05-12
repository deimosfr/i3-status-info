package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/disk"
	"github.com/spf13/cobra"
	"os"
)

var volumePath string
var warningDiskThreshold int8
var criticalDiskThreshold int8

var getDisk = &cobra.Command{
	Use:   "disk",
	Short: "Get Disk free",
	Run: func(cmd *cobra.Command, args []string) {
		volume, err := cmd.Flags().GetString("volumePath")
		if  err != nil {
			os.Exit(1)
		}
		if !utils.CheckRegularPercentage(warningCpuThreshold, criticalCpuThreshold) {
			os.Exit(1)
		}
		showDiskInfo(volume)
	},
}

func init() {
	rootCmd.AddCommand(getDisk)
	getDisk.Flags().StringVar(&volumePath, "volumePath", "/", "Volume path")
	getDisk.Flags().Int8Var(&warningDiskThreshold, "warning", 60, "Warning threshold ([1-99])")
	getDisk.Flags().Int8Var(&criticalDiskThreshold, "critical", 80, "Critical threshold ([2-100])")
}

func showDiskInfo(volumePath string) {
	v, _ := disk.Usage(volumePath)

	color := utils.DefineColor(int8(v.UsedPercent), warningDiskThreshold, criticalDiskThreshold)
	printable := fmt.Sprintf("%dG", v.Free / 1024 / 1024 / 1024)

	utils.ColorPrint(printable, color)
}