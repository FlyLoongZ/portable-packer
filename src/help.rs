pub fn help() {
	let help_content = {
		let mut content = String::new();

		content.push_str("This is Portable packer, a tool to build sandboxed package.\n");
		content.push_str("Visit https://github.com/Kraftland/portable for documentation and information.");

		content
	};

	println!("{}", help_content)
}
