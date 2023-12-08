use std::path::{Path, PathBuf};
use std::process;

use similar::TextDiff;
use walkdir::WalkDir;

use argh::FromArgs;
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "compile")]
/// Compile
pub struct CompileCmd {
    #[argh(option, short = 'i')]
    /// path to the IBC-Go proto files
    ibc: PathBuf,

    #[argh(option, short = 's')]
    /// path to the Cosmos SDK proto files
    sdk: PathBuf,

    #[argh(option, short = 'c')]
    /// path to the Cosmos ICS proto files
    ics: PathBuf,

    #[argh(option, short = 'o')]
    /// path to output the generated Rust sources into
    out: PathBuf,
}

impl CompileCmd {
    pub fn run(&self) {
        Self::compile_ibc_protos(
            self.ibc.as_ref(),
            self.sdk.as_ref(),
            self.ics.as_ref(),
            self.out.as_ref(),
        )
        .unwrap_or_else(|e| {
            eprintln!("[error] failed to compile protos: {}", e);
            process::exit(1);
        });

        Self::patch_generated_files(self.out.as_ref()).unwrap_or_else(|e| {
            eprintln!("[error] failed to patch generated files: {}", e);
            process::exit(1);
        });

        println!("[info ] Done!");
    }

    fn compile_ibc_protos(
        ibc_dir: &Path,
        sdk_dir: &Path,
        ics_dir: &Path,
        out_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Compiling IBC .proto files to Rust into '{}'...",
            out_dir.display()
        );

        let root = env!("CARGO_MANIFEST_DIR");

        // Paths
        let proto_paths = [
            format!("{}/../../definitions/mock", root),
            format!("{}/../../definitions/ibc/lightclients/localhost/v1", root),
            format!("{}/../../definitions/stride/interchainquery/v1", root),
            format!("{}/ibc", ibc_dir.display()),
            format!("{}/cosmos/auth", sdk_dir.display()),
            format!("{}/cosmos/gov", sdk_dir.display()),
            format!("{}/cosmos/tx", sdk_dir.display()),
            format!("{}/cosmos/base", sdk_dir.display()),
            format!("{}/cosmos/crypto", sdk_dir.display()),
            format!("{}/cosmos/bank", sdk_dir.display()),
            format!("{}/cosmos/staking", sdk_dir.display()),
            format!("{}/cosmos/upgrade", sdk_dir.display()),
            format!("{}/interchain_security/ccv/v1", ics_dir.display()),
            format!("{}/interchain_security/ccv/provider", ics_dir.display()),
            format!("{}/interchain_security/ccv/consumer", ics_dir.display()),
        ];

        let proto_includes_paths = [
            format!("{}", sdk_dir.display()),
            format!("{}", ibc_dir.display()),
            format!("{}", ics_dir.display()),
            format!("{}/../../definitions/mock", root),
            format!("{}/../../definitions/ibc/lightclients/localhost/v1", root),
            format!("{}/../../definitions/stride/interchainquery/v1", root),
        ];

        // List available proto files
        let mut protos: Vec<PathBuf> = vec![];
        for proto_path in &proto_paths {
            println!("Looking for proto files in {:?}", proto_path);
            protos.append(
                &mut WalkDir::new(proto_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.file_type().is_file()
                            && e.path().extension().is_some()
                            && e.path().extension().unwrap() == "proto"
                    })
                    .map(|e| e.into_path())
                    .collect(),
            );
        }

        println!("Found the following protos:");
        // Show which protos will be compiled
        for proto in &protos {
            println!("\t-> {:?}", proto);
        }
        println!("[info ] Compiling..");

        // List available paths for dependencies
        let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

        // Automatically derive a `prost::Name` implementation.
        let mut config = prost_build::Config::new();
        config.enable_type_names();

        let descriptor_set_path = out_dir.join("proto_descriptor.bin");

        tonic_build::configure()
            .build_client(true)
            .compile_well_known_types(true)
            .extern_path(".google.protobuf", "::pbjson_types")
            .client_mod_attribute(".", r#"#[cfg(feature = "client")]"#)
            .build_server(true)
            .server_mod_attribute(".", r#"#[cfg(feature = "server")]"#)
            .out_dir(out_dir)
            .file_descriptor_set_path(descriptor_set_path.clone())
            .extern_path(".tendermint", "::tendermint_proto")
            .extern_path(".ics23", "::ics23")
            .compile_with_config(config, &protos, &includes)?;


        println!("[info] Building pbjson Serialize, Deserialize impls...");
        let descriptor_set = std::fs::read(descriptor_set_path.clone())?;

        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .out_dir(&out_dir)
            .build(&[".ibc"])?;

        println!("[info ] Protos compiled successfully");

        Ok(())
    }

    fn patch_generated_files(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Patching generated files in '{}'...",
            out_dir.display()
        );

        const PATCHES: &[(&str, &[(&str, &str)])] = &[
            (
                "cosmos.staking.v1beta1.rs",
                &[
                    ("pub struct Validators", "pub struct ValidatorsVec"),
                    (
                        "impl ::prost::Name for Validators {",
                        "impl ::prost::Name for ValidatorsVec {",
                    ),
                    ("AllowList(Validators)", "AllowList(ValidatorsVec)"),
                    ("DenyList(Validators)", "DenyList(ValidatorsVec)"),
                ],
            ),
            (
                "ibc.applications.transfer.v1.rs",
                &[(
                    "The denomination trace (\\[port_id\\]/[channel_id])+/\\[denom\\]",
                    "The denomination trace `([port_id]/[channel_id])+/[denom]`",
                )],
            ),
        ];

        for (file, patches) in PATCHES {
            println!("[info ] Patching {file}...");

            let path = out_dir.join(file);
            let original = std::fs::read_to_string(&path)?;
            let mut patched = original.clone();

            for (before, after) in patches.iter() {
                patched = patched.replace(before, after);
            }

            let diff = TextDiff::from_lines(&original, &patched);
            println!("{}", diff.unified_diff().context_radius(3));

            std::fs::write(&path, patched)?;
        }

        Ok(())
    }
}
