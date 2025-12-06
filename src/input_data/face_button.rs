// TODO: define a struct here named FaceButtonInput that has a FaceButton and a u8 for length held in frames

#[derive(thiserror::Error, Debug)]
pub enum FaceButtonError {
    #[error("Non Existent Face Button")]
    NonExistentFaceButton,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaceButton {
    Accelerator,
    Brake,
    Item,
    BrakeWhileAcceleratorHeld,
    Unknown,
}

impl From<FaceButton> for u16 {
    fn from(value: FaceButton) -> u16 {
        match value {
            FaceButton::Accelerator => 0x01,
            FaceButton::Brake => 0x02,
            FaceButton::Item => 0x04,
            FaceButton::BrakeWhileAcceleratorHeld => 0x08,
            FaceButton::Unknown => 0xF0,
        }
    }
}

impl TryFrom<u8> for FaceButton {
    type Error = FaceButtonError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(FaceButton::Accelerator),
            0x02 => Ok(FaceButton::Brake),
            0x04 => Ok(FaceButton::Item),
            0x08 => Ok(FaceButton::BrakeWhileAcceleratorHeld),
            0xF0 => Ok(FaceButton::Unknown),
            _ => Err(FaceButtonError::NonExistentFaceButton),
        }
    }
}
