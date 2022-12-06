use libycresources::common::types::errors::Error;
use libycresources::common::types::models::{Identifier, sprite};
use libycresources::formats::{frm, pal};

pub(crate) trait RenderProvider {
    fn provide(&self, identifier: &Identifier<sprite::Kind>) -> Result<(frm::Sprite, Option<pal::Palette>), Error>;
}