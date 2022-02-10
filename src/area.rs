extern crate sdl2;

use sdl2::rect::Rect;

fn scale(a: u32, b: f32) -> i32 {
    (b * a as f32).round() as i32
}

pub fn safe_unsigned(val: i32) -> u32 {
    match val.try_into() {
        Err(_) => 0,
        Ok(result) => result,
    }
}

pub fn safe_signed(val: u32) -> i32 {
    match val.try_into() {
        Err(_) => i32::MAX,
        Ok(result) => result,
    }
}

pub struct Area {
    absx: i32,
    absy: i32,
    relx: f32,
    rely: f32,
    absw: i32,
    absh: i32,
    relw: f32,
    relh: f32,
}

pub struct ResolvedArea {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Area {
    pub fn default() -> Self {
        Self {
            absx: 0,
            absy: 0,
            relx: 0.0,
            rely: 0.0,
            absw: 0,
            absh: 0,
            relw: 1.0,
            relh: 1.0,
        }
    }

    pub fn resolve(&self, parent_area: ResolvedArea) -> ResolvedArea {
        ResolvedArea {
            x: parent_area.x + self.absx + scale(parent_area.w, self.relx),
            y: parent_area.y + self.absy + scale(parent_area.h, self.rely),
            w: safe_unsigned(self.absw + scale(parent_area.w, self.relw)),
            h: safe_unsigned(self.absh + scale(parent_area.h, self.relh)),
        }
    }
}

impl ResolvedArea {
    pub fn into_sdl_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }

    pub fn from_sdl_rect(rect: &Rect) -> Self {
        Self {
            x: rect.x(),
            y: rect.y(),
            w: rect.width(),
            h: rect.height(),
        }
    }
}
