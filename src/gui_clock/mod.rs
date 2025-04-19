// Łączymy podmoduły 'app' i 'utils'
pub mod app;
pub mod utils;

// Eksportujemy publicznie funkcje i struktury
pub use app::ClockApp;
pub use utils::polar_to_cartesian;
