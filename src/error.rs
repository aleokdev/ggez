//! Error types and conversion functions.
use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// An enum containing all kinds of game framework errors.
#[derive(Debug)]
pub enum GameError {
    /// An error in the filesystem layout
    FilesystemError(String),
    /// An error in the config file
    ConfigError(String),
    /// Happens when an `winit::event_loop::EventLoopProxy` attempts to
    /// wake up an `winit::event_loop::EventLoop` that no longer exists.
    EventLoopError(String),
    /// An error trying to load a resource, such as getting an invalid image file.
    ResourceLoadError(String),
    /// Unable to find a resource; the `Vec` is the paths it searched for and associated errors
    ResourceNotFound(String, Vec<(std::path::PathBuf, GameError)>),
    /// Something went wrong in the renderer
    RenderError(String),
    /// Something went wrong when requesting a logical device from the graphics API.
    RequestDeviceError(wgpu::RequestDeviceError),
    /// Something went wrong in the audio playback
    AudioError(String),
    /// Something went wrong trying to set or get window properties.
    WindowError(String),
    /// Something went wrong trying to create a window
    WindowCreationError(Arc<winit::error::OsError>),
    /// Something went wrong trying to read from a file
    #[allow(clippy::upper_case_acronyms)]
    IOError(Arc<std::io::Error>),
    /// Something went wrong trying to load a font
    FontError(glyph_brush::ab_glyph::InvalidFont),
    /// Something went wrong applying video settings.
    VideoError(String),
    /// Something went wrong with the `gilrs` gamepad-input library.
    GamepadError(gilrs::Error),
    /// Something went wrong with the `lyon` shape-tesselation library.
    LyonError(String),
    /// Something went wrong when spawning a task with `futures`.
    SpawnError(futures::task::SpawnError),
    /// Something went wrong when drawing text.
    GlyphBrushError(glyph_brush::BrushError),
    /// Attempted to draw text with a non-existent font name.
    FontSelectError {
        /// The non-existent font that ggez tried to load.
        font: String,
    },
    /// Something went wrong when asynchronously mapping a GPU buffer.
    BufferAsyncError(wgpu::BufferAsyncError),
    /// Deadlock when trying to lock a mutex.
    LockError,
    /// A custom error type for use by users of ggez.
    /// This lets you handle custom errors that may happen during your game (such as, trying to load a malformed file for a level)
    /// using the same mechanism you handle ggez's other errors.
    ///
    /// Please include an informative message with the error.
    CustomError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::ConfigError(s) => write!(f, "Config error: {}", s),
            GameError::ResourceLoadError(s) => write!(f, "Error loading resource: {}", s),
            GameError::ResourceNotFound(s, paths) => write!(
                f,
                "Resource not found: {}, searched in paths {:?}",
                s, paths
            ),
            GameError::WindowError(e) => write!(f, "Window creation error: {}", e),
            GameError::CustomError(s) => write!(f, "Custom error: {}", s),
            GameError::RequestDeviceError(e) => {
                write!(f, "Failed to request logical device: {}", e)
            }
            GameError::SpawnError(e) => {
                write!(f, "Failed to spawn a task with `futures`: {}", e)
            }
            GameError::GlyphBrushError(e) => write!(f, "Text rendering error: {}", e),
            GameError::FontSelectError { font } => write!(f, "No such font '{}'", font),
            GameError::BufferAsyncError(e) => write!(f, "Async buffer map error: {}", e),
            _ => write!(f, "GameError {:?}", self),
        }
    }
}

impl Error for GameError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            GameError::RequestDeviceError(e) => Some(e),
            GameError::WindowCreationError(e) => Some(&**e),
            GameError::IOError(e) => Some(&**e),
            GameError::FontError(e) => Some(e),
            GameError::SpawnError(e) => Some(e),
            GameError::GlyphBrushError(e) => Some(e),
            GameError::BufferAsyncError(e) => Some(e),
            _ => None,
        }
    }
}

/// A convenient result type consisting of a return type and a `GameError`
pub type GameResult<T = ()> = Result<T, GameError>;

impl From<std::io::Error> for GameError {
    fn from(e: std::io::Error) -> GameError {
        GameError::IOError(Arc::new(e))
    }
}

impl From<toml::de::Error> for GameError {
    fn from(e: toml::de::Error) -> GameError {
        let errstr = format!("TOML decode error: {}", e);

        GameError::ConfigError(errstr)
    }
}

impl From<toml::ser::Error> for GameError {
    fn from(e: toml::ser::Error) -> GameError {
        let errstr = format!("TOML error (possibly encoding?): {}", e);
        GameError::ConfigError(errstr)
    }
}

impl From<zip::result::ZipError> for GameError {
    fn from(e: zip::result::ZipError) -> GameError {
        let errstr = format!("Zip error: {}", e);
        GameError::ResourceLoadError(errstr)
    }
}

#[cfg(feature = "audio")]
impl From<rodio::decoder::DecoderError> for GameError {
    fn from(e: rodio::decoder::DecoderError) -> GameError {
        let errstr = format!("Audio decoder error: {:?}", e);
        GameError::AudioError(errstr)
    }
}

#[cfg(feature = "audio")]
impl From<rodio::PlayError> for GameError {
    fn from(e: rodio::PlayError) -> GameError {
        let errstr = format!("Audio playing error: {:?}", e);
        GameError::AudioError(errstr)
    }
}

impl From<image::ImageError> for GameError {
    fn from(e: image::ImageError) -> GameError {
        let errstr = format!("Image load error: {}", e);
        GameError::ResourceLoadError(errstr)
    }
}
impl From<winit::error::OsError> for GameError {
    fn from(s: winit::error::OsError) -> GameError {
        GameError::WindowCreationError(Arc::new(s))
    }
}

#[cfg(feature = "gamepad")]
impl From<gilrs::Error> for GameError {
    fn from(s: gilrs::Error) -> GameError {
        GameError::GamepadError(s)
    }
}

impl From<lyon::lyon_tessellation::TessellationError> for GameError {
    fn from(s: lyon::lyon_tessellation::TessellationError) -> GameError {
        let errstr = format!(
            "Error while tesselating shape (did you give it an infinity or NaN?): {:?}",
            s
        );
        GameError::LyonError(errstr)
    }
}

impl From<lyon::lyon_tessellation::geometry_builder::GeometryBuilderError> for GameError {
    fn from(s: lyon::lyon_tessellation::geometry_builder::GeometryBuilderError) -> GameError {
        let errstr = format!(
            "Error while building geometry (did you give it too many vertices?): {:?}",
            s
        );
        GameError::LyonError(errstr)
    }
}

impl From<wgpu::RequestDeviceError> for GameError {
    fn from(s: wgpu::RequestDeviceError) -> GameError {
        GameError::RequestDeviceError(s)
    }
}

impl From<Arc<winit::error::OsError>> for GameError {
    fn from(s: Arc<winit::error::OsError>) -> GameError {
        GameError::WindowCreationError(s)
    }
}

impl From<Arc<std::io::Error>> for GameError {
    fn from(s: Arc<std::io::Error>) -> GameError {
        GameError::IOError(s)
    }
}

impl From<glyph_brush::ab_glyph::InvalidFont> for GameError {
    fn from(s: glyph_brush::ab_glyph::InvalidFont) -> GameError {
        GameError::FontError(s)
    }
}

impl From<futures::task::SpawnError> for GameError {
    fn from(s: futures::task::SpawnError) -> GameError {
        GameError::SpawnError(s)
    }
}

impl From<glyph_brush::BrushError> for GameError {
    fn from(s: glyph_brush::BrushError) -> GameError {
        GameError::GlyphBrushError(s)
    }
}

impl From<wgpu::BufferAsyncError> for GameError {
    fn from(s: wgpu::BufferAsyncError) -> GameError {
        GameError::BufferAsyncError(s)
    }
}
