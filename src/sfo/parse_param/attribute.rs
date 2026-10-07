const SUPPORTS_VR: u32 = 0x0000_4000;
const SUPPORTS_NEO: u32 = 0x0080_0000;
const REQUIRES_VR: u32 = 0x0400_0000;

const FLAGS: &[(u32, &str)] = &[
    (
        0x0000_0001,
        "The application does support the initial user's logout",
    ),
    (
        0x0000_0002,
        "Enter Button Assignment for the common dialog: Cross button",
    ),
    (
        0x0000_0004,
        "Menu for Warning Dialog for PS Move is displayed in the option menu",
    ),
    (0x0000_0008, "The application supports Stereoscopic 3D"),
    (
        0x0000_0010,
        "The application is suspended when PS button is pressed (e.g. Amazon Instant Video)",
    ),
    (
        0x0000_0020,
        "Enter Button Assignment for the common dialog: Assigned by the System Software",
    ),
    (
        0x0000_0040,
        "The application overwrites the default behavior of the Share Menu",
    ),
    (
        0x0000_0100,
        "The application is suspended when the special output resolution is set and PS button is pressed",
    ),
    (0x0000_0200, "HDCP is enabled"),
    (0x0000_0400, "HDCP is disabled for non games app"),
    (SUPPORTS_VR, "This Application supports PlayStation VR"),
    (0x0000_8000, "CPU mode (6 CPU)"),
    (0x0001_0000, "CPU mode (7 CPU)"),
    (SUPPORTS_NEO, "The application supports NEO mode (PS4 pro)"),
    (REQUIRES_VR, "The Application Requires PlayStation VR"),
    (0x2000_0000, "This Application Supports HDR"),
    (0x8000_0000, "Display Location (?)"),
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

pub fn supports_vr(bytes: [u8; 4]) -> bool {
    u32::from_le_bytes(bytes) & (SUPPORTS_VR | REQUIRES_VR) != 0
}

pub fn supports_neo(bytes: [u8; 4]) -> bool {
    u32::from_le_bytes(bytes) & SUPPORTS_NEO != 0
}
