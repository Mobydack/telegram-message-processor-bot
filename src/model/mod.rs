pub mod types;
mod yandex;

use crate::configuration::model::ModelConfiguration;
use std::rc::Rc;
use types::ModelAPI;
use yandex::YandexModel;

pub struct ModelAPIFactory {}

impl ModelAPIFactory {
    pub fn create(model_configuration: &ModelConfiguration) -> Rc<dyn ModelAPI> {
        match &model_configuration {
            ModelConfiguration::Yandex(model) => Rc::new(YandexModel::new(
                model.token.clone(),
                model.catalog_id.clone(),
            )),
        }
    }
}
