
extern crate git2;
extern crate cmake;

use git2::Repository;
use std::path::Path;
use std::ops::Add;

const LIBEBUR128_GIT_URL: &str = "https://github.com/jiixyj/libebur128.git";
const LIBEBUR128_GIT_TAG: &str = "v1.2.4";

const LIBEBUR128_BASE_NAME: &str = "ebur128";

fn main() {

	use std::env;
	
	let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
	let out_dir = env::var("OUT_DIR").unwrap();
	let out_path =  Path::new(out_dir.as_str());
	let c_src_dir = "c-libebur128-src-".to_owned().add(LIBEBUR128_GIT_TAG);
	let c_src_path = Path::new(&c_src_dir);
	let c_libebur128_src_path = out_path.join(c_src_path);

	if c_libebur128_src_path.exists() {
		println!("INFO: libebur128 C source exists in {}, skipping clone", c_libebur128_src_path.to_str().unwrap());
	} else {
		// clone source repository
		println!("INFO: cloning {} into {}...", LIBEBUR128_GIT_URL, c_libebur128_src_path.to_str().unwrap());
		let repo = Repository::clone(LIBEBUR128_GIT_URL, c_libebur128_src_path.as_path()) 
		.expect(("failed to clone: ".to_owned() + LIBEBUR128_GIT_URL).as_str());

		// build revision object that we need for checkout
		// NOTE: to get master, we'd reference 'refs/heads/master' here
		let revname = String::from("refs/tags/").add(LIBEBUR128_GIT_TAG);
		let revision = repo.revparse_single(&revname)
		.expect("ERROR:cannot parse checkout revision");

		// checkout selected revision (tag)
		repo.checkout_tree(&revision, None)
		.expect("ERROR:checkout failed");

		println!("INFO: done.");
	}

	let dst = cmake::build(c_libebur128_src_path.to_str().unwrap());
	let libpath = dst.join("lib");

	println!("INFO: cmake wrote library to {}", libpath.display());

	let mut libname = LIBEBUR128_BASE_NAME.to_owned();
	if target_os.eq("windows") {
		// on windows, the built static library has a suffix
		libname.push_str("_static");
	}
	println!("cargo:rustc-link-search=native={}", libpath.display());
	println!("cargo:rustc-link-lib=static={}", libname);

}

