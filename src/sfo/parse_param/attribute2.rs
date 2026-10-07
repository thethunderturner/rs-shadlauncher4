const FLAGS: &[(u32, &str)] = &[
    (
        0x0000_0002,
        "The application supports Video Recording Feature (SDK4000 or newer)",
    ),
    (
        0x0000_0004,
        "The application supports Content Search Feature (SDK4000 or newer)",
    ),
    (
        0x0000_0010,
        "PSVR Personal Eye-to-Eye distance setting disabled (SDK4000 or newer)",
    ),
    (
        0x0000_0020,
        "PSVR Personal Eye-to-Eye distance dynamically changeable (SDK4000 or newer)",
    ),
    (
        0x0000_0100,
        "The application supports broadcast separate mode",
    ),
    (
        0x0000_0200,
        "The library does not apply dummy load for tracking Playstation Move to CPU (SDK4000 or newer)",
    ),
    (
        0x0000_0800,
        "The application supports One on One match event with an old SDK (SDK 3500 or older)",
    ),
    (
        0x0000_1000,
        "The application supports Team on team tournament with an old SDK (SDK 4500 or older)",
    ),
];

// Undocumented bits are ignored; the original bytes are not modified.
pub fn description(bytes: [u8; 4]) -> Vec<String> {
    let value = u32::from_le_bytes(bytes);
    FLAGS
        .iter()
        .filter(|(flag, _)| value & flag != 0)
        .map(|(_, text)| (*text).to_owned())
        .collect()
}
