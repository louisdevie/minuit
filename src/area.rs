extern crate sdl2;

use sdl2::rect::Rect;

fn scale(a: i32, b: f32) -> i32 {
    (b * a as f32).round() as i32
}

fn safe_size(val: i32) -> u32 {
    if val < 0 {
        0
    } else {
        val.try_into().unwrap()
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
    x: i32,
    y: i32,
    w: i32,
    h: i32,
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
            w: self.absw + scale(parent_area.w, self.relw),
            h: self.absh + scale(parent_area.h, self.relh),
        }
    }
}

impl ResolvedArea {
    pub fn as_sdl_rect(&self) -> Rect {
        Rect::new(self.x, self.y, safe_size(self.w), safe_size(self.h))
    }
}
