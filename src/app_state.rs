pub const STAGE: &str = "app_state";
#[derive(Clone)]
pub enum AppState {
    Setup,
    Started,
}
