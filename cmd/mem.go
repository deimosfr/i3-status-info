package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/mem"
	"github.com/spf13/cobra"
)

var getMemory = &cobra.Command{
	Use:   "mem",
	Short: "Get memory info",
	Run: func(cmd *cobra.Command, args []string) {
		showMemoryInfo()
	},
}

func init() {
	rootCmd.AddCommand(getMemory)
}

func showMemoryInfo() {
	v, _ := mem.VirtualMemory()
	printable := fmt.Sprintf("%.1fG", float64(v.Used) / 1024 / 1024 / 1024)
	utils.ColorPrint(printable, "")
}