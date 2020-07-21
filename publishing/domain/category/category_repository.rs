use common::error::Error;

use crate::domain::category::{Category, CategoryID};

pub trait CategoryRepository {
    fn find_all_categories(&self) -> Result<Vec<Category>, Error>;
    fn find_category_by_id(&self, id: &CategoryID) -> Result<Category, Error>;
}