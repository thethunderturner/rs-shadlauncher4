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
        .filter(|(flag, _)| value == *flag)
        .map(|(_, text)| (*text).to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::description;

    #[test]
    fn app_type_is_one_value_not_a_bitmask() {
        assert_eq!(description(0u32.to_le_bytes()), vec!["Not Specified"]);
        assert_eq!(description(3u32.to_le_bytes()), vec!["Demo App"]);
        assert!(description(5u32.to_le_bytes()).is_empty());
    }
}
