package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/mdlayher/wifi"
	"github.com/spf13/cobra"
	"math"
	"os"
)

var wifiInterface string

var getWifi = &cobra.Command{
	Use:   "wifi",
	Short: "Get Wifi info",
	Run: func(cmd *cobra.Command, args []string) {
		device, err := cmd.Flags().GetString("wifiInterface")
		if  err != nil {
			os.Exit(1)
		}
		showWifiInfo(device)
	},
}

func init() {
	rootCmd.AddCommand(getWifi)
	getWifi.Flags().StringVar(&wifiInterface, "wifiInterface", "", "Wifi device interface")
}

func showWifiInfo(device string) {
	var printable string
	WifiCoef := 1.8
	client, _ := wifi.New()
	defer client.Close()

	ifis, _ := client.Interfaces()
	for _, ifi := range ifis {
		if ifi.Name != device {
			continue
		}
		bss, _ := client.BSS(ifi)
		station, _ := client.StationInfo(ifi)
		signalPercentage := WifiCoef * (float64(station[0].Signal) + 100)
		printable = fmt.Sprintf("%d%% %s", int(math.Round(signalPercentage)), bss.SSID)
		break
	}
	utils.ColorPrint(printable, "")
}