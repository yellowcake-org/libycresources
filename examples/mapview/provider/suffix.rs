use libycresources::formats::pro::object::common::critter;
use libycresources::formats::pro::object::item::weapon;

pub(crate) fn detect(weapon: &Option<weapon::Animation>, animation: &critter::Animation) -> Option<(char, char)> {
    fn w_code(weapon: &Option<weapon::Animation>) -> char {
        return (b'c' + match weapon {
            None => return b'a' as char,
            Some(weapon::Animation::Knife) => 1u8,
            Some(weapon::Animation::Club) => 2,
            Some(weapon::Animation::Sledgehammer) => 3,
            Some(weapon::Animation::Spear) => 4,
            Some(weapon::Animation::Pistol) => 5,
            Some(weapon::Animation::SubmachineGun) => 6,
            Some(weapon::Animation::Rifle) => 7,
            Some(weapon::Animation::BigGun) => 8,
            Some(weapon::Animation::Minigun) => 9,
            Some(weapon::Animation::RocketLauncher) => 10,
        }) as char
    }

    fn a_char(a: &critter::Animation) -> char {
        (match a {
            critter::Animation::Stand => 0u8,
            critter::Animation::Walk => 1,
            critter::Animation::JumpBegin => 2,
            critter::Animation::JumpEnd => 3,
            critter::Animation::ClimbLadder => 4,
            critter::Animation::Falling => 5,
            critter::Animation::UpStairsRight => 6,
            critter::Animation::UpStairsLeft => 7,
            critter::Animation::DownStairsRight => 8,
            critter::Animation::DownStairsLeft => 9,
            critter::Animation::MagicHandsGround => 10,
            critter::Animation::MagicHandsMiddle => 11,
            critter::Animation::MagicHandsUp => 12,
            critter::Animation::Dodge => 13,
            critter::Animation::HitFromFront => 14,
            critter::Animation::HitFromBack => 15,
            critter::Animation::ThrowPunch => 16,
            critter::Animation::KickLeg => 17,
            critter::Animation::Throw => 18,
            critter::Animation::Running => 19,
            critter::Animation::FallBack => 20,
            critter::Animation::FallFront => 21,
            critter::Animation::BadLanding => 22,
            critter::Animation::BigHole => 23,
            critter::Animation::CharredBody => 24,
            critter::Animation::ChunksOfFlesh => 25,
            critter::Animation::DancingAutofire => 26,
            critter::Animation::Electrify => 27,
            critter::Animation::SlicedInHalf => 28,
            critter::Animation::BurnedToNothing => 29,
            critter::Animation::ElectrifiedToNothing => 30,
            critter::Animation::ExplodedToNothing => 31,
            critter::Animation::MeltedToNothing => 32,
            critter::Animation::FireDance => 33,
            critter::Animation::FallBackBlood => 34,
            critter::Animation::FallFrontBlood => 35,
            critter::Animation::ProneToStanding => 36,
            critter::Animation::BackToStanding => 37,
            critter::Animation::TakeOut => 38,
            critter::Animation::PutAway => 39,
            critter::Animation::Parry => 40,
            critter::Animation::Thrust => 41,
            critter::Animation::Swing => 42,
            critter::Animation::Point => 43,
            critter::Animation::Unpoint => 44,
            critter::Animation::FireSingle => 45,
            critter::Animation::FireBurst => 46,
            critter::Animation::FireContinuous => 47,
            critter::Animation::FallBackSf => 48,
            critter::Animation::FallFrontSf => 49,
            critter::Animation::BadLandingSf => 50,
            critter::Animation::BigHoleSf => 51,
            critter::Animation::CharredBodySf => 52,
            critter::Animation::ChunksOfFleshSf => 53,
            critter::Animation::DancingAutofireSf => 54,
            critter::Animation::ElectrifySf => 55,
            critter::Animation::SlicedInHalfSf => 56,
            critter::Animation::BurnedToNothingSf => 57,
            critter::Animation::ElectrifiedToNothingSf => 58,
            critter::Animation::ExplodedToNothingSf => 59,
            critter::Animation::MeltedToNothingSf => 60,
            critter::Animation::FallBackBloodSf => 61,
            critter::Animation::FallFrontBloodSf => 62,
            critter::Animation::CalledShotPic => 64,
        }) as char
    }

    fn a_code(animation: &critter::Animation, base: &critter::Animation, root: char) -> char {
        let c = root as u8 + a_char(animation) as u8 - a_char(base) as u8;
        assert!(c.is_ascii());
        c as char
    }

    Some(match animation {
        critter::Animation::ProneToStanding => ('c', 'h'),
        critter::Animation::BackToStanding => ('c', 'j'),
        _ if a_char(animation) as u8 >= a_char(&critter::Animation::TakeOut) as u8 &&
            a_char(animation) as u8 <= a_char(&critter::Animation::FireContinuous) as u8 => {
            if weapon.is_none() { return None; }
            (w_code(weapon), a_code(animation, &critter::Animation::TakeOut, 'c'))
        }
        critter::Animation::CalledShotPic => ('n', 'a'),
        _ if a_char(animation) as u8 >= a_char(&critter::Animation::FallBackSf) as u8 =>
            ('r', a_code(animation, &critter::Animation::FallBackSf, 'a')),
        _ if a_char(animation) as u8 >= a_char(&critter::Animation::FallBack) as u8 =>
            ('b', a_code(animation, &critter::Animation::FallBack, 'a')),
        critter::Animation::Throw => match weapon {
            Some(weapon::Animation::Knife) | Some(weapon::Animation::Spear) => (w_code(weapon), 'm'),
            _ => (w_code(&None), 's')
        }
        _ if animation != &critter::Animation::Dodge => {
            (match animation {
                critter::Animation::Stand | critter::Animation::Walk => w_code(weapon),
                _ => w_code(&None),
            },
             a_code(animation, &critter::Animation::Stand, 'a'))
        }
        _ if weapon.is_none() => { (w_code(weapon), 'n') }
        _ => (w_code(weapon), 'e')
    }).map(|(c1, c2)| (c1, c2))
}