package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/load"
	"github.com/spf13/cobra"
)

var getLoad = &cobra.Command{
	Use:   "load",
	Short: "Get load info",
	Run: func(cmd *cobra.Command, args []string) {
		showLoadInfo()
	},
}

func init() {
	rootCmd.AddCommand(getLoad)
}

func showLoadInfo() {
	v, _ := load.Avg()
	currentLoad := fmt.Sprintf("%.2f/%.2f/%.2f", v.Load1, v.Load5, v.Load15)
	utils.ColorPrint(currentLoad, "")
}