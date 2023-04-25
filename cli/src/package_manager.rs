use std::{fmt::Display, str::FromStr};

use crate::template::Template;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackageManager {
    Pnpm,
    Yarn,
    Npm,
}

impl Default for PackageManager {
    fn default() -> Self {
        PackageManager::Pnpm
    }
}

impl<'a> PackageManager {
    pub const ALL: &'a [PackageManager] = &[
        PackageManager::Pnpm,
        PackageManager::Yarn,
        PackageManager::Npm,
    ];
}
impl PackageManager {
    /// Returns templates without flavors
    pub const fn templates(&self) -> &[Template] {
        match self {
            PackageManager::Pnpm | PackageManager::Yarn | PackageManager::Npm => &[
                Template::Fastify,
                Template::FastifyLavamoat,
                Template::Lambda,
                Template::LambdaLavamoat,
                Template::CloudflareWorkers,
            ],
        }
    }

   pub const fn install_cmd(&self) -> Option<&str> {
        match self {
            PackageManager::Pnpm => Some("pnpm install"),
            PackageManager::Yarn => Some("yarn"),
            PackageManager::Npm => Some("npm install"),
            _ => None,
        }
    }

    pub const fn run_cmd(&self) -> &str {
        match self {
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
            PackageManager::Npm => "npm run",
        }
    }

    pub const fn is_node(&self) -> bool {
        matches!(
            self,
            PackageManager::Pnpm | PackageManager::Yarn | PackageManager::Npm,
        )
    }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn"),
            PackageManager::Npm => write!(f, "npm"),
        }
    }
}

impl FromStr for PackageManager {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pnpm" => Ok(PackageManager::Pnpm),
            "yarn" => Ok(PackageManager::Yarn),
            "npm" => Ok(PackageManager::Npm),
            _ => Err("Invalid package manager".to_string()),
        }
    }
}
