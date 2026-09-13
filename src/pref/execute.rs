impl super::OperationMode {
	pub async fn act(self) {
		match self {
			Self::Help	=> {
				unimplemented!("help");
				// return;
			}
			Self::CopyArch { options }
					=> {
				println!("Copying from an Arch Linux package");
				unimplemented!();
			}
			Self::PostOnlyArch { options }
					=> {
				unimplemented!();
			}
		}
	}
}
