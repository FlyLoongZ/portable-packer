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
	ArchLinux,
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

	/**
		The path for supplied .desktop file (currently single)
	*/
	pub desktop_file:	std::path::PathBuf,
}
