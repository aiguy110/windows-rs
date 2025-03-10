use rayon::prelude::*;
use std::collections::*;
use std::io::prelude::*;

/// Namespaces to exclude from code generation for the `windows-sys` crate.
const EXCLUDE_NAMESPACES: [&str; 28] = [
    "Windows.Win32.AI.MachineLearning",
    "Windows.Win32.Graphics.CompositionSwapchain",
    "Windows.Win32.Graphics.Direct2D",
    "Windows.Win32.Graphics.Direct3D",
    "Windows.Win32.Graphics.Direct3D10",
    "Windows.Win32.Graphics.Direct3D11",
    "Windows.Win32.Graphics.Direct3D11on12",
    "Windows.Win32.Graphics.Direct3D12",
    "Windows.Win32.Graphics.Direct3D9",
    "Windows.Win32.Graphics.Direct3D9on12",
    "Windows.Win32.Graphics.DirectComposition",
    "Windows.Win32.Graphics.DirectDraw",
    "Windows.Win32.Graphics.DirectManipulation",
    "Windows.Win32.Graphics.DirectWrite",
    "Windows.Win32.Graphics.DXCore",
    "Windows.Win32.Graphics.Dxgi",
    "Windows.Win32.Graphics.Imaging",
    "Windows.Win32.Interop",
    "Windows.Win32.Media.Audio.DirectSound",
    "Windows.Win32.Media.DirectShow",
    "Windows.Win32.Media.MediaFoundation",
    "Windows.Win32.Media.PictureAcquisition",
    "Windows.Win32.System.Diagnostics.Debug.WebApp",
    "Windows.Win32.System.SideShow",
    "Windows.Win32.System.TransactionServer",
    "Windows.Win32.System.WinRT",
    "Windows.Win32.Web",
    "Windows.Win32.UI.Xaml",
];

fn main() {
    let mut rustfmt = true;
    let mut expect_namespace = false;
    let mut namespace = String::new();
    for arg in std::env::args() {
        match arg.as_str() {
            "-p" => rustfmt = false,
            "-n" => expect_namespace = true,
            _ => {
                if expect_namespace {
                    namespace = arg;
                }
            }
        }
    }
    let mut output = std::path::PathBuf::from("crates/libs/sys/src/Windows");
    if namespace.is_empty() {
        let _ = std::fs::remove_dir_all(&output);
    }
    output.pop();
    let files = metadata::reader::File::with_default(&[]).unwrap();
    let reader = &metadata::reader::Reader::new(&files);
    if !namespace.is_empty() {
        let tree = reader.tree(&namespace, &[]).expect("Namespace not found");
        gen_tree(reader, &output, &tree, rustfmt);
        return;
    }
    let win32 = reader.tree("Windows.Win32", &EXCLUDE_NAMESPACES).expect("`Windows.Win32` namespace not found");
    let root = metadata::reader::Tree { namespace: "Windows", nested: BTreeMap::from([("Win32", win32)]) };
    let trees = root.flatten();
    trees.par_iter().for_each(|tree| gen_tree(reader, &output, tree, rustfmt));
    output.pop();
    output.push("Cargo.toml");
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows-sys"
version = "0.45.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
readme = "../../../docs/readme.md"
rust-version = "1.48"

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []
all-features = true

[target.'cfg(not(windows_raw_dylib))'.dependencies]
windows-targets = { path = "../targets",  version = "0.42.1" }

[features]
default = []
"#
        .as_bytes(),
    )
    .unwrap();

    // Skip the root Windows tree while writing features
    for tree in trees.iter().skip(1) {
        let feature = tree.namespace[root.namespace.len() + 1..].replace('.', "_");

        if let Some(pos) = feature.rfind('_') {
            let dependency = &feature[..pos];

            file.write_all(format!("{feature} = [\"{dependency}\"]\n").as_bytes()).unwrap();
        } else {
            file.write_all(format!("{feature} = []\n").as_bytes()).unwrap();
        }
    }
}

fn gen_tree(reader: &metadata::reader::Reader, output: &std::path::Path, tree: &metadata::reader::Tree, rustfmt: bool) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.sys = true;
    gen.cfg = true;
    gen.doc = true;
    let mut tokens = bindgen::namespace(&gen, tree);
    lib::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();
}
