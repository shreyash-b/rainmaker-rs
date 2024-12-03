# Prerequisits for building & running applications

## Rust compiler and cargo (obviously)
You can find instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started)

## ESP Rust development environment
This is required for building and flashing applications on ESP32 microcontrollers. \
This is recommended but you can skip this section if you're intending to run only on Linux platform. \
You can find the instructions for the same well documented at [ESP32 Rust Template](https://github.com/esp-rs/esp-idf-template#prerequisites). \
**Note:** You only need to perform instruction present in the **prerequisites** section.


## RainMaker CLI
RainMaker cloud backend used X509 based mutual authentication for authenticating the node. The resulting client certificate and key are together called claim data. \
RainMaker CLI is required for generating the claim data used by the node for authenticating with the backend.


- Installation: \
RainMaker CLI is available as a python package and can be installed using\
`pip install esp-rainmaker-cli` 


- Claiming: \
The process of generating the claim data making them available to the application is called as claiming. 

    - First you'll need to login to your RainMaker account:\
      `esp-rainmaker-cli login`

    - For generating and flashing the claim data on ESP32: \
      `esp-rainmaker-cli claim /dev/ttyUSB0 --addr 0x3FA000` \
      Here, replace `/dev/ttyUSB0` with the serial port your ESP32 is connected to

    - For generating claim data for use with Linux systems: \
      ESP RainMaker doesn't firsthand support Linux devices so we need to do a little workaround for using the said claim data. \
      First, note the MAC address of your device. This is used for uniquely identifying and regenerating claim data for your node(system). \
      The steps to do this can vary based on the distribution/DE you are using.

      Then you can generate the claim data by running: \
      `esp-rainmaker-cli claim /dev/null --mac <mac_addr>` \
      substitute `<mac_addr>` with MAC address of your network card in the format `AABBCC112233`

      Thereafter, the generated claim data is stored in `~/.espressif/rainmaker/claim_data/<accound_id>/<mac_addr>`. \
      You'll need to specify this path as `RMAKER_CLAIMDATA_PATH` while running application for the first time on Linux.

