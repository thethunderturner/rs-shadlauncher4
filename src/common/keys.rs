use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Keys {
    #[serde(rename = "DebugRifKeyset")]
    debug_rif_keyset: Keyset,

    #[serde(rename = "FakeKeyset")]
    fake_keyset: Keyset,

    #[serde(rename = "PkgDerivedKey3Keyset")]
    pkg_derived_key3_keyset: Keyset,

    #[serde(rename = "TrophyKeySet")]
    trophy_key_set: TrophyKeySet,
}

#[derive(Debug, Deserialize)]
struct Keyset {
    coefficient: String,
    exponent1: String,
    exponent2: String,
    modulus: String,
    prime1: String,
    prime2: String,
    private_exponent: String,
    public_exponent: String,
}

#[derive(Debug, Deserialize)]
struct TrophyKeySet {
    release_trophy_key: String,
}
