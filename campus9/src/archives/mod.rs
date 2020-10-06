pub use packages::{
    patches::{
        Id,
        Patch,
        decompress::Oodle,
        files::FileInfo,
    },
    Package,
};
use std::{
    collections::HashMap,
    fs::read_dir,
    io::Result,
    path::{
        Path,
        PathBuf,
    },
};

pub mod packages;

fn create_package_map(package_dir: PathBuf) -> HashMap<Id, Package> {
    let mut map: HashMap<Id, Package> = HashMap::new();

    let mut paths = read_dir(package_dir).expect(
        "Could not read directory!"
    ).map(
        |res| res.map(|e| e.path())
    ).collect::<Result<Vec<_>>>().expect("Path collection failed!");
    paths.sort();
    paths.reverse();

    for path in paths {
        let file_info: FileInfo = FileInfo::from_path(path);
        let package: &mut Package;
        if map.contains_key(&file_info.header.package_id) {
            package = map.get_mut(
                &file_info.header.package_id
            ).expect("Failed to fetch package");
        } else {
            map.insert(
                file_info.header.package_id,
                Package::new(file_info.header.package_id, file_info.description.clone())
            );
            package = map.get_mut(&file_info.header.package_id).expect("Failed to fetch package");
        }
        let id = file_info.patch_id;
        let mut new_patch = Patch::new(file_info);
        if package.patches.is_empty() {
            new_patch.load();
            package.patches.insert(id, new_patch);
        } else {
            package.patches.entry(
                id
            ).or_insert_with(|| new_patch);
        }
    }

    map
}

pub struct Archives {
    pub base_dir: PathBuf,
    pub oodle_path: PathBuf,
    pub package_dir: PathBuf,
    pub packages: HashMap<Id, Package>,
}

impl Archives {
    pub fn new(path: String) -> Self {
        let base_dir = Path::new(&path).to_path_buf();
        let mut oodle_path: PathBuf = base_dir.clone();
        oodle_path.push("bin");
        oodle_path.push("x64");
        oodle_path.push("oo2core_3_win64");
        oodle_path.set_extension("dll");

        let mut package_dir: PathBuf = base_dir.clone();
        package_dir.push("packages");

       Oodle::init(oodle_path.to_str().unwrap());

        let packages: HashMap<Id, Package> = create_package_map(package_dir.clone());
        Self {
            base_dir,
            oodle_path,
            package_dir,
            packages,
        }
    }
}

