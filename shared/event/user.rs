use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

use crate::util;

#[derive(Serialize, Deserialize, Debug)]
pub enum UserEvent {
    Registered {
        id: String,
        username: String,
        email: String,
        validation_code: String,
    },
    LoggedIn {
        id: String,
        auth_token: String,
    },
    Updated {
        id: String,
        name: String,
        lastname: String,
    },
    Validated {
        id: String,
    },
    PasswordRecoveryRequested {
        id: String,
        temp_password: String,
        email: String,
    },
    Deleted {
        id: String,
    },
}

impl ToString for UserEvent {
    fn to_string(&self) -> String {
        match self {
            UserEvent::Registered { .. } => "registered".to_owned(),
            UserEvent::LoggedIn { .. } => "logged-in".to_owned(),
            UserEvent::Updated { .. } => "updated".to_owned(),
            UserEvent::Validated { .. } => "validated".to_owned(),
            UserEvent::PasswordRecoveryRequested { .. } => "password-recovery-requested".to_owned(),
            UserEvent::Deleted { .. } => "deleted".to_owned(),
        }
    }
}

impl ToEvent for UserEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = util::serialize(&self, "user")?;

        Ok(Event::new("user".to_owned(), self.to_string(), payload))
    }
}
