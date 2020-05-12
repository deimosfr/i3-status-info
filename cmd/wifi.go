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
		if !wifiCheckFlags() {
			os.Exit(1)
		}
		showWifiInfo(device)
	},
}

func init() {
	rootCmd.AddCommand(getWifi)
	getWifi.Flags().StringVar(&wifiInterface, "wifiInterface", "", "Wifi device interface")
	getWifi.Flags().Int8Var(&warningWifiThreshold, "warning", 50, "Warning threshold ([2-100])")
	getWifi.Flags().Int8Var(&criticalWifiThreshold, "critical", 30, "Critical threshold ([1-99])")
}

func wifiCheckFlags() bool {
	if warningWifiThreshold > 100 || warningWifiThreshold < 2 {
		fmt.Println("Warning threshold should be set between 2 and 100")
		return false
	}
	if criticalWifiThreshold < 1 || criticalWifiThreshold > 99 {
		fmt.Println("Critical threshold should be set between 1 and 99")
		return false
	}
	if warningWifiThreshold < criticalWifiThreshold {
		fmt.Printf("Warning threshold (%d) can't be lower than critical threshold (%d)\n", warningWifiThreshold, criticalWifiThreshold)
		return false
	}
	return true
}

func showWifiInfo(device string) {
	var printable string
	var validPercentage int8
	color := ""
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

	if validPercentage < criticalWifiThreshold {
		color="#FF0000"
	} else if validPercentage < warningWifiThreshold {
		color="#FFFC00"
	}

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