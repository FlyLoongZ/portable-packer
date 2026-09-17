#[cfg(feature = "archlinux")]
pub mod archlinux;

pub mod symlink_parse;

/**
	The public trait GetFileList is implemented by multiple backends
*/
pub trait GetFileList {
	fn list(&self)
	-> impl std::future::Future<Output = Result<Vec<PackageFile>, Self::ListError>> + Send;

	type ListError;
}

/**
	The public enum PackageFile represents a file in a package.

	It describes several key information to implement the "copy" action.
*/
#[derive(Debug)]
pub enum PackageFile {
	Regular {
		/**
			source_path describes the current file path on disk.
		*/
		source_path:	std::path::PathBuf,
	},

	Symlink {
		/**
			source_path describes the destination to which the link exists.

			The parent path will be automatically created.
		*/
		source_path:	std::path::PathBuf,

		/**
			link_target should be a path describing the link target
		*/
		link_target:	std::path::PathBuf,
	}
}

impl PackageFile {
	/**
		Install the file or symbolic link into the package directory
	*/
	pub async fn copy(self, pkgdir: std::sync::Arc<std::path::PathBuf>) -> Result<(), std::io::Error> {
		match self {
			Self::Regular { source_path }	=> {
				let install_path = {
					let mut path = pkgdir.to_path_buf();

					for element in source_path.components() {
						match element {
							std::path::Component::RootDir	=> {
								continue;
							}
							component			=> {
								path.push(component);
							}
						}
					};

					path
				};

				println!("Installing {source_path:?} to {install_path:?}");

				match install_path.parent() {
					Some(v)	=> {
						tokio::fs::DirBuilder::new()
							.recursive(true)
							.mode(0o755)
							.create(v)
							.await
							?;
					}
					None	=> {}
				};

				tokio::fs::copy(
					source_path,
					install_path,
				)
					.await
					?;
				Ok(())
			}
			Self::Symlink { source_path, link_target }	=> {
				let install_path = {
					let mut path = pkgdir.to_path_buf();

					for element in source_path.components() {
						match element {
							std::path::Component::RootDir	=> {
								continue;
							}
							component			=> {
								path.push(component);
							}
						}
					};

					path
				};

				println!("Linking {install_path:?} to {link_target:?}");

				match install_path.parent() {
					Some(v)	=> {
						tokio::fs::DirBuilder::new()
							.recursive(true)
							.mode(0o755)
							.create(v)
							.await
							?;
					}
					None	=> {}
				};

				tokio::fs::symlink(
					link_target,
					install_path,
				)
					.await
					?;
				Ok(())
			}
		}
	}
}
