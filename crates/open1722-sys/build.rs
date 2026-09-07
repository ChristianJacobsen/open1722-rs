use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const VENDOR: &str = "vendor/open1722";

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor = manifest_dir.join(VENDOR);

    if !vendor.join("include").exists() {
        panic!(
            "vendored Open1722 not found at {}; did you run `git submodule update --init`?",
            vendor.display()
        );
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let staged = stage_and_patch(&vendor, &out_dir);
    let include = staged.join("include");
    let src = staged.join("src/avtp");

    compile(&src, &include);
    generate_bindings(&include, &out_dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", vendor.display());
}

/// Copies the vendored sources into `OUT_DIR/open1722`, applies upstream
/// fixes that are safe in normal use (single header per TU) but trip bindgen
/// when every header is pulled into one translation unit, and stages the
/// `SharedExports.c` companion export unit.
///
/// TODO: upstream these identifier-collision fixes to COVESA/Open1722.
fn stage_and_patch(vendor: &Path, out_dir: &Path) -> PathBuf {
    let staged = out_dir.join("open1722");
    if staged.exists() {
        fs::remove_dir_all(&staged).expect("clean staged dir");
    }
    copy_tree(&vendor.join("include"), &staged.join("include"));
    copy_tree(&vendor.join("src"), &staged.join("src"));

    // Crf.h:43 has a wrong struct tag (`struct Avtp_Cvf`) that collides with
    // the genuine struct in Cvf.h.
    patch(
        &staged.join("include/avtp/Crf.h"),
        "typedef struct Avtp_Cvf {",
        "typedef struct Avtp_Crf {",
    );

    stage_shared_exports(&staged);

    staged
}

/// Upstream's `src/avtp/export/InlineExports.c` forces external definitions
/// of every format-header accessor, but keeps the shared dependency headers
/// (Byteorder, AcfCommon) in static-inline mode, so their functions get no
/// exported symbols. Stage a companion export unit that covers them, so
/// every function the bindings declare also resolves at link time.
///
/// The macro must be defined empty before any Open1722 header is included:
/// `Inline.h` only applies its `static inline` default when `OPEN1722_INLINE`
/// is undefined at that point.
fn stage_shared_exports(staged: &Path) {
    fs::write(
        staged.join("src/avtp/export/SharedExports.c"),
        concat!(
            "#include <string.h>\n",
            "\n",
            "#undef OPEN1722_INLINE\n",
            "#define OPEN1722_INLINE\n",
            "\n",
            "#include \"avtp/Byteorder.h\"\n",
            "#include \"avtp/acf/AcfCommon.h\"\n",
        ),
    )
    .expect("write SharedExports.c");
}

fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create staged dir");
    for entry in fs::read_dir(src).expect("read source tree") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}

fn patch(file: &Path, from: &str, to: &str) {
    patch_all(file, &[(from, to)]);
}

fn patch_all(file: &Path, replacements: &[(&str, &str)]) {
    let mut contents = fs::read_to_string(file).expect("read patch target");
    for (from, to) in replacements {
        assert!(
            contents.contains(from),
            "expected to find `{from}` in {}",
            file.display()
        );
        contents = contents.replace(from, to);
    }
    fs::write(file, contents).expect("write patched file");
}

fn compile(src: &Path, include: &Path) {
    let sources = collect_files(src, "c");
    cc::Build::new()
        .files(&sources)
        .include(include)
        .std("c99")
        .warnings(false)
        .compile("open1722");
}

fn generate_bindings(include: &Path, out_dir: &Path) {
    let headers = collect_files(&include.join("avtp"), "h");

    // The upstream headers define all getters/setters as `static inline`
    // through the OPEN1722_INLINE macro (see upstream docs/INLINE.md).
    // Defining the macro empty on the preprocessor command line turns every
    // accessor into a plain external declaration, so bindgen emits direct
    // `extern "C"` bindings for it. The matching symbols are exported by
    // upstream's InlineExports.c (format headers) and by the staged
    // SharedExports.c (shared dependency headers).
    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include.display()))
        .clang_arg("-DOPEN1722_INLINE=")
        .use_core()
        .derive_default(true)
        .derive_copy(true)
        .derive_debug(true)
        .layout_tests(true)
        .allowlist_type("Avtp_.*")
        .allowlist_function("Avtp_.*")
        .allowlist_var("AVTP_.*")
        .blocklist_function("avtp_pdu_get")
        .blocklist_function("avtp_pdu_set")
        .blocklist_type("avtp_common_pdu")
        .blocklist_type("avtp_stream_pdu")
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        });

    for header in &headers {
        builder = builder.header(header.to_string_lossy().into_owned());
    }

    let bindings = builder.generate().expect("bindgen failed");
    let out = out_dir.join("bindings.rs");
    bindings.write_to_file(out).expect("write bindings.rs");
}

fn collect_files(root: &Path, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("walk source tree") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().map(|e| e == ext).unwrap_or(false) {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}
