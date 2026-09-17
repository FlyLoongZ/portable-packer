pub mod cmdline;
pub mod config;
pub mod config_toml;
pub mod config_legacy;
mod execute;

#[derive(Debug)]
pub enum OperationMode {
	Help,
	Copy {
		source:		CopySource,
		dest:		InstallDestination,
		options:	RuntimeOptions,
	},
	PostOnly {
		dest:		InstallDestination,
		options:	RuntimeOptions,
	},
}

/**
	Source of which to copy from.
*/
#[derive(Debug)]
pub enum CopySource {
	/**
		A local pacman package installed on the system
	*/
	PacmanLocal {
		pkgname:	String,
	},
}

/**
	Destination distro to install, should default to Arch when unspecified
*/
#[derive(Debug)]
pub enum InstallDestination {
	ArchLinux {
		pkgdir:		std::sync::Arc<std::path::PathBuf>,
		pkgname:	std::sync::Arc<String>,
	},
}

/**
	The public struct RuntimeOptions describes both decoded configuration and options parsed from
		command line arguments
*/
#[derive(Debug)]
pub struct RuntimeOptions {
	/**
		The sandbox_id is the equivalent of config's sandbox_id

		It represents a unique, fixed identity of a sandbox.
	*/
	pub config:		crate::pref::config::Config,

	pub config_path:	PortableConfig,

	/**
		The path for supplied .desktop file (currently single)
	*/
	pub desktop_file:	std::path::PathBuf,
}

#[derive(Debug)]
pub enum PortableConfig {
	Modern(std::path::PathBuf),
	Legacy(std::path::PathBuf),
}

impl AsRef<std::path::Path> for PortableConfig {
	fn as_ref(&self) -> &std::path::Path {
		match self {
			Self::Modern(v)	=> v.as_path(),
			Self::Legacy(v)	=> v.as_path(),
		}
	}
}
