use druid::widget::{Flex};
use druid::{Widget};

use crate::models;

pub fn ui_builder() -> impl Widget<models::AppData> {
    Flex::column()
}
