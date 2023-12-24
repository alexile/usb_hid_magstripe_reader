use std::collections::HashMap;

pub struct MagstripeReader {}

impl MagstripeReader {
    pub fn read(vendor_id: u16, product_id: u16) -> String {
        let scancode_map = HashMap::from([
            ("4", "a"),
            ("5", "b"),
            ("6", "c"),
            ("7", "d"),
            ("8", "e"),
            ("9", "f"),
            ("a", "g"),
            ("b", "h"),
            ("c", "i"),
            ("d", "j"),
            ("e", "k"),
            ("f", "l"),
            ("10", "m"),
            ("11", "n"),
            ("12", "o"),
            ("13", "p"),
            ("14", "q"),
            ("15", "r"),
            ("16", "s"),
            ("17", "t"),
            ("18", "u"),
            ("19", "v"),
            ("1a", "w"),
            ("1b", "x"),
            ("1c", "y"),
            ("1d", "z"),
            ("1e", "1"),
            ("1f", "2"),
            ("20", "3"),
            ("21", "4"),
            ("22", "5"),
            ("23", "6"),
            ("24", "7"),
            ("25", "8"),
            ("26", "9"),
            ("27", "0"),
            ("28", "\n"),
            ("2c", " "),
            ("2d", "-"),
            ("2e", "="),
            ("2f", "["),
            ("30", "]"),
            ("31", "\\"),
            ("32", "#"),
            ("33", ";"),
            ("34", "'"),
            ("35", "`"),
            ("36", ","),
            ("37", "."),
            ("38", "/")
        ]);
        let shift_scancode_map = HashMap::from([
            ("4", "A"),
            ("5", "B"),
            ("6", "C"),
            ("7", "D"),
            ("8", "E"),
            ("9", "F"),
            ("a", "G"),
            ("b", "H"),
            ("c", "I"),
            ("d", "J"),
            ("e", "K"),
            ("f", "L"),
            ("10", "M"),
            ("11", "N"),
            ("12", "O"),
            ("13", "P"),
            ("14", "Q"),
            ("15", "R"),
            ("16", "S"),
            ("17", "T"),
            ("18", "U"),
            ("19", "V"),
            ("1a", "W"),
            ("1b", "X"),
            ("1c", "Y"),
            ("1d", "Z"),
            ("1e", "!"),
            ("1f", "@"),
            ("20", "#"),
            ("21", "$"),
            ("22", "%"),
            ("23", "^"),
            ("24", "&"),
            ("25", "*"),
            ("26", "("),
            ("27", ")"),
            ("28", "\n"),
            ("2c", " "),
            ("2d", "_"),
            ("2e", "+"),
            ("2f", "{"),
            ("30", "}"),
            ("31", "|"),
            ("32", "~"),
            ("33", ":"),
            ("34", "\""),
            ("35", "~"),
            ("36", "<"),
            ("37", ">"),
            ("38", "?"),
        ]);
        let mut device_handle = rusb::open_device_with_vid_pid(vendor_id, product_id).expect("Cannot open the device");
        let interface = 0;
        let endpoint = 0x81;
        const MAX_BUFFER_LENGTH: usize = 8;

        let has_kernel_driver = device_handle.kernel_driver_active(interface).expect("Cannot find the interface");
        if has_kernel_driver {
            device_handle.detach_kernel_driver(interface).expect("Cannot detach a kernel driver");
        }
        let mut buf = [0u8; MAX_BUFFER_LENGTH];
        let mut is_reading = false;
        let mut result = "".to_string();

        loop {
            match device_handle.read_interrupt(endpoint, &mut buf, std::time::Duration::from_millis(100)) {
                Ok(_) => {
                    is_reading = true;
                    let codes = &buf[..MAX_BUFFER_LENGTH];
                    let modifier_key = format!("{:x?}", codes[0]);
                    let keys: Vec<String> = codes[2..].iter().map(|key| {
                        format!("{:x?}", key)
                    }).collect();

                    for key in keys {
                        if modifier_key == "20" {
                            let character = match shift_scancode_map.get(key.as_str()) {
                                Some(value) => {
                                    value.to_owned()
                                },
                                None => ""
                            };
                            result.push_str(character);
                        } else {
                            let character = match scancode_map.get(key.as_str()) {
                                Some(value) => {
                                    value.to_owned()
                                },
                                None => ""
                            };
                            result.push_str(character);
                        }
                    }
                }
                Err(_) => {
                    if is_reading {
                        break;
                    } else {
                        is_reading = false;
                    }
                }
            }
        }
        result
    }
}
