use std::rc::Rc;

pub struct Envs {
    envs: Option<Vec<Rc<String>>>,
    selected_env: Rc<String>,
}

impl Envs {
    pub fn new() -> Self {
        Self {
            envs: None,
            selected_env: Rc::new("".to_string()),
        }
    }

    pub fn initialized(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_envs(&mut self, envs: Vec<String>) {
        self.envs = Some(envs.into_iter().map(Rc::new).collect());
        let selected_env = crate::storage::selected_env::get();
        self.set_active_env(&selected_env);
    }

    pub fn set_active_env(&mut self, active_env: &str) {
        if let Some(envs) = self.envs.as_mut() {
            for env in envs {
                if env.as_str() == active_env {
                    self.selected_env = env.clone();
                    return;
                }
            }
        }

        if self.selected_env.as_str().is_empty() {
            if let Some(envs) = self.envs.as_ref() {
                if let Some(env) = envs.first() {
                    self.selected_env = env.clone();
                    crate::storage::selected_env::set(env.as_str());
                }
            }
        }
    }

    pub fn get_selected(&self) -> &Rc<String> {
        &self.selected_env
    }

    pub fn is_selected(&self, other: &str) -> bool {
        self.selected_env.as_str() == other
    }

    pub fn iter(&self) -> std::slice::Iter<Rc<String>> {
        let envs = self.envs.as_ref().unwrap();
        envs.iter()
    }
}
