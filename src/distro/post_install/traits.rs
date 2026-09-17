pub trait PostInstall: Send {
	/**
		Overlay the contents in bin into app-private copy of OverlayFS.

		Deletes original binaries, and generates a new stub one for launching the sandbox.

		The resulting String would be that binary name.
	*/
	fn binary(&self, app_id: std::sync::Arc<String>, overlay: bool) -> impl std::future::Future<Output = Result<String, Self::PostError>> + Send;

	/**
		Removes any .desktop file that is installed in the package root and autostart.

		Installs the new one into package root.
	*/
	fn desktop_file(
		&self,
		app_id:		std::sync::Arc<String>,
		desktop_file:	std::path::PathBuf,
	) -> impl std::future::Future<Output = Result<(), Self::PostError>> + Send;

	/**
		Removes any D-Bus service installed in package root.

		Generates a new one and installs them if enabled.
	*/
	fn dbus_service(
		&self,
		app_id:		std::sync::Arc<String>,
		generate:	bool,
	) -> impl std::future::Future<Output = Result<(), Self::PostError>> + Send;

	/**
		Removes any GNOME Shell Extensions, Modes installed in the system.

		Search Provider is not preserved, with sandbox_id.ini being the name.
	*/
	fn gnome_shell(
		&self,
	) -> impl std::future::Future<Output = Result<(), Self::PostError>> + Send;

	/**
		Install a Portable config
	*/
	fn portable_config(
		&self,
		config_fs:	crate::pref::PortableConfig,
		app_id:		std::sync::Arc<String>,
	) -> impl std::future::Future<Output = Result<(), Self::PostError>> + Send;

	type PostError: std::fmt::Debug;
}

