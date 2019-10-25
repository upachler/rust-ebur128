
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
	let out_dir = env::var("OUT_DIR").unwrap();
	let out_path =  Path::new(out_dir.as_str());
	let c_src_dir = "c-libebur128-src-".to_owned().add(LIBEBUR128_GIT_TAG);
	let c_src_path = Path::new(&c_src_dir);
	let c_libebur128_src_path = out_path.join(c_src_path);

	if c_libebur128_src_path.exists() {
		println!("INFO: libebur128 C source exists in {}, skipping clone", c_libebur128_src_path.to_str().unwrap());
	} else {
		println!("INFO: cloning {} into {}...", LIBEBUR128_GIT_URL, c_libebur128_src_path.to_str().unwrap());
		let repo = Repository::clone(LIBEBUR128_GIT_URL, c_libebur128_src_path.as_path()) 
		.expect(("failed to clone: ".to_owned() + LIBEBUR128_GIT_URL).as_str());

		//let revname = "refs/tags/".to_owned().add(LIBEBUR128_GIT_TAG).as_str();
		let revname = "refs/heads/master";
		let revision = repo.revparse_single(revname)
		.expect("ERROR:cannot parse checkout revision");

		repo.checkout_tree(&revision, None)
		.expect("ERROR:checkout failed");

		println!("INFO: done.");
	}
	// FIXME: need to checkout tag, not master

	let dst = cmake::build(c_libebur128_src_path.to_str().unwrap());
	let libpath = dst.join("lib");

	println!("INFO: cmake wrote library to {}", libpath.display());

	let libname = LIBEBUR128_BASE_NAME.to_owned() + "_static";
	println!("cargo:rustc-link-search=native={}", libpath.display());
	println!("cargo:rustc-link-lib=static={}", libname);

}

