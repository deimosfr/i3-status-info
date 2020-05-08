package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/cpu"
	"github.com/spf13/cobra"
)

var getCpu = &cobra.Command{
	Use:   "cpu",
	Short: "Get CPU info",
	Run: func(cmd *cobra.Command, args []string) {
		showCpuInfo()
	},
}

func init() {
	rootCmd.AddCommand(getCpu)
}

func showCpuInfo() {
	v, _ := cpu.Percent(0, false)
	printable := fmt.Sprintf("%.1f%%", v[0])
	utils.ColorPrint(printable, "")
}