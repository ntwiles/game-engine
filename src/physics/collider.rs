pub struct Collider {
    pub origin: cgmath::Vector2<f32>,
    pub width: f32,
    pub height: f32,
}

impl Collider {
    pub fn up(&self) -> f32 {
        self.origin.y + (self.height / 2.0)
    }

    pub fn down(&self) -> f32 {
        self.origin.y - (self.height / 2.0)
    }

    pub fn left(&self) -> f32 {
        self.origin.x - (self.width / 2.0)
    }

    pub fn right(&self) -> f32 {
        self.origin.x + (self.width / 2.0)
    }

    fn check_overlap_up(&self, offset: f32, other: &Self, other_offset: f32) -> bool {
        self.up() + offset > other.down() + other_offset
            && self.up() + offset < other.up() + other_offset
    }

    fn check_overlap_down(&self, offset: f32, other: &Self, other_offset: f32) -> bool {
        self.down() + offset < other.up() + other_offset
            && self.down() + offset > other.down() + other_offset
    }

    fn check_overlap_left(&self, offset: f32, other: &Self, other_offset: f32) -> bool {
        self.left() + offset < other.right() + other_offset
            && self.left() + offset > other.left() + other_offset
    }

    fn check_overlap_right(&self, offset: f32, other: &Self, other_offset: f32) -> bool {
        self.right() + offset > other.left() + other_offset
            && self.right() + offset < other.right() + other_offset
    }

    fn check_overlap_horz(&self, offset: f32, other: &Self, other_offset: f32) -> bool {
        self.check_overlap_right(offset, other, other_offset)
            || self.check_overlap_left(offset, other, other_offset)
    }

    fn check_overlap_vert(&self, offset: f32, other: &Self, other_offset: f32) -> bool {
        self.check_overlap_up(offset, other, other_offset)
            || self.check_overlap_down(offset, other, other_offset)
    }

    pub fn cast(
        &self,
        offset: cgmath::Vector2<f32>,
        other: &Self,
        other_offset: cgmath::Vector2<f32>,
    ) -> bool {
        self.check_overlap_horz(offset.x, other, other_offset.x)
            && self.check_overlap_vert(offset.y, other, other_offset.y)
    }
}
