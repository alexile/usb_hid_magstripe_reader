# usb_hid_magstripe_reader

`usb_hid_magstripe_reader` is a Rust library that provides a simple interface for reading data from a USB magstripe reader.
It allows users to easily integrate magstripe reader functionality into their Rust applications, as normally it acts as a keyboard.

## Features

- A crappy 5-minute project to make a cheap Aliexpress magstripe reader work
- Suddenly, it works

## Installation

To use `usb_hid_magstripe_reader` in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
usb_hid_magstripe_reader = "0.1.0"
```

## Usage

Here's a simple example demonstrating how to use `usb_hid_magstripe_reader`:

```rust
use usb_hid_magstripe_reader::MagstripeReader;

fn main() {
    let vendor_id = 0xffff; // Replace with your magstripe reader's vendor ID, mine came with 0xffff
    let product_id = 0x0035; // Replace with your magstripe reader's product ID, mine came with 0x0035

    let result = MagstripeReader::read(vendor_id, product_id); // At this point, you need to swipe a card

    println!("Magstripe data: {}", data)
}
```

Make sure to replace the `vendor_id` and `product_id` values with the appropriate values for your specific magstripe reader.
#### How to get vendor_id and product_id:
1. `lsusb`
2. Find your device (e.g. MEGAHUNT Megahunt HID FS Keyboard / Run Mall)
3. Use `ffff:0035` part, where `ffff` is a vendor_id and `0035` is a product_id

## Troubleshooting

There's a chance the library cannot open a device.
1. Check `dmesg`. If it writes something like `Device is not authorized for usage`, it means `usbguard` doesn't recognize the device
2. Find device's id `usbguard list-devices`
3. Allow the device `usbguard allow-device -p 77`
4. `sudo chmod -R 777 /dev/bus/usb/`, why not

## Contributing

Contributions to `usb_hid_magstripe_reader` are welcome! If you encounter any issues or have suggestions for improvements, please open an issue on the [GitHub repository](https://github.com/alexile/usb_hid_magstripe_reader). Pull requests are also appreciated.