impl super::OperationMode {
	pub async fn act(self) {
		match self {
			Self::Help	=> {
				crate::help::help();
				return
			}
			Self::Copy { source, dest, options }
					=> {
				println!(
					"Executing the copy stage: source {:?}, destination {:?}, options: {:?}",
					&source,
					&dest,
					&options,
				);
				let file_list = match source {
					super::CopySource::PacmanLocal { pkgname }	=> {
						use crate::distro::file_list::archlinux::Arch;
						use crate::distro::file_list::GetFileList;
						let package = Arch::LocalPackage { pkgname };
						package
							.list()
							.await
							.expect("Could not generate a list to copy from")
					}
				};

				let (pkgdir, pkgname ) = match dest {
					crate::pref::InstallDestination::ArchLinux { pkgdir, pkgname }
						=> {
							(
								pkgdir,
								pkgname,
							)
						}
				};

				let workers = {
					let mut workers = vec![];

					for file in file_list {
						workers.push(
							tokio::spawn(
								file
									.copy(pkgdir.clone())
							)
						);
					};

					workers
				};

				for worker in workers {
					worker
						.await
						.expect("Could not spawn copy task")
						.expect("Could not copy from source")
				};

				let (post_object, options) = {
					let post_object = crate::distro::post_install::archlinux::ArchPost {
						pkgdir:		pkgdir,
						pkgname:	pkgname,
					};
					(post_object, options)
				};

				post_install(post_object, options).await;
			}

			Self::PostOnly { dest, options } => {
				let post_object = match dest {
					crate::pref::InstallDestination::ArchLinux { pkgdir, pkgname }
					=> {
						crate::distro::post_install::archlinux::ArchPost {
							pkgdir:		pkgdir,
							pkgname:	pkgname,
						}
					}
				};

				post_install(post_object, options).await;
			}
		};
	}
}

async fn post_install (object: impl crate::distro::post_install::traits::PostInstall, options: crate::pref::RuntimeOptions) {
	let sandbox_id = std::sync::Arc::new(options.config.metadata.sandbox_id);

	let _binary_name = object.binary(
		sandbox_id.clone(),
		options.config.exec.overlay,
	)
		.await
		.expect("Could not install or remove binaries");

	object.desktop_file(
		sandbox_id.clone(),
		options.desktop_file,
	)
		.await
		.expect("Could not install or remove .desktop file");

	object.dbus_service(
		sandbox_id.clone(),
		options.config.dbus_activation.enable,
	)
		.await
		.expect("Could not install or remove D-Bus service");

	object.gnome_shell()
		.await
		.expect("Could not install or remove GNOME Shell service");
}
