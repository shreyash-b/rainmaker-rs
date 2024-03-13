
# Rust Implementation of ESP Rainmaker

A cross-platform implementation of ESP Rainmaker for ESP32 products and Linux using Rust.



## Build Prerequisites

- Follow the [Prerequisites](https://github.com/esp-rs/esp-idf-template#prerequisites) section in the ``` esp-idf-template``` crate.
- Install Protobuf compiler on your system.


## Get rainmaker-rs

Please clone this repository using the below command:

```bash
  git clone https://github.com/shreyash-b/rainmaker-rs.git
```
    
## Running on ESP devices

1. Erase flash contents:
```bash
espflash erase-flash
```
2. Use [rainmaker cli](https://rainmaker.espressif.com/docs/cli-setup.html) to perform manual claiming on ESP devices
```bash
./rainmaker.py claim /dev/ttyUSB0
```
3. Navigate to rainmaker-rs directory
```bash
cd rainmaker-rs
```
4. Build the project
```bash
cargo build
```
5. Run
```bash
cargo run --target <mcu-target>
```
| MCU | Target     | 
| :-------- | :------- |
| ESP32 | `xtensa-esp32-espidf` | 
| ESP32-S2 | `xtensa-esp32-espidf` |
| ESP32-S3 | `xtensa-esp32-espidf` |
| ESP32-C2 | `riscv32imc-esp-espidf` |
| ESP32-C3 | `riscv32imc-esp-espidf` |
| ESP32-C6 | `riscv32imc-esp-espidf` |

6. Monitor
```bash
espflash monitor
```

## Running on Linux
1. Create directories for storing persistent data
```bash
mkdir -p ~/.config/rmaker/fctry
mkdir -p ~/.config/rmaker/nvs
```
2. Fetch claim data using rainmaker cli
```bash
./rainmaker.py login
./rainmaker.py claim --mac <MAC addr> /dev/null 
```
3. Run
```bash
cargo run --target x86_64-unknown-linux-gnu
OR
cargo run_linux
```

When running for the first time, you'll need to set ```RMAKER_CLAIMDATA_PATH``` environment variable to the folder containing your claimdata(mentioned in before running section)


 


---

### Note

- **When running on ESP32 for first time, on initial boot it will start wifi provisioning and user node mapping.** 
- **After performing wifi provisioning using Rainmaker Android application restart the ESP32 for normal functioning**
---
