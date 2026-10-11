const FLAGS: &[(u32, &str)] = &[
    (0, "Not Specified"),
    (1, "Paid Standalone Full App"),
    (2, "Upgradable App"),
    (3, "Demo App"),
    (4, "Freemium App"),
];

// https://www.psdevwiki.com/ps4/Param.sfo#APP_TYPE
pub fn description(bytes: [u8; 4]) -> Vec<String> {
    let value = u32::from_le_bytes(bytes);
    FLAGS
        .iter()
        .filter(|(flag, _)| value & flag != 0)
        .map(|(_, text)| (*text).to_owned())
        .collect()
}