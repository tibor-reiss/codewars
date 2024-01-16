#[derive(Debug)]
pub enum VMError {
    InvalidVersion, // for the `from_version` function
    NoHistory,      // for the `rollback` method
}

pub struct VersionManager {
    major: u32,
    minor: u32,
    patch: u32,
    history: Vec<String>,
}

impl VersionManager {
    
    pub fn new() -> Self {
        VersionManager{major: 0, minor: 0, patch: 1, history: vec![]}
    }
    
    pub fn from_version(version: &str) -> Result<Self, VMError> {
        if version == "" {return Ok(VersionManager::new())}
        
        let mut major: u32 = 0;
        let mut minor: u32 = 0;
        let mut patch: u32 = 0;
        
        for (idx, value) in version.split(".").enumerate() {
            match idx {
                0 => match value.parse::<u32>() {
                    Ok(v) => major = v,
                    Err(_) => return Err(VMError::InvalidVersion),
                }
                1 => match value.parse::<u32>() {
                    Ok(v) => minor = v,
                    Err(_) => return Err(VMError::InvalidVersion),
                }
                2 => match value.parse::<u32>() {
                    Ok(v) => patch = v,
                    Err(_) => return Err(VMError::InvalidVersion),
                }
                _ => break,
            }
        }
        
        Ok(VersionManager{major, minor, patch, history: vec![]})
    }
    
    pub fn major(&mut self) -> &mut Self {
        self.history.push(self.release());
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self
    }
    
    pub fn minor(&mut self) -> &mut Self {
        self.history.push(self.release());
        self.minor += 1;
        self.patch = 0;
        self
    }
    
    pub fn patch(&mut self) -> &mut Self {
        self.history.push(self.release());
        self.patch += 1;
        self
    }
    
    pub fn rollback(&mut self) -> Result<&mut Self, VMError> {
        match self.history.pop() {
            None => Err(VMError::NoHistory),
            Some(previous_version_as_sring) => {
                let temp = VersionManager::from_version(&previous_version_as_sring).unwrap();
                self.major = temp.major;
                self.minor = temp.minor;
                self.patch = temp.patch;
                Ok(self)
            }
        }
    }
    
    pub fn release(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::{VersionManager, VMError};

    #[test]
    fn default_constructor() {
        assert_eq!(VersionManager::new().release(), "0.0.1");
    }

    #[test]
    fn from_valid_version() {
        assert_eq!(VersionManager::from_version("").unwrap().release(), "0.0.1");
        assert_eq!(VersionManager::from_version("10").unwrap().release(), "10.0.0");
        assert_eq!(VersionManager::from_version("1.1").unwrap().release(), "1.1.0");
        assert_eq!(VersionManager::from_version("1.2.3").unwrap().release(), "1.2.3");
        assert_eq!(VersionManager::from_version("1.2.3.4").unwrap().release(), "1.2.3");
        assert_eq!(VersionManager::from_version("1.2.3.d").unwrap().release(), "1.2.3");
    }

    #[test]
    fn from_invalid_version() {
        assert!(matches!(VersionManager::from_version("1.2.c"), Err(VMError::InvalidVersion)));
        assert!(matches!(VersionManager::from_version("a.b.c"), Err(VMError::InvalidVersion)));
    }

   
    #[test]
    fn major_release() {
        assert_eq!(VersionManager::new().major().release(), "1.0.0");
        assert_eq!(VersionManager::from_version("1.2.3").unwrap().major().release(), "2.0.0");
        assert_eq!(VersionManager::from_version("2.2.3.4.5").unwrap().major().major().release(), "4.0.0");
    }

    #[test]
    fn minor_release() {
        assert_eq!(VersionManager::new().minor().release(), "0.1.0");
        assert_eq!(VersionManager::from_version("1.2.3").unwrap().minor().release(), "1.3.0");
        assert_eq!(VersionManager::from_version("1").unwrap().minor().release(), "1.1.0");
    }

    #[test]
    fn patch_release() {
        assert_eq!(VersionManager::new().patch().release(), "0.0.2");
        assert_eq!(VersionManager::from_version("1.2.3").unwrap().patch().release(), "1.2.4");
        assert_eq!(VersionManager::from_version("4").unwrap().patch().patch().release(), "4.0.2");
    }


    #[test]
    fn valid_rollbacks() {
        assert_eq!(VersionManager::new().major().rollback().unwrap().release(), "0.0.1");
        assert_eq!(VersionManager::new().minor().rollback().unwrap().release(), "0.0.1");
        assert_eq!(VersionManager::new().patch().rollback().unwrap().release(), "0.0.1");
        assert_eq!(VersionManager::new().major().patch().rollback().unwrap().release(), "1.0.0");
        assert_eq!(VersionManager::new().major().patch().rollback().unwrap().major().rollback().unwrap().release(), "1.0.0");
    }

    #[test]
    fn invalid_rollbacks() {
        assert!(matches!(VersionManager::new().rollback(), Err(VMError::NoHistory)));
        assert!(matches!(VersionManager::new().major().rollback()
            // double rollback
            .and_then(|vm| vm.rollback()),
            Err(VMError::NoHistory)
        ));
    }


    #[test]
    fn mixed_without_rollback() {
        assert_eq!(VersionManager::new().major().minor().patch().major().patch().release(), "2.0.1");
    }

    #[test]
    fn mixed_with_rollback() {
        assert_eq!(VersionManager::new().major().minor().patch().major().patch()
            .rollback()
                // first rollback should be Ok()
                .unwrap_or_else(|e| panic!("VersionManager::new().major().minor().patch().major().patch().rollback() should be Ok(), instead got Err({e:?})"))
            .rollback()
                // second rollback should also be Ok()
                .unwrap_or_else(|e| panic!("VersionManager::new().major().minor().patch().major()\
                                        .patch().rollback()?.rollback() should be Ok(), instead got Err({e:?})"))
            .patch()
            .release(),
            "1.1.2",
            "\nVersionManager::new().major().minor().patch().major().patch()\
                    .rollback()?.rollback()?.patch().release() should return \"1.1.2.\""
        );
    }
}
