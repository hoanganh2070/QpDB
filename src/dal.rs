use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, Seek, SeekFrom, Write},
};
use std::io::Read;

pub struct DataAccessLayer {
    file: Option<File>,
    page_size: i32,
}

pub struct Page {
    num: u64,
    data: Vec<u8>,
}

fn new_dal(path: String, page_size: i32) -> DataAccessLayer {
    let file = OpenOptions::new()
        .read(true) // Quyền đọc
        .write(true) // Quyền ghi
        .create(true) // Nếu tệp không tồn tại, tạo tệp mới
        .open(path) // Mở tệp theo đường dẫn
        .expect("Failed to open the file");
    return DataAccessLayer {
        file: Some(file),
        page_size,
    };
}

impl DataAccessLayer {
    pub fn close(&mut self) -> io::Result<()> {
        if let Some(file) = self.file.take() {
            // Nếu tệp tồn tại, chúng ta sẽ "lấy" file ra và đóng nó
            match file.sync_all() {
                Ok(_) => {
                    self.file = None; // Đặt lại file thành None
                    Ok(())
                }
                Err(err) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("could not close file: {}", err),
                )),
            }
        } else {
            Ok(())
        }
    }

    pub fn allocate_empty_page(&mut self) -> Page {
        Page {
            data: vec![0; self.page_size as usize],
            num: 0,
        }
    }

    pub fn read_page(&mut self, page_num: i32) -> Result<Page, io::Error> {
        let mut p = self.allocate_empty_page();

        let offsest = (page_num * self.page_size) as u64;

        let mut reader = BufReader::new(self.file.as_ref().expect("File is not open"));

        reader.seek(SeekFrom::Start(offsest))?;


        reader.read(&mut p.data)?;

        Ok(p)

    }


    pub fn write_page(&mut self, p: &Page) -> io::Result<()> {
        let offset = p.num * self.page_size as u64;

        if let Some(file) = self.file.as_mut() {
            file.seek(SeekFrom::Start(offset))?;
            file.write_all(&p.data)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "File is not open"));
        }


        Ok(()) // Trả về
    }
}
