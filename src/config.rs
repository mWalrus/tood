use confy::ConfyError;
use std::fs::File;

pub trait Config {
    fn file_is_empty(file_name: &str) -> Result<bool, ConfyError> {
        match confy::get_configuration_file_path("tood", Some(file_name)) {
            Ok(path) => {
                if !path.exists() {
                    return Ok(true);
                }

                // "safe" to unwrap since we checked if the file exists above
                let f = File::open(path).map_err(ConfyError::OpenConfigurationFileError)?;
                let metadata = f
                    .metadata()
                    .map_err(ConfyError::ReadConfigurationFileError)?;

                // check whether the file is empty
                if metadata.len() <= 1 {
                    return Ok(true);
                }

                Ok(false)
            }
            Err(e) => Err(e),
        }
    }
}
