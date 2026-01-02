pub mod app_view;
pub mod home_view;
pub mod settings_view;

use home_view::HomeView;
use settings_view::SettingsView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Views {
    HomeView,
    SettingsView,
}
