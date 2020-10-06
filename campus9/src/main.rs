use crate::archives::Archives;

pub mod archives;

fn main() -> std::io::Result<()> {
    let start = std::time::SystemTime::now();
    let archives = Archives::new(r"C:\Program Files (x86)\Steam\steamapps\common\Destiny 2\".to_string());
    /*for pkg in archives.packages {
        let id = pkg.1.id;
        let count = pkg.1.patches.len();
        println!("0x0{:X} -- {} -- {}", id, count, &pkg.1.description);
    }*/
    println!("Took {:?}ms", std::time::SystemTime::now().duration_since(start).expect("failed").as_millis());
    let pack = (&archives.packages).get(&0x03FF).unwrap();
    let data = pack.extract_entry(*pack.get_entries().get(1).unwrap());
    std::fs::write(r"D:\sample.wem", data).unwrap();
    Ok(())
}
