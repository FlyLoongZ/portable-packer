/**
	Test the portable package for file list extraction
*/
#[tokio::test]
pub async fn test_package() {
	unsafe {
		std::env::set_var("pkgdir", "/package_root")
	};


	let list = super::read_files::get("portable").await.unwrap();

	if list.len() == 0 {
		panic!("Empty file list!")
	};

	println!("Got path list for packer: {list:?}")
}
