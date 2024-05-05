pub mod types;
mod yandex;

use crate::configuration::model::ModelConfiguration;
use std::sync::Arc;
use types::ModelAPI;
use yandex::YandexModel;

pub struct ModelAPIFactory {}

impl ModelAPIFactory {
    pub fn create(model_configuration: &ModelConfiguration) -> Arc<dyn ModelAPI> {
        match &model_configuration {
            ModelConfiguration::Yandex(model) => Arc::new(YandexModel::new(
                model.token.clone(),
                model.catalog_id.clone(),
            )),
        }
    }
}
