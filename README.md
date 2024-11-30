# Rust Implementation of ESP Rainmaker

A cross-platform implementation of ESP Rainmaker for ESP32 products and Linux using Rust.
---

- ESP RainMaker is end-to-end IoT development platform which enables development of IoT applications which can be controled remotely.
- However, the C based ESP RainMaker SDK(which can be found [here](https://github.com/espressif/esp-rainmaker)) only supports execution on Espressif's ESP32 SOCs.
- This crate tries to  implment similar functionalities for Linux platform along with ESP32(which can furter be extended to other microcontrollers).`


## Prerequisites

- Follow the [Prerequisites](https://github.com/esp-rs/esp-idf-template#prerequisites) section in the ``` esp-idf-template``` crate. (only required for running on ESP32)
- Install ESP-Rainmaker app on your phone.

## Execution instructions
### On ESP32 series of devices
1. Erase flash contents

```bash
espflash erase-flash
```

2. Use [rainmaker cli](https://rainmaker.espressif.com/docs/cli-setup.html) to perform manual claiming on ESP devices. Claiming is required for providing the device with appropriate certificates for communicating with ESP RainMaker backend.   
This needs to be performed only first time after flash contents are erased.

```bash
esp-rainmaker-cli claim /dev/ttyUSB0 --addr 0x3FA000
```

3. Build and run the project

```bash
cargo run --target <mcu-target> --bin <name of example>
```
List of targets for various chipsets:

| MCU | Target     |
| :-------- | :------- |
| ESP32 | `xtensa-esp32-espidf` |
| ESP32-S2 | `xtensa-esp32-espidf` |
| ESP32-S3 | `xtensa-esp32-espidf` |
| ESP32-C2 | `riscv32imc-esp-espidf` |
| ESP32-C3 | `riscv32imc-esp-espidf` |
| ESP32-C6 | `riscv32imc-esp-espidf` |

## Running on Linux

1. Create directories for storing persistent data

```bash
mkdir -p ~/.config/rmaker/fctry
mkdir -p ~/.config/rmaker/nvs
```

2. Initialize certificates for your device using esp-rainmaker-cli

```bash
esp-rainmaker-cli login
esp-rainmaker-cli claim --mac <MAC addr>
```
The certificates are stored in `/home/<user>/.espressif/rainmaker/claim_data/<account_id>/<mac_address>`

3. Run

```bash
cargo run --target x86_64-unknown-linux-gnu --bin <example>
```
OR
```bash
cargo run_linux --bin <example>
```

Once the example is running, open the rainmaker mobile application and follow on-screen instructions for adding device


When running for the first time, you'll need to set ```RMAKER_CLAIMDATA_PATH``` environment variable to the folder containing your claim data (mentioned in step )


---