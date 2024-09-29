use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::error::Error;

pub(crate) fn save_file(file: &mut File, path: &str) -> Result<(), Error> {
    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer).map_err(|e| Error::Io(Box::new(e)))?;
    // let mut file = File::create(path).map_err(|e| Error::Io(Box::new(e)))?;
    // file.write_all(&buffer).map_err(|e| Error::Io(Box::new(e)))?;
    todo!("保存文件函数");
    Ok(())
}
