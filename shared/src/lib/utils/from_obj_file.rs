use obj::{load_obj, Obj, ObjResult, Vertex};
use std::fs::File;
use std::io::BufReader;

pub fn get_obj_from_file(path: &str) -> ObjResult<Obj<Vertex, u16>> {
    let input = BufReader::new(File::open(path)?);
    println!("get_obj_from_file({:?}): {:?}", path, input);
    let obj: Obj = load_obj(input)?;

    Ok(obj)
}
