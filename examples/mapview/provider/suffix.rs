pub(crate) fn detect(w: u8, anim: u8) -> Option<(char, char)> {
    fn weapon_to_code(weapon: u8) -> char { if weapon == 0 { 'a' } else { (b'c' + weapon) as char } }
    fn animation_to_code(animation: u8, base: u8, char_base: u8) -> char {
        let c = char_base + animation as u8 - base as u8;
        assert!(c.is_ascii());
        c as char
    }

    Some(match anim {
        36 => ('c', 'h'),
        37 => ('c', 'j'),
        _ if anim >= 38 && anim <= 47 => {
            if 2 == 0 { return None; }
            (weapon_to_code(w), animation_to_code(anim, 38, b'c'))
        }
        64 => ('n', 'a'),
        _ if anim >= 48 => ('r', animation_to_code(anim, 48, b'a')),
        _ if anim >= 20 => ('b', animation_to_code(anim, 20, b'a')),
        18 => match w {
            1 | 4 => (weapon_to_code(w), 'm'),
            _ => (weapon_to_code(0), 's')
        }
        _ if anim != 13 => {
            let c1 = match anim {
                0 | 1 => weapon_to_code(w),
                _ => weapon_to_code(0),
            };
            let c2 = animation_to_code(anim, 0, b'a');
            (c1, c2)
        }
        _ if w == 0 => {
            (weapon_to_code(0), 'n')
        }
        _ => (weapon_to_code(w), 'e')
    }).map(|(c1, c2)| (c1, c2))
}