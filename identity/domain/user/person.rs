use common::result::Result;

use crate::domain::user::{Birthdate, Fullname, Gender, Image};

#[derive(Debug, Clone)]
pub struct Person {
    fullname: Fullname,
    birthdate: Option<Birthdate>,
    gender: Option<Gender>,
    profile_image: Option<Image>,
}

impl Person {
    pub fn new(
        fullname: Fullname,
        birthdate: Option<Birthdate>,
        gender: Option<Gender>,
        profile_image: Option<Image>,
    ) -> Result<Self> {
        Ok(Person {
            fullname,
            birthdate,
            gender,
            profile_image,
        })
    }

    pub fn fullname(&self) -> &Fullname {
        &self.fullname
    }

    pub fn birthdate(&self) -> Option<&Birthdate> {
        self.birthdate.as_ref()
    }

    pub fn gender(&self) -> Option<&Gender> {
        self.gender.as_ref()
    }

    pub fn profile_image(&self) -> Option<&Image> {
        self.profile_image.as_ref()
    }
}
