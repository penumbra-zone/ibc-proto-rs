impl serde::Serialize for Module {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if true {
            len += 1;
        }
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.bank.module.v1.Module", len)?;
        if true {
            struct_ser.serialize_field("blockedModuleAccountsOverride", &self.blocked_module_accounts_override)?;
        }
        if true {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Module {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blocked_module_accounts_override",
            "blockedModuleAccountsOverride",
            "authority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockedModuleAccountsOverride,
            Authority,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockedModuleAccountsOverride" | "blocked_module_accounts_override" => Ok(GeneratedField::BlockedModuleAccountsOverride),
                            "authority" => Ok(GeneratedField::Authority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Module;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.bank.module.v1.Module")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<Module, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blocked_module_accounts_override__ = None;
                let mut authority__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockedModuleAccountsOverride => {
                            if blocked_module_accounts_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockedModuleAccountsOverride"));
                            }
                            blocked_module_accounts_override__ = Some(map.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Module {
                    blocked_module_accounts_override: blocked_module_accounts_override__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.bank.module.v1.Module", FIELDS, GeneratedVisitor)
    }
}
