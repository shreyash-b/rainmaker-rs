# Rust Implementation of ESP Rainmaker

A cross-platform implementation of ESP Rainmaker for ESP32 products and Linux using Rust.
---

- ESP RainMaker is end-to-end IoT development platform which enables development of IoT applications which can be controled remotely.
- However, the C based ESP RainMaker SDK(which can be found [here](https://github.com/espressif/esp-rainmaker)) only supports execution on Espressif's ESP32 SOCs.
- This crate tries to  implment similar functionalities for Linux platform along with ESP32(which can further be extended to other microcontrollers).

## What is working  
- [x] WiFi Provisioning*: \
      Providing WiFi for a new device. No need to hardcode WiFi credentials!
- [x] Remote Control: \
      Controlling Devices connected to a node over internet using phone application.
- [x] User Node Association: \
      Associating a specific node to a user account for control.
- [x] Device sharing: \
      Share access to a device with multiple members.
- [x] Contol using Home Assistants: \
      RainMaker devices can be added to and controlled using Amazon Alexa / Google Home. More details [here](https://rainmaker.espressif.com/docs/3rd-party#enabling-alexa)

\* Currently only supported on ESP32


## Prerequisites
Refer [this](docs/PREREQUISITES.md) for setting up environment for building and running rainmaker-rs application.

## Executing Examples
### On ESP32 series of devices
1. Erase flash contents

```bash
espflash erase-flash
```

2. Make sure claiming is performed for your MCU(mentioned in [prerequisites](/docs/PREREQUISITES.md)) 
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

2. Make sure claiming is performed for your MCU(mentioned in [prerequisites](/docs/PREREQUISITES.md)) 
The claimdata is stored in `/home/<user>/.espressif/rainmaker/claim_data/<account_id>/<mac_address>`

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

## Or you can create a new project based on rainmaker-rs template 
- Install `cargo-generate` \
`$ cargo install cargo-generate`
- Create a new project \
`$ cargo generate rainmaker-rs/template`

More information about the generated template can be found at it's respective repository [here](https://github.com/rainmaker-rs/template)
