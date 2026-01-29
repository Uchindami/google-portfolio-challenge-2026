//! Custom color theme for the portfolio
use ratatui::style::Color;

/// Custom color palette from portfolio brand
pub struct Theme;

impl Theme {
    /// Teal - primary accent
    pub const PRIMARY: Color = Color::Rgb(165, 200, 158); // #A5C89E
    
    /// Coral - secondary accent
    pub const SECONDARY: Color = Color::Rgb(255, 140, 105); // #FF8C69
    
    /// Bright teal - highlights
    pub const HIGHLIGHT: Color = Color::Rgb(100, 230, 210); // #64E6D2
    
    /// Soft purple - text accent
    pub const ACCENT: Color = Color::Rgb(180, 150, 220); // #B496DC
    
    /// Deep teal variant
    pub const PRIMARY_DARK: Color = Color::Rgb(60, 160, 140); // #3CA08C
    
    /// Background (deep charcoal - easier on eyes)
    pub const BACKGROUND: Color = Color::Rgb(18, 18, 18); // #121212
    
    /// Foreground text (warm white)
    pub const FOREGROUND: Color = Color::Rgb(240, 235, 225); // #F0EBE1
    
    /// Muted text (desaturated)
    pub const MUTED: Color = Color::Rgb(150, 130, 110); // #96826E
    
    /// Success/positive (teal)
    pub const SUCCESS: Color = Color::Rgb(80, 200, 180); // #50C8B4
    
    /// Warning (warm amber)
    pub const WARNING: Color = Color::Rgb(255, 180, 90); // #FFB45A
    
    /// Error (soft red)
    pub const ERROR: Color = Color::Rgb(235, 100, 85); // #EB6455
    
    /// Border color (teal)
    pub const BORDER: Color = Color::Rgb(80, 200, 180); // #50C8B4
    
    /// Selected/active item background
    pub const SELECTED_BG: Color = Color::Rgb(80, 200, 180); // #50C8B4
    
    /// Selected/active item foreground
    pub const SELECTED_FG: Color = Color::Rgb(18, 18, 18); // #121212
}