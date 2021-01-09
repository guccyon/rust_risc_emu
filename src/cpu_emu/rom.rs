#[derive(Debug)]
pub struct Rom {
    data: Vec<u16>,
}

impl Rom {
    pub fn new(data: Vec<u16>) -> Self {
        Self { data }
    }

    pub fn read(&self, index: usize) -> Result<u16, String> {
        if self.data.len() > index {
            Ok(self.data[index])
        } else {
            Err("Unexpected EOF".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let rom = Rom::new(vec![0x0010, 0x0012]);
        assert_eq!(rom.read(0), Ok(0x0010));
    }
}
