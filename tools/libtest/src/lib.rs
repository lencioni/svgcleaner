extern crate walkdir;
extern crate crypto;

use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs;
use std::path;
use std::process::Command;

use crypto::digest::Digest;

pub use walkdir::{WalkDir, WalkDirIterator};

pub fn compare_imgs(work_dir: &str, path1: &str, path2: &str, path_diff: &str) -> Option<u32> {
    // compare -metric AE -fuzz 1% foo.png foo2.png diff.png
    let res = Command::new("compare").current_dir(work_dir)
                .arg("-metric").arg("AE")
                .arg("-fuzz").arg("1%")
                .arg(path1).arg(path2).arg(path_diff)
                .output();
    match res {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stderr);
            // println!("Compare output: {}", s);
            let num = s.parse::<u32>();
            match num {
                Ok(n) => return Some(n),
                Err(_) => {
                    println!("{:?}", s);
                    return None;
                },
            }
        },
        Err(e) => {
            println!("{:?}", e);
            return None;
        },
    }
}

pub fn is_svg_file(p: &Result<fs::DirEntry, io::Error>) -> bool {
    p.as_ref().unwrap().path().extension().unwrap().to_str().unwrap() == "svg"
}

pub fn get_img_size(png_path: &str) -> (u32, u32) {
    let o = Command::new("file").arg(png_path).output();
    match o {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            let ss = s.split(',').nth(1).unwrap().trim();
            let mut sss = ss.split(" x ");
            let w = sss.next().unwrap().parse::<u32>().unwrap();
            let h = sss.next().unwrap().parse::<u32>().unwrap();

            return (w, h);
        }
        Err(e) => {
            println!("{:?}", e);
            return (0, 0);
        }
    }
}

pub fn gen_png_path(svg_path: &str, suffix: &str) -> String {
    let mut png_path = svg_path.to_owned();
    let l = png_path.len() - 4;
    png_path.truncate(l);
    png_path.push_str(suffix);
    png_path.push_str(".png");
    png_path
}

pub fn create_dir<P: AsRef<path::Path>>(path: P) {
    match fs::create_dir(path) {
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {},
                _ => {
                    println!("{:?}", e.kind());
                    return;
                },
            }
        },
        Ok(_) => {},
    }
}

pub fn render_svg(render: &str, svg_path: &str, png_path: &str) -> bool {
    // Link with custom QtWebKit and not with one from Qt package.
    // This flag is harmless if there is no QtWebKit.
    let res = Command::new(render)
                .env("LD_LIBRARY_PATH", "/usr/local/lib64")
                .arg(svg_path).arg(png_path).arg("512").output();

    match res {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stderr).into_owned();

            // remove unneeded lines
            let s2 = s.split('\n').filter(|x| {
                   !x.find("QPainter::restore: Unbalanced save/restore").is_some()
                && !x.find("QPainter::end: Painter ended with").is_some()
                && !x.find("libGL error:").is_some()
                && !x.find("Image of format '' blocked").is_some()
            }).collect::<Vec<&str>>().join("\n");

            if !s2.is_empty() {
                println!("Render err:\n{}", s.trim());
                return false;
            } else {
                return true;
            }
        },
        Err(e) => {
            println!("{:?}", e);
            return false;
        },
    }
}

pub fn is_svg(entry: &walkdir::DirEntry) -> bool {
    if entry.file_type().is_file() {
        let flag = match entry.path().extension() {
            Some(e) => e.to_str().unwrap() == "svg",
            None => false,
        };
        flag
    } else {
        true
    }
}

pub fn file_md5sum(path: &str) -> String {
    let mut sh = crypto::md5::Md5::new();
    let d = load_file(path);
    sh.input(&d);
    sh.result_str()
}

pub fn load_file(path: &str) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();

    let length = file.metadata().unwrap().len() as usize;

    let mut v = Vec::with_capacity(length + 1);
    file.read_to_end(&mut v).unwrap();

    v
}
