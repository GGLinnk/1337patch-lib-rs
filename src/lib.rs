use std::fs::File;
use std::io::{self, BufRead, Seek};

pub trait SeekableBufRead: BufRead + Seek {}
impl<R: BufRead + Seek> SeekableBufRead for R {}

/// Enum representing the different errors that can occur when reading a patch file.
/// 
/// See [Variants](#variants) for variants and their meaning.
pub enum PatchFileError {
    /// When the radix or any other conversion fails.
    /// 
    /// Occurs if the values are not in hex.
    /// 
    /// This encapsulates [std::num::ParseIntError].
    ConvertionError(std::num::ParseIntError),
    /// When the file cannot be read.
    /// 
    /// Occurs if the file cannot be read.<br/>
    /// If this happens, the file is probably not accessible, does not exist or insufficient permissions is given to read the file.
    /// 
    /// This encapsulates [std::io::Error].
    ReadError(std::io::Error),
    /// When the file is not in the right format.
    /// 
    /// Occurs if the file is not in the right format.<br/>
    /// Can bee too long, too short values, lines not in the right format, and so on.
    WrongFormat,
}

/// Implement [std::fmt::Debug] trait for [PatchFileError]
impl std::fmt::Debug for PatchFileError {
    /// This is the implementation of [std::fmt::Debug::fmt] for [PatchFileError].
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PatchFileError::ConvertionError(e) => write!(f, "ConvertionError: {}", e),
            PatchFileError::ReadError(e) => write!(f, "ReadError: {}", e),
            PatchFileError::WrongFormat => write!(f, "Error : WrongFormat: The file/buffer data structure is invalid!"),
        }
    }
}

/// Implement [PartialEq] for [PatchFileError]
impl PartialEq for PatchFileError {
    /// This is the implementation of [PartialEq::eq] for [PatchFileError].
    fn eq(&self, other: &Self) -> bool {
        match self {
            PatchFileError::ConvertionError(error_self) => {
                match other {
                    PatchFileError::ConvertionError(error_other) => error_self.kind() == error_other.kind(),
                    _ => false,
                }
            },
            PatchFileError::ReadError(error_self) => {
                match other {
                    PatchFileError::ReadError(error_other) => error_self.kind() == error_other.kind(),
                    _ => false,
                }
            },
            PatchFileError::WrongFormat => {
                match other {
                    PatchFileError::WrongFormat => true,
                    _ => false,
                }
            },
        }
    }
}

/// From [std::num::ParseIntError] to [PatchFileError]
impl From<std::num::ParseIntError> for PatchFileError {
    /// This is the implementation for [std::num::ParseIntError] to [PatchFileError] conversion.
    fn from(error: std::num::ParseIntError) -> Self {
        PatchFileError::ConvertionError(error)
    }
}

/// From [std::io::Error] to [PatchFileError]
impl From<std::io::Error> for PatchFileError {
    /// This is the implementation for [std::io::Error] to [PatchFileError] conversion.
    fn from(error: std::io::Error) -> Self {
        PatchFileError::ReadError(error)
    }
}

/// This is used to create representation of a patch.
/// 
/// A patch is in the following format:<br/>
/// [``TargetAddress``](HexPatch::target_address):[``Old``](HexPatch::old)->[``New``](HexPatch::new) all in HEX in a TXT file.<br/>
/// Target address is always 16 hex digits long, old value and new value are always 2 hex digits long.
/// 
/// Example:
/// ```text
/// 0000000000AF0200:13->37
/// ```
#[derive(Debug)]
pub struct HexPatch {
    /// Target address of the patch.
    pub target_address: u64,
    /// Old value of the patch.
    pub old: u8,
    /// New value of the patch.
    pub new: u8,
}

/// Implementation of [HexPatch]
impl HexPatch {
    /// This is the constructor of [HexPatch]
    /// 
    /// It takes a [target address](HexPatch::target_address), [old value](HexPatch::old) and [new value](HexPatch::new) and returns a [HexPatch].
    /// 
    /// # Arguments
    /// - ``address`` - The target address of the patch.
    /// - ``old`` - The old value of the patch.
    /// - ``new`` - The new value of the patch.
    /// 
    /// # Example
    /// ```rust
    /// use lib1337patch::HexPatch;
    /// 
    /// let patch = HexPatch::new(0x0000000000AF0200, 0x13, 0x37);
    /// ```
    pub fn new(address: u64, old: u8, new: u8) -> HexPatch {
        HexPatch {
            target_address: address,
            old,
            new,
        }
    }
}

/// Implement [PartialEq] for [HexPatch]
impl PartialEq for HexPatch {
    /// This is the implementation of [PartialEq::eq] for [HexPatch].
    fn eq(&self, other: &Self) -> bool {
        self.target_address == other.target_address &&
        self.old == other.old &&
        self.new == other.new
    }
}

/// This is used to create representation of the patch file.
/// 
/// Path files are in the following format:<br/>
/// The first line start with ``>`` and followed by the target file name.
/// 
/// The rest of the lines are patches in the following format:<br/>
/// [``TargetAddress``](HexPatch::target_address):[``Old``](HexPatch::old)->[``New``](HexPatch::new) all in HEX in a TXT file.
/// 
/// Target address is always 16 hex digits long, old value and new value are always 2 hex digits long.
/// 
/// # Example
/// ```text
/// >test.exe
/// 0000000000AF0200:13->37
/// 0000000000AF0206:37->37
/// ```
/// 
/// Patches are stored in a vector of [HexPatch].
#[derive(Debug)]
pub struct F1337Patch {
    /// Target file name. Extracted from the first line of the patch file.
    pub target_filename: String,
    /// Vector of patches. Builded from extracted data from the rest of the lines of the patch file.
    pub patches: Vec<HexPatch>,
}

impl F1337Patch {
    /// This creates an new empty [F1337Patch].
    /// 
    /// # Example
    /// ```rust
    /// use lib1337patch::F1337Patch;
    /// use lib1337patch::HexPatch;
    /// 
    /// let mut f1337patch = F1337Patch::new("test.exe".to_string());
    /// let patch = HexPatch::new(0x0000000000AF0200, 0x13, 0x37);
    /// 
    /// f1337patch.add_patch(patch);
    /// 
    /// println!("File name : {}", f1337patch.target_filename);
    /// println!("Patches : {:?}", f1337patch.patches);
    /// ```
    pub fn new(target_filename: String) -> Self {
        F1337Patch {
            target_filename,
            patches: Vec::new(),
        }
    }

    /// This adds a patch to the [F1337Patch].
    /// 
    /// To create a [HexPatch], use [HexPatch::new].
    /// 
    /// # Arguments
    /// - ``patch``: A [HexPatch]. Can be created with [HexPatch::new].
    /// 
    /// # Example
    /// ```rust
    /// use lib1337patch::F1337Patch;
    /// use lib1337patch::HexPatch;
    /// 
    /// let mut f1337patch = F1337Patch::new("test.exe".to_string());
    /// 
    /// f1337patch.add_patch(HexPatch::new(0x0000000000AF0200, 0x13, 0x37));
    /// println!("File name : {}", f1337patch.target_filename);
    /// ```
    pub fn add_patch(&mut self, patch: HexPatch) {
        self.patches.push(patch);
    }

    /// This creates a new [F1337Patch] from a [File].
    /// 
    /// It takes a mutable reference to a [File] and returns a [Result] of [F1337Patch] or [PatchFileError].
    /// 
    /// This function is a wrapper for [F1337Patch::from_bufreader].
    /// 
    /// # Arguments
    /// - ``patchfile``: A mutable reference to a [File].
    /// 
    /// # Returns
    /// - Result of [F1337Patch] or [PatchFileError].
    /// 
    /// # Errors
    /// - [PatchFileError::ConvertionError] if the file contains invalid hex values. Contains [std::num::ParseIntError].
    /// - [PatchFileError::ReadError] if the file can't be read. Contains [std::io::Error].
    /// - [PatchFileError::WrongFormat] if the file is not in the right format.
    /// 
    /// # Example
    /// ```rust,no_run
    /// use lib1337patch::F1337Patch;
    /// use std::fs::File;
    /// 
    /// let mut patchfile = File::open("test.txt").unwrap();
    /// 
    /// let patch = F1337Patch::from_patchfile(&mut patchfile).unwrap();
    /// ```
    pub fn from_patchfile(patchfile: &File) -> Result<F1337Patch, PatchFileError> {
        Self::from_bufreader(&mut io::BufReader::new(patchfile))
    }

    /// This creates a new [F1337Patch] from a [std::io::BufReader].
    /// 
    /// It takes a mutable reference to a [File] and returns a [Result] of [F1337Patch] or [PatchFileError].
    /// 
    /// # Arguments
    /// - ``bufreader``: A mutable reference to a any BufReader that implements Seek.
    /// 
    /// # Returns
    /// - Result of [F1337Patch] or [PatchFileError].
    /// 
    /// # Errors
    /// - [PatchFileError::ConvertionError] if the file contains invalid hex values. Contains [std::num::ParseIntError].
    /// - [PatchFileError::ReadError] if the file can't be read. Contains [std::io::Error].
    /// - [PatchFileError::WrongFormat] if the file is not in the right format.
    /// 
    /// # Example
    /// ```rust,no_run
    /// use lib1337patch::F1337Patch;
    /// use std::fs::File;
    /// 
    /// let mut patchfile = File::open("test.txt").unwrap();
    /// let patch = F1337Patch::from_patchfile(&patchfile).unwrap();
    /// ```
    /// 
    /// # Note
    /// See [F1337Patch] for more information about the file format.
    pub fn from_bufreader<R: SeekableBufRead>(bufreader: &mut R) -> Result<F1337Patch, PatchFileError> {
        let mut f1337patch: F1337Patch;
        let mut first_line = String::new();
        
        bufreader.seek(io::SeekFrom::Start(0)).unwrap();
        bufreader.read_line(&mut first_line)?;
        f1337patch = F1337Patch::new(Self::get_filename(first_line)?);

        for result in bufreader.lines() {
            let line = result?;

            Self::check_patch_line_format(&line)?;
            f1337patch.patches.push(Self::get_hex_patch_from_line(&line)?);
        }
        
        Ok(f1337patch)
    }

    /// This function checks that patch line is in the right format.
    /// 
    /// # Arguments
    /// - ``line``: A mutable reference to a [String].
    /// 
    /// # Returns
    /// - [Result] of [()] or [PatchFileError].
    /// 
    /// # Errors
    /// - [PatchFileError::WrongFormat] if the line is not in the right format.
    /// 
    /// # Example
    /// ```rust
    /// use lib1337patch::F1337Patch;
    /// 
    /// let line = "0000000000AF0200:13->37".to_string();
    /// F1337Patch::check_patch_line_format(&line).unwrap();
    /// ```
    /// 
    /// # Note
    /// See [F1337Patch] for more information about the file format.
    pub fn check_patch_line_format(line: &String) -> Result<(), PatchFileError> {
        // Check if line is 23 characters long.
        if line.len() != 23 {
            return Err(PatchFileError::WrongFormat);
        }
        // Check the presence of ":" and "->" in the right place.
        if &line[16..17] != ":" {
            return Err(PatchFileError::WrongFormat);
        }
        if &line[19..21] != "->" {
            return Err(PatchFileError::WrongFormat);
        }
        // Check if address, old an new values are only in hex digits.
        if !line[0..16].chars().all(|c| c.is_digit(16)) {
            return Err(PatchFileError::WrongFormat);
        }
        if !line[17..19].chars().all(|c| c.is_digit(16)) {
            return Err(PatchFileError::WrongFormat);
        }
        if !line[21..23].chars().all(|c| c.is_digit(16)) {
            return Err(PatchFileError::WrongFormat);
        }
        Ok(())
    }

    /// This function extracts patch from given line.
    /// 
    /// # Arguments
    /// - ``line``: A reference to a [String].
    /// 
    /// # Returns
    /// - [Result] of [HexPatch] or [PatchFileError].
    /// 
    /// # Errors
    /// - [PatchFileError::ConvertionError] if the file contains invalid hex values. Contains [std::num::ParseIntError].
    /// 
    /// # Example
    /// ```rust
    /// use lib1337patch::F1337Patch;
    /// 
    /// let line = "0000000000AF0200:13->37".to_string();
    /// let patch = F1337Patch::get_hex_patch_from_line(&line).unwrap();
    /// ```
    pub fn get_hex_patch_from_line(line: &String) -> Result<HexPatch, std::num::ParseIntError> {
        let address = u64::from_str_radix(&line[0..16], 16)?;
        let old = u8::from_str_radix(&line[17..19], 16)?;
        let new = u8::from_str_radix(&line[21..23], 16)?;

        Ok(HexPatch::new(address, old, new))
    }

    /// This function extract filename from the first line of the patch file.
    /// The first line start with ">" and followed by the target file name.
    fn get_filename(first_line: String) -> Result<String, PatchFileError> {
        if !first_line.starts_with('>') {
            return Err(PatchFileError::WrongFormat);
        }
        
        // This returns the filename. Trim the end to remove the \n (and \r\n on windows).
        Ok(first_line[1..].trim_end().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempfile;
    use std::io::Write;
    
        // TODO : Add some fuzzing for [F1337Patch::new] and [F1337Patch::from_filepatch] to test more cases.
        // TODO : Add more fuzzing for [F1337Patch::check_patch_line_format] to test more cases.
    
    #[test]
    fn test_f1337patch_new() {
        let f1337path = F1337Patch::new("test.exe".to_string());

        assert_eq!(f1337path.target_filename, "test.exe");
        assert_eq!(f1337path.patches.len(), 0);
    }

    #[test]
    fn test_f1337patch_from_filepatch() {
        let mut dummy_file = tempfile().unwrap();
        let f1337path: F1337Patch;
        let dummy_patches: Vec<HexPatch>;
        
        writeln!(dummy_file, ">test.exe").unwrap();
        writeln!(dummy_file, "0000000000AF0200:13->37").unwrap();
        writeln!(dummy_file, "0000000000AF0206:37->37").unwrap();

        f1337path = F1337Patch::from_patchfile(&mut dummy_file).unwrap();

        assert_eq!(f1337path.target_filename, "test.exe");
        assert_eq!(f1337path.patches.len(), 2);

        dummy_patches = vec![
            HexPatch::new(0xAF0200, 0x13, 0x37),
            HexPatch::new(0xAF0206, 0x37, 0x37),
        ];

        assert_eq!(dummy_patches, f1337path.patches);

        drop(dummy_file);
    }

    #[test]
    fn test_f1337patch_from_bufreader() {
        let mut dummy_file = tempfile().unwrap();
        let f1337path: F1337Patch;
        let dummy_patches: Vec<HexPatch>;
        
        writeln!(dummy_file, ">test.exe").unwrap();
        writeln!(dummy_file, "0000000000AF0200:13->37").unwrap();
        writeln!(dummy_file, "0000000000AF0206:37->37").unwrap();

        f1337path = F1337Patch::from_bufreader(&mut io::BufReader::new(&dummy_file)).unwrap();

        assert_eq!(f1337path.target_filename, "test.exe");
        assert_eq!(f1337path.patches.len(), 2);

        dummy_patches = vec![
            HexPatch::new(0xAF0200, 0x13, 0x37),
            HexPatch::new(0xAF0206, 0x37, 0x37),
        ];

        assert_eq!(dummy_patches, f1337path.patches);

        drop(dummy_file);
    }

    #[test]
    fn test_check_patch_line_format_wrong_format() {
        let lines = vec![
            "0000000000AF0200:13->3",
            "000000AF0200:13->32",
            "0000000000AF020089:13->3A",
            "0000000000AF0200:13->ZA",
            "0000000000AF02KK:13->3A",
        ];

        for line in lines {
            let wrong_format = F1337Patch::check_patch_line_format(&line.to_string()).unwrap_err();
            assert_eq!(wrong_format, PatchFileError::WrongFormat);
        };
    }

    #[test]
    fn test_get_filename_wrong_format() {
        let wrong_format = F1337Patch::get_filename("test.exe".to_string()).unwrap_err();

        assert_eq!(wrong_format, PatchFileError::WrongFormat);
    }
}