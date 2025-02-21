use std::{
    fs::create_dir_all,
    panic::catch_unwind,
    path::{Path, PathBuf},
};

use vglang::svg::{reader::from_svg, writer::to_svg};
#[test]
fn test_spec() {
    // _ = pretty_env_logger::try_init();

    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("spec");

    let spec_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("spec");

    if !spec_dir.exists() {
        create_dir_all(&spec_dir).unwrap();
    }

    let mut svg_files = vec![];

    for entry in std::fs::read_dir(&root_dir).unwrap() {
        let entry = entry.unwrap();

        if !entry.file_type().unwrap().is_dir() {
            continue;
        }

        for image in std::fs::read_dir(&root_dir.join(entry.path())).unwrap() {
            let image = image.unwrap();

            if let Some(extension) = image.path().extension() {
                if extension == "svg" {
                    svg_files.push((
                        root_dir.join(entry.path()).join(image.path()),
                        image.file_name(),
                    ));
                }
            }
        }
    }

    let mut succ = 0;
    let mut faileds = 0;

    for (svg, file_name) in svg_files {
        print!("svg {:?} ... ", file_name);
        let output = spec_dir.join(file_name);

        match catch_unwind(move || test_svg(svg, output)) {
            Ok(_) => {
                println!("ok");
                succ += 1;
            }
            Err(e) => {
                println!("failed");

                faileds += 1;

                eprintln!("{:?}", e);
            }
        }

        // test_svg(svg, output);
    }

    if faileds > 0 {
        panic!("spec result: ok {} passed; {} failed;", succ, faileds);
    } else {
        println!("spec result: ok {} passed; {} failed;", succ, faileds);
    }
}

fn test_svg(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    let opcodes = from_svg(std::fs::read_to_string(&input).unwrap()).unwrap();

    let svg = to_svg(&opcodes).unwrap();

    std::fs::write(output, &svg).unwrap();

    // TODO: fix the escape Character bug of the `xml_dom` crate.
    if input.as_ref().file_name().unwrap() != "rtl_text.svg" {
        assert_eq!(from_svg(svg).unwrap(), opcodes);
    }
}

#[test]
fn test_lyon_logo() {
    // _ = pretty_env_logger::try_init();

    let opcodes = from_svg(include_str!("./lyon.svg")).unwrap();

    let svg = to_svg(&opcodes).unwrap();

    let output_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("spec");

    if !output_dir.exists() {
        create_dir_all(&output_dir).unwrap();
    }

    std::fs::write(output_dir.join("./lyon.svg"), &svg).unwrap();

    assert_eq!(from_svg(svg).unwrap(), opcodes);
}
