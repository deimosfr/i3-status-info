package cmd

import (
	"fmt"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/mdlayher/wifi"
	"github.com/spf13/cobra"
	"os"
)

var wifiInterface string
var warningWifiThreshold int8
var criticalWifiThreshold int8

var getWifi = &cobra.Command{
	Use:   "wifi",
	Short: "Get Wifi info",
	Run: func(cmd *cobra.Command, args []string) {
		device, err := cmd.Flags().GetString("wifiInterface")
		if  err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		if !utils.CheckReversePercentage(warningWifiThreshold, warningWifiThreshold) {
			os.Exit(1)
		}
		showWifiInfo(device)
	},
}

func init() {
	rootCmd.AddCommand(getWifi)
	getWifi.Flags().Int8Var(&warningWifiThreshold, "warning", 50, "Warning threshold ([2-100])")
	getWifi.Flags().Int8Var(&criticalWifiThreshold, "critical", 30, "Critical threshold ([1-99])")
	getWifi.Flags().StringVar(&wifiInterface, "wifiInterface", "", "Wifi device interface")
	err := getWifi.MarkFlagRequired("wifiInterface")
	if err != nil {
		println(err)
		os.Exit(1)
	}
}

func showWifiInfo(device string) {
	var printable string
	var validPercentage int8
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
		validPercentage = validateSignalPercentage(signalPercentage)
		printable = fmt.Sprintf("%d%% %s", validPercentage, bss.SSID)
		break
	}

	color := utils.DefineReverseColor(validPercentage, warningWifiThreshold, criticalWifiThreshold)
	utils.ColorPrint(printable, color)
}

func validateSignalPercentage(signal float64) int8 {
	if signal > 100 {
		return 100
	}
	if signal < 0 {
		return 0
	}
	return int8(signal)
}