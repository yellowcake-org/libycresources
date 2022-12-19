use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use libycresources::common::types::errors;
use libycresources::common::types::models::Identifier;
use libycresources::common::types::models::sprite::Kind;
use libycresources::formats::{frm, pal, pro};
use libycresources::formats::frm::Sprite;
use libycresources::formats::map::parse;
use libycresources::formats::pal::Palette;
use libycresources::formats::pro::{ObjectType, Prototype};

use crate::error;
use crate::traits::render;

mod suffix;

pub struct CommonProvider<'a> {
    pub directory: &'a Path,
}

impl render::Provider for CommonProvider<'_> {
    fn provide<'b>(&self, identifier: &Identifier<Kind>) -> Result<(Sprite, Option<Palette>), error::Error<'b>> {
        let subdirectory = match identifier.kind {
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

        let directory = &self.directory.join(subdirectory);

        let lst = &directory.join(subdirectory.to_string() + ".LST");
        let mut list = File::open(lst)
            .map_err(|io| error::Error::IO(io, "Failed to open .LST file."))?;

        fn values<'a>(index: u16, lst: &mut File) -> Result<Vec<String>, error::Error<'a>> {
            lst.seek(SeekFrom::Start(0))
                .map_err(|io| error::Error::IO(io, "Couldn't reset file handler to start."))?;

            BufReader::with_capacity(1 * 1024 * 1024, lst)
                .lines()
                .nth(index as usize)
                .ok_or(error::Error::Corrupted(".LST file doesn't contain expected line."))?
                .map_err(|io| error::Error::IO(io, ""))
                .map(|s| {
                    let s = s
                        .splitn(2, |c| c == ' ' || c == ';' || c == '\t')
                        .next()
                        .unwrap_or(&s);

                    Ok(s.split(',').map(|s| { s.to_string() }).collect())
                })?
        }

        let row: Vec<String> = values(identifier.index, &mut list)?;
        let name = row.first()
            .ok_or(error::Error::Corrupted("Could not read sprite name from the .LST entry."))?;

        fn filename<'a>(from: &PathBuf, name: &String) -> Result<PathBuf, error::Error<'a>> {
            from.join(name).to_str()
                .map(|s| s.trim())
                .map(|s| { PathBuf::from(s) })
                .map_or(
                    Err(error::Error::Corrupted("Failed to construct correct file path from .LST index record.")),
                    |p| { Ok(p) },
                )
        }

        fn sprite<'a>(path: &PathBuf) -> Result<Sprite, error::Error<'a>> {
            let file = File::open(&path)
                .map_err(|io| { error::Error::IO(io, "Failed to open sprite file.") })?;

            let mut reader = BufReader::with_capacity(1 * 1024 * 1024, file);
            let sprite = frm::parse::sprite(&mut reader)
                .map_err(|i| error::Error::Internal(i, "Failed to parse sprite."))?;

            Ok(sprite)
        }

        let (sprite, path) = if identifier.kind != Kind::Critter {
            let path = filename(&directory, &name)?;
            (sprite(&path)?, path)
        } else {
            let direction = (identifier.raw >> 28) as u8 & 0b111;

            let weapon = (identifier.raw >> 12) as u8 & 0b1111;
            let animation = (identifier.raw >> 16) as u8;

            let suffix = suffix::detect(weapon, animation).
                ok_or(error::Error::Corrupted("Failed to detect proper .FRM file suffix for a critter."))?;

            fn load<'a>(path: &PathBuf, suffix: &(char, char), direction: u8) -> Result<Sprite, error::Error<'a>> {
                let mut path = path.to_str()
                    .map(|s| {
                        s.to_owned() + format!("{}{}", suffix.0, suffix.1).as_str()
                    })
                    .map(|s| { PathBuf::from(s) })
                    .map_or(
                        Err(error::Error::Corrupted("Failed to construct proper .FRM file path.")),
                        |p| { Ok(p) },
                    )?;

                if direction == 0 {
                    path.set_extension("frm");
                    sprite(&path)
                } else {
                    let mut sprites: [Option<Sprite>; 6] = [None, None, None, None, None, None];

                    for i in 0..6 {
                        path.set_extension("fr".to_owned() + i.to_string().as_str());
                        sprites[i] = Some(sprite(&path)?);
                    }

                    let merged = frm::merge::sprites(sprites.map(|o| o.unwrap()))
                        .map_err(|_| {
                            error::Error::Corrupted("Failed to merge separate .fr0-5 sprites into single one. ")
                        })?;

                    Ok(merged)
                }
            }

            let path = filename(&directory, &name)?;
            let result = load(&path, &suffix, direction);

            if let Ok(result) = result { (result, path) } else {
                let index = row.get(1)
                    .map(|s| u16::from_str(s))
                    .ok_or(error::Error::Corrupted("Couldn't find critter's sprite fallback index."))?
                    .map_err(|_| error::Error::Corrupted("Couldn't parse critter's sprite fallback index."))?;

                let row: Vec<String> = values(index, &mut list)?;
                let name = row.first()
                    .ok_or(error::Error::Corrupted("Could not read fallback sprite name from the .LST entry."))?;

                let path = filename(&directory, &name)?;
                let result = load(&path, &suffix, direction)?;

                (result, path)
            }
        };

        let mut path = path;
        path.set_extension("pal");
        let file = File::open(&path).ok();
        let palette = file
            .map(|f| {
                let mut reader = BufReader::with_capacity(1 * 1024 * 1024, f);
                pal::parse::palette(&mut reader)
            })
            .map_or(Ok(None), |r| { r.map(|p| { Some(p) }) })
            .map_err(|e| {
                error::Error::Internal(e, "Failed to load custom palette for a sprite.")
            })?;

        Ok((sprite, palette))
    }
}

impl parse::Provider for CommonProvider<'_> {
    fn provide(&self, identifier: &Identifier<ObjectType>) -> Result<Prototype, errors::Error> {
        let kind = match identifier.kind {
            ObjectType::Item(_) => "ITEMS",
            ObjectType::Critter(_) => "CRITTERS",
            ObjectType::Scenery(_) => "SCENERY",
            ObjectType::Wall(_) => "WALLS",
            ObjectType::Tile(_) => "TILES",
            ObjectType::Misc(_) => "MISC",
        };

        let directory = &self.directory.join(kind);
        let path = directory.join((|| -> Result<String, errors::Error> {
            let lst = &directory.join(kind.to_owned() + ".LST");

            return BufReader::with_capacity(1 * 1024 * 1024, File::open(lst)?)
                .lines()
                .nth(identifier.index as usize - 1)
                .ok_or(errors::Error::Format)?
                .map_err(|e| errors::Error::IO(e));
        })()?);

        let file = File::open(&path)?;
        let mut reader = BufReader::with_capacity(1 * 1024 * 1024, file);

        Ok(pro::parse::prototype(&mut reader)?)
    }
}