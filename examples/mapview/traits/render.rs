use ycresources::common::types::models::{Identifier, sprite};
use ycresources::formats::{frm, pal};

use crate::error::Error;

pub(crate) trait Provider {
    fn provide<'a>(
        &self, identifier: &Identifier<sprite::Kind>,
    ) -> Result<(frm::Sprite, Option<pal::Palette>), Error<'a>>;
}
