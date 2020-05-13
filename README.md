# i3-status-info

Some of my i3 status info colored for i3blocks

The main advantages are:
* 1 binary for several elements (disk, cpu, mem...)
* colored output based on custom thresholds
* slow cpu and memory consumption

```
$ i3_status_info --help

Usage:
  i3-status-info [command]

Available Commands:
  battery     Get Battery info
  cpu         Get CPU info
  disk        Get Disk free
  help        Help about any command
  load        Get load info
  mem         Get memory info
  version     Print current version
  wifi        Get Wifi info

Flags:
  -h, --help            help for i3-status-info

Use "i3-status-info [command] --help" for more information about a command.
```

```
$ i3_status_info disk --help
Get Disk free

Usage:
  i3-status-info disk [flags]

Flags:
      --critical int8       Critical threshold ([2-100]) (default 80)
  -h, --help                help for disk
      --volumePath string   Volume path (default "/")
      --warning int8        Warning threshold ([1-99]) (default 60)
```

# i3block.conf example

Here is an example of the config for i3blocks:

```
color=#8bc2ff
separator=true
separator_block_width=20

[icons]
markup=pango

[cpu]
label=
command=~/.config/i3/i3blocks_bin/i3_status_info cpu
interval=3

[memory]
label=
command=~/.config/i3/i3blocks_bin/i3_status_info mem
interval=10

[disk-slash]
label=
command=~/.config/i3/i3blocks_bin/i3_status_info disk --volumePath /
interval=120

[disk-home]
label=
command=~/.config/i3/i3blocks_bin/i3_status_info disk --volumePath /home
interval=120

[wireless]
label=
command=~/.config/i3/i3blocks_bin/i3_status_info wifi --wifiInterface wlp82s0
interval=10

[battery]
command=~/.config/i3/i3blocks_bin/i3_status_info battery
interval=60
```

## Battery

```
 42%
 42%
#FFFC00
```

## Cpu

```
0.0%
0.0%
```

## Disk

```
329G
329G
```

## Load

```
1.02/0.57/0.40
1.02/0.57/0.40
```

## Memory

```
5.3G
5.3G
```

## Wifi

```
100% SSID_NAME
100% SSID_NAME
```

