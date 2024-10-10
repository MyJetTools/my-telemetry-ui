pub const SELECTED_ENV_STORAGE_KEY: &str = "env";

pub fn get() -> String {
    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(SELECTED_ENV_STORAGE_KEY)
        .unwrap_or_default()
}

pub fn set(env: &str) {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().set(SELECTED_ENV_STORAGE_KEY, env);
}
