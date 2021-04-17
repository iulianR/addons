use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Addon {
    pub id: Uuid,
    pub name: String,
}

impl Addon {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddonPost {
    pub name: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Addons(Vec<Addon>);

// impl Default for Addons {
//     fn default() -> Self {
//         Addons(vec![
//             Addon { id: Uuid::new_v4(), name: "Test1".to_string() },
//             Addon { id: Uuid::new_v4(), name: "Test2".to_string() },
//             Addon { id: Uuid::new_v4(), name: "Test3".to_string() },
//         ])
//     }
// }

// impl Deref for Addons {
//     type Target = Vec<Addon>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for Addons {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl From<Vec<Addon>> for Addons {
//     fn from(addons: Vec<Addon>) -> Self {
//         Addons(addons)
//     }
// }

// The query parameters for list_todos.
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
