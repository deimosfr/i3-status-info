package cmd

import "C"
import (
	"fmt"
	"github.com/AmandaCameron/gobar/utils/dbus/upower"
	"github.com/deimosfr/i3-status-info/utils"
	"github.com/spf13/cobra"
	"github.com/xellio/tools/acpi"
	dbus "launchpad.net/~jamesh/go-dbus/trunk"
	"os"
	"strconv"
	"strings"
)

type uPowerDeviceInfo struct {
	WarningThreshold	int8
	CriticalThrshold	int8
	ModelNameMatch		string
	BatteryPercentage	int8
	Icon 				string
}

type uPowerDeviceInfoRaw struct {
	ModelNameMatch		string
	BetteryPercentage	int8
}

var warningBatteryThreshold int8
var criticalBatteryThreshold int8
var checkBatteryThroughAcpi bool
var model1 []string
var model2 []string
var model3 []string

var getBattery = &cobra.Command{
	Use:   "battery",
	Short: "Get Battery info",
	Long: "This command return by default the laptop battery info using ACPI (can use Dbus/uPower if needed)\n" +
		"You can also show up to 3 uPower info, like your wireless keyboard and mouse. Example:\n" +
		"i3-status-info battery --model1=\"MX Keys Wireless Keyboard,f11c,20,10\"",
	Run: func(cmd *cobra.Command, args []string) {
		var allDevices []uPowerDeviceInfo
		var color string
		var lineToPrint string
		var resultToPrint []string

		if !utils.CheckReversePercentage(warningBatteryThreshold, criticalBatteryThreshold) {
			os.Exit(1)
		}

		// Laptop battery with ACPI
		if checkBatteryThroughAcpi {
			lineToPrint, color = showBatteryInfoAcpi()
			resultToPrint = append(resultToPrint, lineToPrint)
		}

		if !checkBatteryThroughAcpi || len(model1) > 0 || len(model2) > 0 || len(model3) > 0 {
			allRawDevices := getUPowerInfo()
			device1, err := prepareModel(model1)
			if err {
				os.Exit(1)
			}
			allDevices = append(allDevices, device1)
			device2, err := prepareModel(model2)
			if err {
				os.Exit(1)
			}
			allDevices = append(allDevices, device2)
			lineToPrint, _ = showUPowerDevicesInfo(allRawDevices, allDevices)
			resultToPrint = append(resultToPrint, lineToPrint)
		}

		lineToPrint = strings.Join(resultToPrint, " ")
		utils.ColorPrint(strings.Trim(lineToPrint, " "), color)
	},
}

func init() {
	rootCmd.AddCommand(getBattery)
	getBattery.Flags().Int8Var(&warningBatteryThreshold, "warning", 50, "Warning laptop battery threshold ([2-100])")
	getBattery.Flags().Int8Var(&criticalBatteryThreshold, "critical", 30, "Critical laptop battery threshold ([1-99])")
	getBattery.Flags().BoolVar(&checkBatteryThroughAcpi, "use-acpi", false, "Check laptop battery through ACPI")

	getBattery.Flags().StringSliceVar(&model1, "model1", model1, "uPower device 1: ModelName, unicode icon, warning, critical")
	getBattery.Flags().StringSliceVar(&model2, "model2", model2, "uPower device 2: ModelName, unicode icon, warning, critical")
	getBattery.Flags().StringSliceVar(&model3, "model3", model3, "uPower device 3: ModelName, unicode icon, warning, critical")
}

func prepareModel(currentModel []string) (uPowerDeviceInfo, bool) {
	modelDevice := uPowerDeviceInfo{
		WarningThreshold: 0,
		CriticalThrshold: 0,
		ModelNameMatch:   "",
		Icon:             "",
	}

	if len(currentModel) != 4 {
		return modelDevice, true
	}

	warningInt, _ := strconv.Atoi(currentModel[2])
	criticalInt, _ := strconv.Atoi(currentModel[3])
	warningThreshold := int8(warningInt)
	criticalThreshold := int8(criticalInt)
	if !utils.CheckReversePercentage(warningThreshold, criticalThreshold) {
		os.Exit(1)
	}

	modelDevice.CriticalThrshold = criticalThreshold
	modelDevice.WarningThreshold = warningThreshold
	modelDevice.ModelNameMatch = currentModel[0]
	modelDevice.Icon = fmt.Sprintf("%s", currentModel[1])

	return modelDevice, false
}

func failMeMaybe(err error) {
	if err != nil {
		panic(err)
	}
}

func getUPowerInfo() []uPowerDeviceInfoRaw {
	var allDevices []uPowerDeviceInfoRaw
	var currentModel interface{}
	var batteryPercentage float64

	sys, err := dbus.Connect(dbus.SystemBus)
	failMeMaybe(err)
	failMeMaybe(sys.Authenticate())

	up := upower.New(sys)
	devs, errd := up.GetDevices()
	failMeMaybe(errd)

	for _, dev := range devs {
		currentModel, err = dev.Get("org.freedesktop.UPower.Device", "Model")
		failMeMaybe(err)
		batteryPercentage, err = dev.Charge()
		failMeMaybe(err)
		allDevices = append(allDevices, uPowerDeviceInfoRaw{
			ModelNameMatch:    fmt.Sprintf("%v", currentModel),
			BetteryPercentage: int8(batteryPercentage),
		})
	}

	return allDevices
}

func showUPowerDevicesInfo(allRawDevices []uPowerDeviceInfoRaw, allDevices []uPowerDeviceInfo) (string, string) {
	var lineToPrint []string
	var colorResult string
	color := ""

	for _, rawDevice := range allRawDevices {
		for _, device := range allDevices {
			if rawDevice.ModelNameMatch == device.ModelNameMatch {
				lineToPrint = append(lineToPrint, fmt.Sprintf("%s %d%%", device.Icon, rawDevice.BetteryPercentage))
				colorResult = utils.DefineReverseColor(rawDevice.BetteryPercentage, device.WarningThreshold, device.CriticalThrshold)
				if utils.DefineReverseColor(rawDevice.BetteryPercentage, device.WarningThreshold, device.CriticalThrshold) != "" {
					color = colorResult
				}
			}
		}
	}

	return strings.Join(lineToPrint, " "), color
}

func showBatteryInfoAcpi() (string, string) {
	color := ""
	batteryNumber := 0

	batInfo, err := acpi.Battery()
	if err != nil {
		fmt.Println(err)
	}

	// Need this because of non idempotency on returned batteries
	if len(batInfo) > 1 {
		for batteryCurrentNumber := range batInfo {
			if batInfo[batteryCurrentNumber].Level != 0 {
				batteryNumber = batteryCurrentNumber
				break
			}
		}
	}

	batteryLevel := batInfo[batteryNumber].Level
	batteryStatus := batInfo[batteryNumber].Status

	icon := iconSelector(batteryLevel, batteryStatus)
	if batInfo[batteryNumber].Status == "Discharging" {
		color = utils.DefineReverseColor(int8(batteryLevel), warningBatteryThreshold, criticalBatteryThreshold)
	}

	printable := fmt.Sprintf("%s %d%%", icon, batteryLevel)
	return printable, color
}

func iconSelector(currentPercentage int, status string) string {
	var icon string
	if status == "Discharging" || status == "Unknown" {
		if currentPercentage < 20 {
			icon=""
		} else if currentPercentage < 40 {
			icon=""
		} else if currentPercentage < 60 {
			icon=""
		} else if currentPercentage < 85 {
			icon=""
		} else {
			icon=""
		}
	} else {
		if currentPercentage < 20 {
			icon=""
		} else if currentPercentage < 40 {
			icon=""
		} else if currentPercentage < 60 {
			icon=""
		} else if currentPercentage < 85 {
			icon=""
		} else {
			icon=""
		}
	}
	return icon
}