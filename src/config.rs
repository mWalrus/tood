use confy::ConfyError;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, rc::Rc};

pub trait Config {
    type Item: Default;

    fn read_from_file(name: &str) -> Result<Option<Self>, ConfyError>
    where
        Self: Sized + Serialize + DeserializeOwned + Default,
    {
        match confy::load("tood", Some(name)) {
            Ok(cfg) => Ok(Some(cfg)),
            Err(_) if Self::file_is_empty(name)? => Ok(None),
            Err(e) => Err(e),
        }
    }
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

    #[allow(clippy::wrong_self_convention)]
    fn to_shared(self) -> Rc<Self::Item>;
}
