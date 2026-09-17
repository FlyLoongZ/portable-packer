enum OpMode {
	Help,
	Copy,
	Post,
}

enum Distro {
	Arch,
}

impl From<&str> for Distro {
	fn from(value: &str) -> Self {
		match value {
			"arch" | "archlinux"	=> {
				Self::Arch
			}
			v			=> {
				unimplemented!("Unknown distribution codename: {v:?}")
			}
		}
	}
}

impl Distro {
	fn to_copy_source(self, pkgname: String) -> super::CopySource {
		match self {
			Self::Arch	=> {
				super::CopySource::PacmanLocal { pkgname: pkgname }
			}
		}
	}
}

impl Into<super::InstallDestination> for Distro {
	fn into(self) -> super::InstallDestination {
		match self {
			Self::Arch	=> {
				super::InstallDestination::ArchLinux {
					pkgdir:		std::path::PathBuf::from(
						std::env::var("pkgdir")
							.expect("Expected a $pkgdir variable")
						)
						.into(),
					pkgname:	std::env::var("pkgname")
						.expect("Expected a $pkgname variable")
						.into(),
				}
			}
		}
	}
}

/**
	Get the user preference and possibly configuration data from cmdline
*/
pub async fn get_pref() -> super::OperationMode {
	let mut args = std::env::args();
	args.next();

	if args.len() <= 1 {
		return super::OperationMode::Help;
	};

	let mut mode: Option<OpMode> = None;
	let mut config: Option<super::config::Config> = None;
	let mut config_fs: Option<super::PortableConfig> = None;
	let mut distro: Option<(Distro, Distro)> = None;
	let mut desktop_path: Option<std::path::PathBuf> = None;
	let mut copy_source: Option<String> = None;

	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--distro"		=> {
				let next_arg = args
					.next()
					.expect("Expected distribution names as src:dst");

				match next_arg.split_once(":")
				{
					Some(v)	=> {
						distro = Some(
							(
								Distro::from(v.0),
								Distro::from(v.1),
							)
						)
					}
					None	=> {
						eprintln!(
						"Expected distribution names as src:dst, assuming src = dst"
						);
						distro = Some(
							(
								Distro::from(next_arg.as_str()),
								Distro::from(next_arg.as_str()),
							)
						)
					}
				};
			}
			"--mode"		=> {
				match args.next().expect("Expected argument after --mode").as_str() {
					"copy"	=> {
						mode = Some(OpMode::Copy);
						copy_source = Some(
							args
								.next()
								.expect("Expected source after copy")
						)
					}
					"post"	=> {
						mode = Some(OpMode::Post);
					}
					v	=> {
						panic!("Could not parse cmdline: unexpected argument {v:?} after --mode")
					}
				};
			}
			"--config"		=> {
				eprintln!("Legacy configuration is deprecated in Portable 14");

				let path: std::path::PathBuf = args.next().expect("Expected path after --config").into();
				config_fs = Some(
					super::PortableConfig::Legacy(path.to_path_buf())
				);

				config = Some(
					super::config_legacy::get(&path)
						.await
				)

			}
			"--config-ng"		=> {
				let path: std::path::PathBuf = args.next().expect("Expected path after --config-ng").into();
				config_fs = Some(
					super::PortableConfig::Modern(path.to_path_buf())
				);

				config = Some(super::config_toml::get(&path).await)
			}
			"--desktop-file"	=> {
				desktop_path = Some(
					args.next().expect("Expected path after").into()
				)
			}
			v			=> {
				eprintln!("Unimplemented argument: {v:?}")
			}
		}
	};

	let runtime_options = super::RuntimeOptions {
		config:		{
			config.expect("Expected a configuration")
		},
		config_path:	config_fs.expect("Expected a configuration"),
		desktop_file:	desktop_path.expect("Expected a desktop file"),
	};

	match mode.unwrap_or(OpMode::Help) {
		OpMode::Help	=> {
			super::OperationMode::Help
		}
		OpMode::Copy	=> {
			let distro = distro.expect("No distribution specified");

			super::OperationMode::Copy {
				source:		distro.0.to_copy_source(
					copy_source.expect(
						"Expected source to copy from",
					),
				),
				dest:		distro.1.into(),
				options:	runtime_options,
			}
		}
		OpMode::Post	=> {

			super::OperationMode::PostOnly {
				dest:	distro
					.expect("No distribution specified")
					.1
					.into(),
				options: runtime_options,
			}
		}
	}

}
