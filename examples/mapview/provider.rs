use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use libycresources::common::types::errors::Error;
use libycresources::common::types::models::Identifier;
use libycresources::common::types::models::sprite::Kind;
use libycresources::formats::{frm, pal, pro};
use libycresources::formats::frm::Sprite;
use libycresources::formats::map::parse::PrototypeProvider;
use libycresources::formats::pal::Palette;
use libycresources::formats::pro::{ObjectType, Prototype};

use crate::traits::RenderProvider;

pub struct Provider<'a> {
    pub directory: &'a Path,
}

impl PrototypeProvider for Provider<'_> {
    fn provide(&self, identifier: &Identifier<ObjectType>) -> Result<Prototype, Error> {
        let kind = match identifier.kind {
            ObjectType::Item(_) => "ITEMS",
            ObjectType::Critter(_) => "CRITTERS",
            ObjectType::Scenery(_) => "SCENERY",
            ObjectType::Wall(_) => "WALLS",
            ObjectType::Tile(_) => "TILES",
            ObjectType::Misc(_) => "MISC",
        };

        let directory = &self.directory.join(kind);
        let path = directory.join((|| -> Result<String, Error> {
            let lst = &directory.join(kind.to_owned() + ".LST");

            return BufReader::with_capacity(1 * 1024 * 1024, File::open(lst)?)
                .lines()
                .nth(identifier.value as usize - 1)
                .ok_or(Error::Format)?
                .map_err(|e| Error::IO(e));
        })()?);

        let file = File::open(&path)?;
        let mut reader = BufReader::with_capacity(1 * 1024 * 1024, file);

        Ok(pro::parse::prototype(&mut reader)?)
    }
}

impl RenderProvider for Provider<'_> {
    fn provide(&self, identifier: &Identifier<Kind>) -> Result<(Sprite, Option<Palette>), Error> {
        let kind = match identifier.kind {
            Kind::Item => "ITEMS",
            Kind::Critter => "CRITTERS",
            Kind::Scenery => "SCENERY",
            Kind::Wall => "WALLS",
            Kind::Tile => "TILES",
            Kind::Misc => "MISC",
            Kind::Background => "BACKGRND",
            Kind::Interface => "INTRFACE",
            Kind::Inventory => "INVEN",
            Kind::Head => "HEADS",
            Kind::Skilldex => "SKILLDEX",
        };

        let directory = &self.directory.join(kind);
        let mut path = directory.join((|| -> Result<String, Error> {
            let lst = &directory.join(kind.to_owned() + ".LST");

            return BufReader::with_capacity(1 * 1024 * 1024, File::open(lst)?)
                .lines()
                .nth(identifier.value as usize)
                .ok_or(Error::Format)?
                .map_err(|e| Error::IO(e));
        })()?);

        path = path.to_str()
            .map(|s| s.trim())
            .map(|s| { PathBuf::from(s) })
            .map_or(Err(Error::Format), |p| { Ok(p) })?;

        let file = File::open(&path)?;
        let mut reader = BufReader::with_capacity(1 * 1024 * 1024, file);
        let sprite = frm::parse::sprite(&mut reader)?;

        path.set_extension("pal");
        let file = File::open(&path).ok();
        let palette = file
            .map(|f| {
                let mut reader = BufReader::with_capacity(1 * 1024 * 1024, f);
                pal::parse::palette(&mut reader)
            })
            .map_or(Ok(None), |r| { r.map(|p| { Some(p) }) })?;

        Ok((sprite, palette))
    }
}