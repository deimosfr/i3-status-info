package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/shirou/gopsutil/disk"
	"github.com/spf13/cobra"
	"os"
)

var volumePath string

var getDisk = &cobra.Command{
	Use:   "disk",
	Short: "Get Disk free",
	Run: func(cmd *cobra.Command, args []string) {
		volume, err := cmd.Flags().GetString("volumePath")
		if  err != nil {
			os.Exit(1)
		}
		showDiskInfo(volume)
	},
}

func init() {
	rootCmd.AddCommand(getDisk)
	getDisk.Flags().StringVar(&volumePath, "volumePath", "/", "Volume path")
}

func showDiskInfo(volumePath string) {
	v, _ := disk.Usage(volumePath)
	printable := fmt.Sprintf("%dG", v.Free / 1024 / 1024 / 1024)
	utils.ColorPrint(printable, "")
}