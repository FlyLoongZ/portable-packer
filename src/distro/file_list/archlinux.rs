mod read_files;

#[cfg(test)]
pub mod test;

/**
	Implements the GetFileList trait
*/
pub enum Arch {
	LocalPackage {
		pkgname:	String,
	},
}

#[derive(Debug, thiserror::Error)]
pub enum ArchError {
	#[error("I/O error reading file: {0:#?}")]
	IOError(std::io::Error),

	#[error("I/O error listing pacman database: {0:#?}")]
	DbIOError(std::io::Error),

	#[error("I/O error reading pacman database desc: {0:#?}")]
	DbDescIOError(std::io::Error),

	#[error("I/O error reading pacman database files: {0:#?}")]
	DbFilesIOError(std::io::Error),

	#[error("No relevant entry in pacman database")]
	NoSuchPackageInDatabase,

	#[error("Error converting OsString to &str: {0:#?}")]
	OsStringError(std::ffi::OsString),

	#[error("I/O error reading symlink: {0:?}: {1:#?}")]
	SymlinkIOError(std::path::PathBuf, std::io::Error),

	#[error("Package file missing: {0:#?}")]
	FileMissing(std::path::PathBuf),

	#[error("Missing pkgdir variable")]
	MissingPkgdir,
}

impl crate::distro::file_list::GetFileList for Arch {
	async fn list(&self) -> Result<Vec<super::PackageFile>, Self::ListError> {
		let raw_list = match &self {
			Arch::LocalPackage { pkgname }	=> {
				read_files::get(&pkgname)
					.await
					?
			}
		};

		let mut ret = vec![];

		for path in raw_list {
			if path.is_dir() {
				println!("Skipping directory: {path:?}");
				continue;
			};

			if path.is_symlink() {
				let link_dest = match super::
							symlink_parse::
							read_dest(&path)
							.await {
					Ok(v)	=> v,
					Err(e)	=> {
						return Err(
							ArchError::SymlinkIOError(
								path,
								e,
							)
						);
					}
				};
				ret.push(
					super::PackageFile::Symlink {
						source_path:	path.to_path_buf(),
						link_target:	link_dest,
					}
				);
				continue;
			}

			match tokio::fs::try_exists(&path).await {
				Ok(true)	=> {}
				Ok(false)	=> {
					return Err(ArchError::FileMissing(path))
				}
				Err(e)		=> {
					return Err(
						ArchError::IOError(e)
					)
				}
			}

			ret.push(
				super::PackageFile::Regular {
					source_path:	path,
				}
			);
		};

		Ok(ret)
	}

	type ListError = ArchError;
}
