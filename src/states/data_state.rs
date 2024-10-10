use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum DataState<T: Debug + Clone> {
    None,
    Loading,
    Loaded(T),
    Error(String),
}

impl<T: Debug + Clone> DataState<T> {
    #[allow(dead_code)]
    pub fn is_none(&self) -> bool {
        match self {
            DataState::None => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_loading(&self) -> bool {
        match self {
            DataState::Loading => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn to_not_loaded_cases(&self) -> Option<NotLoadedCases> {
        match self {
            DataState::None => NotLoadedCases::None.into(),
            DataState::Loading => NotLoadedCases::Loading.into(),
            DataState::Loaded(_) => None,
            DataState::Error(_) => None,
        }
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: Debug + Clone> From<T> for DataState<T> {
    fn from(value: T) -> Self {
        Self::Loaded(value)
    }
}

#[derive(Debug, Clone)]
pub enum NotLoadedCases {
    None,
    Loading,
}
