use crate::error::Error;

mod bitmap;

pub(crate) fn overlay<'a>(image: &mut (&mut Vec<(u8, u8, u8)>, (usize, usize))) -> Result<(), Error<'a>> {
    let (tw, th, sh) = (32usize, 16usize, 8usize);

    let (gw, gh) = (image.1.0 / tw, image.1.1 as usize / th);
    let (gw, gh) = (gw, gh + ((0..gh).reduce(|a, i| {
        a + (i * (th - sh / 2)) / gh
    }).unwrap_or(0) as usize - (th - sh / 2)) / th - 1);

    for gy in 0..gh {
        for gx in 0..gw {
            if gy % 2 == 0 && (gx == gw - 1) { continue; }
            if gy == gh - 1 { continue; }

            let (ox, oy) = ((gx * tw + if gy % 2 == 0 { tw / 2 } else { 0 }) as usize, (gy * th) as usize);
            let (ox, oy) = (ox, oy - (gy as usize * sh as usize / 2));
            let (ox, oy) = (ox, oy + (sh as usize) / 2);

            let v0 = (ox + tw / 2, oy);
            let v1 = (ox + tw - 1, oy + (th - sh) / 2 - 1);
            let v2 = (ox + tw - 1, oy + th - sh / 2 - 1);
            let v3 = (ox + tw / 2, oy + th - 1);
            let v4 = (ox, oy + th - sh / 2 - 1);
            let v5 = (ox, oy + (th - sh) / 2 - 1);

            bitmap::line(v0, v1, image);
            bitmap::line(v1, v2, image);
            bitmap::line(v2, v3, image);
            bitmap::line(v3, v4, image);
            bitmap::line(v4, v5, image);
            bitmap::line(v5, v0, image);
        }
    }

    Ok(())
}