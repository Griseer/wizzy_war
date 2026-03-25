use bitflags::bitflags;
use shared::math::Vec2f;

pub struct PhysicBody {
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub vertical_velocity: f32,
    pub height: f32,
}

impl PhysicBody {
    pub fn default() -> Self {
        PhysicBody {
            position: Vec2f::ZERO,
            velocity: Vec2f::ZERO,
            vertical_velocity: 0.0,
            height: 0.0,
        }
    }
}

pub enum ColliderShape {
    Circle { radius: f32 },
    Aabb { half_extents: Vec2f },
}

pub struct Collider {
    pub shape: ColliderShape,
    pub layer: CollisionLayer,
    pub mask: CollisionLayer,
}

impl Collider {
    pub fn cricle(radius: f32) -> Self {
        Collider {
            shape: ColliderShape::Circle { radius },
            layer: CollisionLayer::empty(),
            mask: CollisionLayer::empty(),
        }
    }
}

bitflags! {
    pub struct CollisionLayer: u16 {
        const PLAYER     = 1 << 0;
        const PROJECTILE = 1 << 1;
        const WALL       = 1 << 2;
        const AREA       = 1 << 3;
    }
}
