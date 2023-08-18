use std::fs::File;
use std::io::{self, BufRead, Seek};

/// This represents previous error encountered.
/// 
/// It is used to store the [function](PreviousError::function), [filename](PreviousError::filename), [line](PreviousError::line) and [error](PreviousError::error) of the previous error.
/// 
/// It is used in [PatchFileError] to store the previous error.
pub struct PreviousError {
    /// The function where the error occured.
    pub function: String,
    /// The filename where the error occured.
    pub filename: String,
    /// The line where the error occured.
    pub line:     u32,
    /// The error that occured. Can be any error that implements [std::error::Error]. Boxed.
    pub error:    Box<dyn std::error::Error>,
}

/// Implementaion of [PreviousError].
/// 
/// It implements the following:
/// - [new](PreviousError::new) - Constructor of [PreviousError].
impl PreviousError {
    /// This is the constructor of [PreviousError].
    /// 
    /// It takes a [function](PreviousError::function), [filename](PreviousError::filename), [line](PreviousError::line) and [error](PreviousError::error) and returns a [PreviousError].
    /// 
    /// # Arguments
    /// - ``function`` - The function where the error occured.
    /// - ``filename`` - The filename where the error occured.
    /// - ``line`` - The line where the error occured.
    /// - ``error`` - The error that occured.
    /// 
    /// # Example
    /// ```rust
    /// let error = PreviousError::new("function", "filename", 1, Box::new(std::io::Error::new(std::io::ErrorKind::Other, "error")));
    /// ```
    fn new(function: String, filename: String, line: u32, error: Box<dyn std::error::Error>) -> PreviousError {
        PreviousError {function, filename, line, error}
    }
}

/// Enum representing the different errors that can occur when reading a patch file.
/// 
/// See [Variants](#variants) for variants and their meaning.
/// 
/// [ConvertionError](PatchFileError::ConvertionError) and [ReadError](PatchFileError::ReadError) contain a [PreviousError] that contains the [function](PreviousError::function), [filename](PreviousError::filename), [line](PreviousError::line) and the previous [error](PreviousError::error) that occured.
pub enum PatchFileError {
    /// When the radix or any other conversion fails.
    /// 
    /// Occurs if the values are not in hex.
    /// 
    /// This encapsulates [std::num::ParseIntError](std::num::ParseIntError).
    ConvertionError(PreviousError),
    /// When the file cannot be read.
    /// 
    /// Occurs if the file cannot be read.<br/>
    /// If this happens, the file is probably not accessible, does not exist or insufficient permissions is given to read the file.
    /// 
    /// This encapsulates [std::io::Error](std::io::Error).
    ReadError(PreviousError),
    /// When the file is not in the right format.
    /// 
    /// Occurs if the file is not in the right format.<br/>
    /// Can bee too long, too short values, lines not in the right format, and so on.
    WrongFormat,
}

impl PatchFileError {
    /// This is the constructor of [PatchFileError::ConvertionError]
    /// 
    /// It takes a [function](PreviousError::function), [filename](PreviousError::filename), [line](PreviousError::line) and [error](PreviousError::error) and returns a [PatchFileError::ConvertionError].
    /// 
    /// # Arguments
    /// - ``function`` - The function where the error occured.
    /// - ``filename`` - The filename where the error occured.
    /// - ``line`` - The line where the error occured.
    /// - ``error`` - The error that occured.
    /// 
    /// # Example
    /// ```rust
    /// let error = PatchFileError::new_convertion_error("function", "filename", 1, Box::new(std::io::Error::new(std::io::ErrorKind::Other, "error")));
    /// ```
    fn new_convertion_error(function: &str, filename: &str, line: u32, error: Box<dyn std::error::Error>) -> PatchFileError {
        PatchFileError::ConvertionError(PreviousError::new(function.to_string(), filename.to_string(), line, error))
    }

    /// This is the constructor of [PatchFileError::ReadError]
    /// 
    /// It takes a [function](PreviousError::function), [filename](PreviousError::filename), [line](PreviousError::line) and [error](PreviousError::error) and returns a [PatchFileError::ReadError].
    /// 
    /// # Arguments
    /// - ``function`` - The function where the error occured.
    /// - ``filename`` - The filename where the error occured.
    /// - ``line`` - The line where the error occured.
    /// - ``error`` - The error that occured.
    /// 
    /// # Example
    /// ```rust
    /// let error = PatchFileError::new_read_error("function", "filename", 1, Box::new(std::io::Error::new(std::io::ErrorKind::Other, "error")));
    /// ```
    fn new_read_error(function: &str, filename: &str, line: u32, error: Box<dyn std::error::Error>) -> PatchFileError {
        PatchFileError::ReadError(PreviousError::new(function.to_string(), filename.to_string(), line, error))
    }
}

/// Implement [std::fmt::Debug] trait for [PatchFileError]
impl std::fmt::Debug for PatchFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PatchFileError::ConvertionError(e) => write!(f, "ConvertionError on {}:{} : {}", e.filename, e.line, e.error),
            PatchFileError::ReadError(e) => write!(f, "ReadError on {}:{} : {}", e.filename, e.line, e.error),
            PatchFileError::WrongFormat => write!(f, "Error : WrongFormat"),
        }
    }
}

/// Implement [PartialEq] for [PatchFileError]
impl PartialEq for PatchFileError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            PatchFileError::ConvertionError(e) => {
                match other {
                    PatchFileError::ConvertionError(e2) => e.error.to_string() == e2.error.to_string(),
                    _ => false,
                }
            },
            PatchFileError::ReadError(e) => {
                match other {
                    PatchFileError::ReadError(e2) => e.error.to_string() == e2.error.to_string(),
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

/// This is used to create representation of a patch.
/// 
/// A patch is in the following format:<br/>
/// [``TargetAddress``](HexPatch::target_address):[``Old``](HexPatch::old)->[``New``](HexPatch::new) all in HEX in a TXT file.<br/>
/// Target address is always 16 hex digits long, old value and new value are always 2 hex digits long.
/// 
/// Example:
/// ```
/// 0000000000AF0200:13->37
/// ```
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
    /// ```
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

/// Implement PartialEq for HexPatch
impl PartialEq for HexPatch {
    fn eq(&self, other: &Self) -> bool {
        self.target_address == other.target_address &&
        self.old == other.old &&
        self.new == other.new
    }
}

/// Implement Debug trait for HexPatch
impl std::fmt::Debug for HexPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:016X}:{:02X}->{:02X}", self.target_address, self.old, self.new)
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
/// Example:
/// ```text
/// >test.exe
/// 0000000000AF0200:13->37
/// 0000000000AF0206:37->37
/// ```
/// 
/// Patches are stored in a vector of [HexPatch].
pub struct F1337Patch {
    /// Target file name. Extracted from the first line of the patch file.
    pub target_filename: String,
    /// Vector of patches. Builded from extracted data from the rest of the lines of the patch file.
    pub patches: Vec<HexPatch>,
}

impl F1337Patch {
    /// This is the constructor of [F1337Patch].
    /// 
    /// It takes a mutable reference to a [File] and returns a [Result] of [F1337Patch] or [PatchFileError].
    /// 
    /// # Arguments
    /// - ``patchfile``: A mutable reference to a [File].
    /// 
    /// # Returns
    /// - [Result] of [F1337Patch] or [PatchFileError].
    /// 
    /// # Errors
    /// - [PatchFileError::ConvertionError] if the file contains invalid hex values. Contains [PreviousError].
    /// - [PatchFileError::ReadError] if the file can't be read. Contains [PreviousError].
    /// - [PatchFileError::WrongFormat] if the file is not in the right format.
    /// 
    /// # Example
    /// ```rust
    /// let patchfile = File::open("test.txt").unwrap();
    /// let patch = F1337Patch::new(&mut patchfile).unwrap();
    /// ```
    /// 
    /// # Remarks
    /// The file must be in the following format:<br/>
    /// The first line start with ``>`` and followed by the target file name.<br/>
    /// The rest of the lines are patches in the following format:<br/>
    /// [``TargetAddress``](HexPatch::target_address):[``Old``](HexPatch::old)->[``New``](HexPatch::new) all in HEX in a TXT file.
    /// 
    /// Target address is always 16 hex digits long, old value and new value are always 2 hex digits long.
    /// 
    /// ## Example:
    /// ```text
    /// >test.exe
    /// 0000000000AF0200:13->37
    /// 0000000000AF0206:37->37
    /// ```
    pub fn new(patchfile: &mut File) -> Result<F1337Patch, PatchFileError> {
        patchfile.seek(io::SeekFrom::Start(0)).unwrap();
        let mut bufreader = io::BufReader::new(patchfile);
        let mut first_line = String::new();
        let first_line_size = match bufreader.read_line(&mut first_line) {
            Ok(size) => size,
            Err(e) => return Err(PatchFileError::new_read_error("F1337Patch::new", file!(), line!(), Box::new(e))),
        };
        let target_filename = match Self::get_filename(first_line, first_line_size) {
            Ok(filename) => filename,
            Err(e) => return Err(e),
        };
        let mut patches = Vec::new();
        
        for line in bufreader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => return Err(PatchFileError::new_read_error("F1337Patch::new", file!(), line!(), Box::new(e))),
            };
            let patch = match Self::get_hex_patch_from_line(&line) {
                Ok(patch) => patch,
                Err(e) => return Err(e),
            };
            
            patches.push(patch);
        }
        
        Ok(F1337Patch {
            target_filename,
            patches,
        })
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
    /// let line = "0000000000AF0200:13->37".to_string();
    /// F1337Patch::check_patch_line_format(&line).unwrap();
    /// ```
    fn check_patch_line_format(line: &String) -> Result<(), PatchFileError> {
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
    /// - [PatchFileError::ConvertionError] if the file contains invalid hex values. Contains [PreviousError].
    /// - [PatchFileError::WrongFormat] if the line is not in the right format.
    /// 
    /// # Example
    /// ```rust
    /// let line = "0000000000AF0200:13->37".to_string();
    /// let patch = F1337Patch::get_hex_patch_from_line(&line).unwrap();
    /// ```
    fn get_hex_patch_from_line(line: &String) -> Result<HexPatch, PatchFileError> {
        match Self::check_patch_line_format(line) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        let address = match u64::from_str_radix(&line[0..16], 16) {
            Ok(address) => address,
            Err(e) => return Err(PatchFileError::new_convertion_error("F1337Patch::get_hex_patch_from_line", file!(), line!(), Box::new(e))),
        };
        let old = match u8::from_str_radix(&line[17..19], 16) {
            Ok(old) => old,
            Err(_) => return Err(PatchFileError::WrongFormat),
        };
        let new = match u8::from_str_radix(&line[21..23], 16) {
            Ok(new) => new,
            Err(_) => return Err(PatchFileError::WrongFormat),
        };

        Ok(HexPatch::new(address, old, new))
    }

    /// This function extract filename from the first line of the patch file.
    /// The first line start with ">" and followed by the target file name.
    fn get_filename(first_line: String, size: usize) -> Result<String, PatchFileError> {
        let mut filename = String::with_capacity(size);
        let mut chars = first_line.chars();
        
        if chars.next() != Some('>') {
            return Err(PatchFileError::WrongFormat);
        }
        
        // This gets the filename. Trim the end to remove the \n (and \r\n on windows).
        filename.push_str(&first_line[1..].trim_end());
        
        Ok(filename)
    }
}


/// Implements [std::fmt::Debug] for [F1337Patch].
impl std::fmt::Debug for F1337Patch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("F1337Patch")
            .field("target_filename", &self.target_filename)
            .field("patches", &self.patches)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempfile;
    use std::io::Write;
    
    #[test]
    fn test_f1337patch_new() {
        let mut dummy_file = tempfile().unwrap();
        
        writeln!(dummy_file, ">test.exe").unwrap();
        writeln!(dummy_file, "0000000000AF0200:13->37").unwrap();
        writeln!(dummy_file, "0000000000AF0206:37->37").unwrap();
        println!("Temp file: {:?}", dummy_file);

        let f1337path = F1337Patch::new(&mut dummy_file).unwrap();

        assert_eq!(f1337path.target_filename, "test.exe");
        assert_eq!(f1337path.patches.len(), 2);

        let dummy_patches = vec![
            HexPatch::new(0xAF0200, 0x13, 0x37),
            HexPatch::new(0xAF0206, 0x37, 0x37),
        ];

        assert_eq!(dummy_patches, f1337path.patches);

        drop(dummy_file);
    }

    #[test]
    fn test_get_hex_path_from_line_wrong_format() {
        let line = vec![
            "0000000000AF0200:13->3",
            "000000AF0200:13->32",
            "0000000000AF020089:13->3A",
            "0000000000AF0200:13->ZA",
            "0000000000AF02KK:13->3A",];
        for line in line {
            let patch = F1337Patch::get_hex_patch_from_line(&line.to_string()).unwrap_err();
            assert_eq!(patch, PatchFileError::WrongFormat);
        }
    }

    #[test]
    fn test_get_filename_wrong_format() {
        let first_line = "test.exe".to_string();
        let filename = F1337Patch::get_filename(first_line, 10).unwrap_err();
        assert_eq!(filename, PatchFileError::WrongFormat);
    }
}