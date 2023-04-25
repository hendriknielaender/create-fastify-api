use std::{fmt::Display, fs, io::Write, path, str::FromStr};

use anyhow::Context;
use rust_embed::RustEmbed;

use crate::{manifest::Manifest, package_manager::PackageManager};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/fragments"]
#[allow(clippy::upper_case_acronyms)]
struct FRAGMENTS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Template {
    Fastify,
    FastifyLavamoat,
    Lambda,
    LambdaLavamoat,
    CloudflareWorkers,
}

impl Default for Template {
    fn default() -> Self {
        Template::Fastify
    }
}

impl Template {
    pub const fn select_text<'a>(&self) -> &'a str {
        match self {
            Template::Fastify=> "Fastify - (https://www.fastify.io/)",
            Template::FastifyLavamoat=> "Fastify + Lavamoat - (https://www.fastify.io/)",
            Template::Lambda => "AWS Lambda - (https://aws.amazon.com/de/lambda/)",
            Template::LambdaLavamoat => "AWS Lambda + Lavamoat Runtime - (https://aws.amazon.com/de/lambda/)",
            Template::CloudflareWorkers => "Cloudflare workers - (https://workers.cloudflare.com/)",
            _ => unreachable!(),
        }
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Template::Fastify => write!(f, "fastify"),
            Template::FastifyLavamoat => write!(f, "fastify-lavamoat"),
            Template::Lambda => write!(f, "lambda"),
            Template::LambdaLavamoat => write!(f, "lambda-lavamoat"),
            Template::CloudflareWorkers => write!(f, "workers"),
        }
    }
}

impl FromStr for Template {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fastify" => Ok(Template::Fastify),
            "fastify-lavamoat" => Ok(Template::FastifyLavamoat),
            "lambda" => Ok(Template::Lambda),
            "lambda-lavamoat" => Ok(Template::LambdaLavamoat),
            "workers" => Ok(Template::CloudflareWorkers),
            _ => Err("Invalid template".to_string()),
        }
    }
}

impl<'a> Template {
    pub const ALL: &'a [Template] = &[
        Template::Fastify,
        Template::FastifyLavamoat,
        Template::Lambda,
        Template::LambdaLavamoat,
        Template::CloudflareWorkers,
    ];


    pub fn render(
        &self,
        target_dir: &path::Path,
        pkg_manager: PackageManager,
        package_name: &str,
    ) -> anyhow::Result<()> {
        let manifest_bytes = FRAGMENTS::get(&format!("fragment-{self}/_cta_manifest_"))
            .with_context(|| "Failed to get manifest bytes")?
            .data;
        let manifest_str = String::from_utf8(manifest_bytes.to_vec())?;
        let manifest = Manifest::parse(&manifest_str, false)?;

        let lib_name = format!("{}_lib", package_name.replace('-', "_"));

        let write_file = |file: &str| -> anyhow::Result<()> {
            let manifest = manifest.clone();

            // remove the first component, which is certainly the fragment directory they were in before getting embeded into the binary
            let p = path::PathBuf::from(file)
                .components()
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .collect::<path::PathBuf>();

            let p = target_dir.join(p);
            let file_name = p.file_name().unwrap().to_string_lossy();

            let target_file_name = match &*file_name {
                "_gitignore" => ".gitignore",
                "_Cargo.toml" => "Cargo.toml",
                "_cta_manifest_" => return Ok(()),
                name if name.starts_with("%(") && name[1..].contains(")%") => {
                    let mut s = name.strip_prefix("%(").unwrap().split(")%");
                    let (mut flags, name) = (
                        s.next().unwrap().split('-').collect::<Vec<_>>(),
                        s.next().unwrap(),
                    );

                    let for_stable = flags.contains(&"stable");

                    // remove these flags to only keep package managers flags
                    flags.retain(|e| !["stable"].contains(e));

                    if (for_stable)
                        && (flags.contains(&pkg_manager.to_string().as_str()) || flags.is_empty())
                    {
                        name
                    } else {
                        // skip writing this file
                        return Ok(());
                    }
                }
                _ => &file_name,
            };

            let mut data = FRAGMENTS::get(file).unwrap().data.to_vec();

            // Only modify specific set of files
            if [
                "Cargo.toml",
                "package.json",
            ]
            .contains(&target_file_name)
            {
                if let Ok(content) = String::from_utf8(data.to_vec()) {
                    // Replacement order is important
                    data = content
                        .replace("{{lib_name}}", &lib_name)
                        .replace("{{pkg_manager_run_command}}", pkg_manager.run_cmd())
                        .replace(
                            "{{fragment_before_dev_command}}",
                            manifest.before_dev_command.unwrap_or_default(),
                        )
                        .replace(
                            "{{fragment_before_build_command}}",
                            manifest.before_build_command.unwrap_or_default(),
                        )
                        .replace(
                            "{{fragment_dev_path}}",
                            manifest.dev_path.unwrap_or_default(),
                        )
                        .replace(
                            "{{fragment_dist_dir}}",
                            manifest.dist_dir.unwrap_or_default(),
                        )
                        .replace("{{package_name}}", package_name)
                        .replace("{{pkg_manager_run_command}}", pkg_manager.run_cmd())
                        .as_bytes()
                        .to_vec();
                }
            }

            let parent = p.parent().unwrap();
            fs::create_dir_all(parent)?;
            fs::write(parent.join(target_file_name), &data)?;
            Ok(())
        };

        for file in FRAGMENTS::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == "_base_"
        }) {
            write_file(&file)?;
        }

        // then write template files which can override files from base
        for file in FRAGMENTS::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == path::PathBuf::from(format!("fragment-{self}"))
        }) {
            write_file(&file)?;
        }

        // then write extra files specified in the fragment manifest
        for (src, dest) in manifest.files {
            let data = FRAGMENTS::get(&format!("_assets_/{src}"))
                .with_context(|| format!("Failed to get asset file bytes: {src}"))?
                .data;
            let dest = target_dir.join(dest);
            let parent = dest.parent().unwrap();
            fs::create_dir_all(parent)?;
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(dest)?;
            file.write_all(&data)?;
        }

        Ok(())
    }
}

