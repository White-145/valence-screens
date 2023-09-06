#[allow(clippy::module_inception)]
pub mod abstract_decoration {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "Marker component for `abstract_decoration` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AbstractDecorationEntity;
}
#[allow(clippy::module_inception)]
pub mod abstract_donkey {
    #![doc = "Parent class: [`abstract_horse`][super::abstract_horse]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Chest(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Chest {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `abstract_donkey` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AbstractDonkeyEntity;
}
#[allow(clippy::module_inception)]
pub mod abstract_fireball {
    #![doc = "Parent class: [`explosive_projectile`][super::explosive_projectile]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Item(pub valence_protocol::ItemStack);
    #[allow(clippy::derivable_impls)]
    impl Default for Item {
        fn default() -> Self {
            Self(valence_protocol::ItemStack::default())
        }
    }
    #[doc = "Marker component for `abstract_fireball` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AbstractFireballEntity;
}
#[allow(clippy::module_inception)]
pub mod abstract_horse {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HorseFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for HorseFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `abstract_horse` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AbstractHorseEntity;
}
#[allow(clippy::module_inception)]
pub mod abstract_minecart {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DamageWobbleTicks(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for DamageWobbleTicks {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DamageWobbleSide(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for DamageWobbleSide {
        fn default() -> Self {
            Self(1i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DamageWobbleStrength(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for DamageWobbleStrength {
        fn default() -> Self {
            Self(0f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CustomBlockId(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for CustomBlockId {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CustomBlockOffset(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for CustomBlockOffset {
        fn default() -> Self {
            Self(6i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CustomBlockPresent(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for CustomBlockPresent {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `abstract_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AbstractMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod abstract_piglin {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ImmuneToZombification(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for ImmuneToZombification {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `abstract_piglin` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AbstractPiglinEntity;
}
#[allow(clippy::module_inception)]
pub mod abstract_skeleton {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "Marker component for `abstract_skeleton` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AbstractSkeletonEntity;
}
#[allow(clippy::module_inception)]
pub mod allay {
    #![doc = "Parent class: [`path_aware`][super::path_aware]."]
    #[doc = "The bundle of components for spawning `allay` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct AllayEntityBundle {
        pub allay_entity: super::allay::AllayEntity,
        pub allay_dancing: super::allay::Dancing,
        pub allay_can_duplicate: super::allay::CanDuplicate,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for AllayEntityBundle {
        fn default() -> Self {
            Self {
                allay_entity: Default::default(),
                allay_dancing: Default::default(),
                allay_can_duplicate: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ALLAY,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Dancing(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Dancing {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CanDuplicate(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for CanDuplicate {
        fn default() -> Self {
            Self(true)
        }
    }
    #[doc = "Marker component for `allay` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AllayEntity;
}
#[allow(clippy::module_inception)]
pub mod ambient {
    #![doc = "Parent class: [`mob`][super::mob]."]
    #[doc = "Marker component for `ambient` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AmbientEntity;
}
#[allow(clippy::module_inception)]
pub mod animal {
    #![doc = "Parent class: [`passive`][super::passive]."]
    #[doc = "Marker component for `animal` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AnimalEntity;
}
#[allow(clippy::module_inception)]
pub mod area_effect_cloud {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `area_effect_cloud` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct AreaEffectCloudEntityBundle {
        pub area_effect_cloud_entity: super::area_effect_cloud::AreaEffectCloudEntity,
        pub area_effect_cloud_radius: super::area_effect_cloud::Radius,
        pub area_effect_cloud_color: super::area_effect_cloud::Color,
        pub area_effect_cloud_waiting: super::area_effect_cloud::Waiting,
        pub area_effect_cloud_particle_id: super::area_effect_cloud::ParticleId,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for AreaEffectCloudEntityBundle {
        fn default() -> Self {
            Self {
                area_effect_cloud_entity: Default::default(),
                area_effect_cloud_radius: Default::default(),
                area_effect_cloud_color: Default::default(),
                area_effect_cloud_waiting: Default::default(),
                area_effect_cloud_particle_id: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::AREA_EFFECT_CLOUD,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Radius(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for Radius {
        fn default() -> Self {
            Self(3f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Color(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Color {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Waiting(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Waiting {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ParticleId(pub valence_protocol::packets::play::particle_s2c::Particle);
    #[allow(clippy::derivable_impls)]
    impl Default for ParticleId {
        fn default() -> Self {
            Self(valence_protocol::packets::play::particle_s2c::Particle::EntityEffect)
        }
    }
    #[doc = "Marker component for `area_effect_cloud` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AreaEffectCloudEntity;
}
#[allow(clippy::module_inception)]
pub mod armor_stand {
    #![doc = "Parent class: [`living`][super::living]."]
    #[doc = "The bundle of components for spawning `armor_stand` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ArmorStandEntityBundle {
        pub armor_stand_entity: super::armor_stand::ArmorStandEntity,
        pub armor_stand_armor_stand_flags: super::armor_stand::ArmorStandFlags,
        pub armor_stand_tracker_head_rotation: super::armor_stand::TrackerHeadRotation,
        pub armor_stand_tracker_body_rotation: super::armor_stand::TrackerBodyRotation,
        pub armor_stand_tracker_left_arm_rotation: super::armor_stand::TrackerLeftArmRotation,
        pub armor_stand_tracker_right_arm_rotation: super::armor_stand::TrackerRightArmRotation,
        pub armor_stand_tracker_left_leg_rotation: super::armor_stand::TrackerLeftLegRotation,
        pub armor_stand_tracker_right_leg_rotation: super::armor_stand::TrackerRightLegRotation,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ArmorStandEntityBundle {
        fn default() -> Self {
            Self {
                armor_stand_entity: Default::default(),
                armor_stand_armor_stand_flags: Default::default(),
                armor_stand_tracker_head_rotation: Default::default(),
                armor_stand_tracker_body_rotation: Default::default(),
                armor_stand_tracker_left_arm_rotation: Default::default(),
                armor_stand_tracker_right_arm_rotation: Default::default(),
                armor_stand_tracker_left_leg_rotation: Default::default(),
                armor_stand_tracker_right_leg_rotation: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ARMOR_STAND,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ArmorStandFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for ArmorStandFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackerHeadRotation(pub crate::EulerAngle);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackerHeadRotation {
        fn default() -> Self {
            Self(crate::EulerAngle {
                pitch: 0f32,
                yaw: 0f32,
                roll: 0f32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackerBodyRotation(pub crate::EulerAngle);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackerBodyRotation {
        fn default() -> Self {
            Self(crate::EulerAngle {
                pitch: 0f32,
                yaw: 0f32,
                roll: 0f32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackerLeftArmRotation(pub crate::EulerAngle);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackerLeftArmRotation {
        fn default() -> Self {
            Self(crate::EulerAngle {
                pitch: -10f32,
                yaw: 0f32,
                roll: -10f32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackerRightArmRotation(pub crate::EulerAngle);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackerRightArmRotation {
        fn default() -> Self {
            Self(crate::EulerAngle {
                pitch: -15f32,
                yaw: 0f32,
                roll: 10f32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackerLeftLegRotation(pub crate::EulerAngle);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackerLeftLegRotation {
        fn default() -> Self {
            Self(crate::EulerAngle {
                pitch: -1f32,
                yaw: 0f32,
                roll: -1f32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackerRightLegRotation(pub crate::EulerAngle);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackerRightLegRotation {
        fn default() -> Self {
            Self(crate::EulerAngle {
                pitch: 1f32,
                yaw: 0f32,
                roll: 1f32,
            })
        }
    }
    #[doc = "Marker component for `armor_stand` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ArmorStandEntity;
}
#[allow(clippy::module_inception)]
pub mod arrow {
    #![doc = "Parent class: [`persistent_projectile`][super::persistent_projectile]."]
    #[doc = "The bundle of components for spawning `arrow` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ArrowEntityBundle {
        pub arrow_entity: super::arrow::ArrowEntity,
        pub arrow_color: super::arrow::Color,
        pub persistent_projectile_entity: super::persistent_projectile::PersistentProjectileEntity,
        pub persistent_projectile_projectile_flags: super::persistent_projectile::ProjectileFlags,
        pub persistent_projectile_pierce_level: super::persistent_projectile::PierceLevel,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ArrowEntityBundle {
        fn default() -> Self {
            Self {
                arrow_entity: Default::default(),
                arrow_color: Default::default(),
                persistent_projectile_entity: Default::default(),
                persistent_projectile_projectile_flags: Default::default(),
                persistent_projectile_pierce_level: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ARROW,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Color(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Color {
        fn default() -> Self {
            Self(-1i32)
        }
    }
    #[doc = "Marker component for `arrow` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ArrowEntity;
}
#[allow(clippy::module_inception)]
pub mod axolotl {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `axolotl` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct AxolotlEntityBundle {
        pub axolotl_entity: super::axolotl::AxolotlEntity,
        pub axolotl_variant: super::axolotl::Variant,
        pub axolotl_playing_dead: super::axolotl::PlayingDead,
        pub axolotl_from_bucket: super::axolotl::FromBucket,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for AxolotlEntityBundle {
        fn default() -> Self {
            Self {
                axolotl_entity: Default::default(),
                axolotl_variant: Default::default(),
                axolotl_playing_dead: Default::default(),
                axolotl_from_bucket: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::AXOLOTL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Variant(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Variant {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PlayingDead(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for PlayingDead {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct FromBucket(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for FromBucket {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `axolotl` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct AxolotlEntity;
}
#[allow(clippy::module_inception)]
pub mod bat {
    #![doc = "Parent class: [`ambient`][super::ambient]."]
    #[doc = "The bundle of components for spawning `bat` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct BatEntityBundle {
        pub bat_entity: super::bat::BatEntity,
        pub bat_bat_flags: super::bat::BatFlags,
        pub ambient_entity: super::ambient::AmbientEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for BatEntityBundle {
        fn default() -> Self {
            Self {
                bat_entity: Default::default(),
                bat_bat_flags: Default::default(),
                ambient_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::BAT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BatFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for BatFlags {
        fn default() -> Self {
            Self(1i8)
        }
    }
    #[doc = "Marker component for `bat` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct BatEntity;
}
#[allow(clippy::module_inception)]
pub mod bee {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `bee` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct BeeEntityBundle {
        pub bee_entity: super::bee::BeeEntity,
        pub bee_bee_flags: super::bee::BeeFlags,
        pub bee_anger: super::bee::Anger,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for BeeEntityBundle {
        fn default() -> Self {
            Self {
                bee_entity: Default::default(),
                bee_bee_flags: Default::default(),
                bee_anger: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::BEE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BeeFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for BeeFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Anger(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Anger {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `bee` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct BeeEntity;
}
#[allow(clippy::module_inception)]
pub mod blaze {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `blaze` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct BlazeEntityBundle {
        pub blaze_entity: super::blaze::BlazeEntity,
        pub blaze_blaze_flags: super::blaze::BlazeFlags,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for BlazeEntityBundle {
        fn default() -> Self {
            Self {
                blaze_entity: Default::default(),
                blaze_blaze_flags: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::BLAZE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BlazeFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for BlazeFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `blaze` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct BlazeEntity;
}
#[allow(clippy::module_inception)]
pub mod block_display {
    #![doc = "Parent class: [`display`][super::display]."]
    #[doc = "The bundle of components for spawning `block_display` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct BlockDisplayEntityBundle {
        pub block_display_entity: super::block_display::BlockDisplayEntity,
        pub block_display_block_state: super::block_display::BlockState,
        pub display_entity: super::display::DisplayEntity,
        pub display_start_interpolation: super::display::StartInterpolation,
        pub display_interpolation_duration: super::display::InterpolationDuration,
        pub display_translation: super::display::Translation,
        pub display_scale: super::display::Scale,
        pub display_left_rotation: super::display::LeftRotation,
        pub display_right_rotation: super::display::RightRotation,
        pub display_billboard: super::display::Billboard,
        pub display_brightness: super::display::Brightness,
        pub display_view_range: super::display::ViewRange,
        pub display_shadow_radius: super::display::ShadowRadius,
        pub display_shadow_strength: super::display::ShadowStrength,
        pub display_width: super::display::Width,
        pub display_height: super::display::Height,
        pub display_glow_color_override: super::display::GlowColorOverride,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for BlockDisplayEntityBundle {
        fn default() -> Self {
            Self {
                block_display_entity: Default::default(),
                block_display_block_state: Default::default(),
                display_entity: Default::default(),
                display_start_interpolation: Default::default(),
                display_interpolation_duration: Default::default(),
                display_translation: Default::default(),
                display_scale: Default::default(),
                display_left_rotation: Default::default(),
                display_right_rotation: Default::default(),
                display_billboard: Default::default(),
                display_brightness: Default::default(),
                display_view_range: Default::default(),
                display_shadow_radius: Default::default(),
                display_shadow_strength: Default::default(),
                display_width: Default::default(),
                display_height: Default::default(),
                display_glow_color_override: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::BLOCK_DISPLAY,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BlockState(pub valence_protocol::BlockState);
    #[allow(clippy::derivable_impls)]
    impl Default for BlockState {
        fn default() -> Self {
            Self(valence_protocol::BlockState::default())
        }
    }
    #[doc = "Marker component for `block_display` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct BlockDisplayEntity;
}
#[allow(clippy::module_inception)]
pub mod boat {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `boat` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct BoatEntityBundle {
        pub boat_entity: super::boat::BoatEntity,
        pub boat_damage_wobble_ticks: super::boat::DamageWobbleTicks,
        pub boat_damage_wobble_side: super::boat::DamageWobbleSide,
        pub boat_damage_wobble_strength: super::boat::DamageWobbleStrength,
        pub boat_boat_type: super::boat::BoatType,
        pub boat_left_paddle_moving: super::boat::LeftPaddleMoving,
        pub boat_right_paddle_moving: super::boat::RightPaddleMoving,
        pub boat_bubble_wobble_ticks: super::boat::BubbleWobbleTicks,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for BoatEntityBundle {
        fn default() -> Self {
            Self {
                boat_entity: Default::default(),
                boat_damage_wobble_ticks: Default::default(),
                boat_damage_wobble_side: Default::default(),
                boat_damage_wobble_strength: Default::default(),
                boat_boat_type: Default::default(),
                boat_left_paddle_moving: Default::default(),
                boat_right_paddle_moving: Default::default(),
                boat_bubble_wobble_ticks: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::BOAT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DamageWobbleTicks(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for DamageWobbleTicks {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DamageWobbleSide(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for DamageWobbleSide {
        fn default() -> Self {
            Self(1i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DamageWobbleStrength(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for DamageWobbleStrength {
        fn default() -> Self {
            Self(0f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BoatType(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for BoatType {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LeftPaddleMoving(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for LeftPaddleMoving {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct RightPaddleMoving(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for RightPaddleMoving {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BubbleWobbleTicks(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for BubbleWobbleTicks {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `boat` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct BoatEntity;
}
#[allow(clippy::module_inception)]
pub mod camel {
    #![doc = "Parent class: [`abstract_horse`][super::abstract_horse]."]
    #[doc = "The bundle of components for spawning `camel` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct CamelEntityBundle {
        pub camel_entity: super::camel::CamelEntity,
        pub camel_dashing: super::camel::Dashing,
        pub camel_last_pose_tick: super::camel::LastPoseTick,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for CamelEntityBundle {
        fn default() -> Self {
            Self {
                camel_entity: Default::default(),
                camel_dashing: Default::default(),
                camel_last_pose_tick: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::CAMEL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Dashing(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Dashing {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LastPoseTick(pub i64);
    #[allow(clippy::derivable_impls)]
    impl Default for LastPoseTick {
        fn default() -> Self {
            Self(0i64)
        }
    }
    #[doc = "Marker component for `camel` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct CamelEntity;
}
#[allow(clippy::module_inception)]
pub mod cat {
    #![doc = "Parent class: [`tameable`][super::tameable]."]
    #[doc = "The bundle of components for spawning `cat` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct CatEntityBundle {
        pub cat_entity: super::cat::CatEntity,
        pub cat_cat_variant: super::cat::CatVariant,
        pub cat_in_sleeping_pose: super::cat::InSleepingPose,
        pub cat_head_down: super::cat::HeadDown,
        pub cat_collar_color: super::cat::CollarColor,
        pub tameable_entity: super::tameable::TameableEntity,
        pub tameable_tameable_flags: super::tameable::TameableFlags,
        pub tameable_owner_uuid: super::tameable::OwnerUuid,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for CatEntityBundle {
        fn default() -> Self {
            Self {
                cat_entity: Default::default(),
                cat_cat_variant: Default::default(),
                cat_in_sleeping_pose: Default::default(),
                cat_head_down: Default::default(),
                cat_collar_color: Default::default(),
                tameable_entity: Default::default(),
                tameable_tameable_flags: Default::default(),
                tameable_owner_uuid: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::CAT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CatVariant(pub crate::CatKind);
    #[allow(clippy::derivable_impls)]
    impl Default for CatVariant {
        fn default() -> Self {
            Self(crate::CatKind::Black)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct InSleepingPose(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for InSleepingPose {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HeadDown(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for HeadDown {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CollarColor(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for CollarColor {
        fn default() -> Self {
            Self(14i32)
        }
    }
    #[doc = "Marker component for `cat` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct CatEntity;
}
#[allow(clippy::module_inception)]
pub mod cave_spider {
    #![doc = "Parent class: [`spider`][super::spider]."]
    #[doc = "The bundle of components for spawning `cave_spider` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct CaveSpiderEntityBundle {
        pub cave_spider_entity: super::cave_spider::CaveSpiderEntity,
        pub spider_entity: super::spider::SpiderEntity,
        pub spider_spider_flags: super::spider::SpiderFlags,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for CaveSpiderEntityBundle {
        fn default() -> Self {
            Self {
                cave_spider_entity: Default::default(),
                spider_entity: Default::default(),
                spider_spider_flags: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::CAVE_SPIDER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `cave_spider` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct CaveSpiderEntity;
}
#[allow(clippy::module_inception)]
pub mod chest_boat {
    #![doc = "Parent class: [`boat`][super::boat]."]
    #[doc = "The bundle of components for spawning `chest_boat` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ChestBoatEntityBundle {
        pub chest_boat_entity: super::chest_boat::ChestBoatEntity,
        pub boat_entity: super::boat::BoatEntity,
        pub boat_damage_wobble_ticks: super::boat::DamageWobbleTicks,
        pub boat_damage_wobble_side: super::boat::DamageWobbleSide,
        pub boat_damage_wobble_strength: super::boat::DamageWobbleStrength,
        pub boat_boat_type: super::boat::BoatType,
        pub boat_left_paddle_moving: super::boat::LeftPaddleMoving,
        pub boat_right_paddle_moving: super::boat::RightPaddleMoving,
        pub boat_bubble_wobble_ticks: super::boat::BubbleWobbleTicks,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ChestBoatEntityBundle {
        fn default() -> Self {
            Self {
                chest_boat_entity: Default::default(),
                boat_entity: Default::default(),
                boat_damage_wobble_ticks: Default::default(),
                boat_damage_wobble_side: Default::default(),
                boat_damage_wobble_strength: Default::default(),
                boat_boat_type: Default::default(),
                boat_left_paddle_moving: Default::default(),
                boat_right_paddle_moving: Default::default(),
                boat_bubble_wobble_ticks: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::CHEST_BOAT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `chest_boat` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ChestBoatEntity;
}
#[allow(clippy::module_inception)]
pub mod chest_minecart {
    #![doc = "Parent class: [`storage_minecart`][super::storage_minecart]."]
    #[doc = "The bundle of components for spawning `chest_minecart` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ChestMinecartEntityBundle {
        pub chest_minecart_entity: super::chest_minecart::ChestMinecartEntity,
        pub storage_minecart_entity: super::storage_minecart::StorageMinecartEntity,
        pub abstract_minecart_entity: super::abstract_minecart::AbstractMinecartEntity,
        pub abstract_minecart_damage_wobble_ticks: super::abstract_minecart::DamageWobbleTicks,
        pub abstract_minecart_damage_wobble_side: super::abstract_minecart::DamageWobbleSide,
        pub abstract_minecart_damage_wobble_strength:
            super::abstract_minecart::DamageWobbleStrength,
        pub abstract_minecart_custom_block_id: super::abstract_minecart::CustomBlockId,
        pub abstract_minecart_custom_block_offset: super::abstract_minecart::CustomBlockOffset,
        pub abstract_minecart_custom_block_present: super::abstract_minecart::CustomBlockPresent,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ChestMinecartEntityBundle {
        fn default() -> Self {
            Self {
                chest_minecart_entity: Default::default(),
                storage_minecart_entity: Default::default(),
                abstract_minecart_entity: Default::default(),
                abstract_minecart_damage_wobble_ticks: Default::default(),
                abstract_minecart_damage_wobble_side: Default::default(),
                abstract_minecart_damage_wobble_strength: Default::default(),
                abstract_minecart_custom_block_id: Default::default(),
                abstract_minecart_custom_block_offset: Default::default(),
                abstract_minecart_custom_block_present: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::CHEST_MINECART,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `chest_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ChestMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod chicken {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `chicken` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ChickenEntityBundle {
        pub chicken_entity: super::chicken::ChickenEntity,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ChickenEntityBundle {
        fn default() -> Self {
            Self {
                chicken_entity: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::CHICKEN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `chicken` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ChickenEntity;
}
#[allow(clippy::module_inception)]
pub mod cod {
    #![doc = "Parent class: [`schooling_fish`][super::schooling_fish]."]
    #[doc = "The bundle of components for spawning `cod` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct CodEntityBundle {
        pub cod_entity: super::cod::CodEntity,
        pub schooling_fish_entity: super::schooling_fish::SchoolingFishEntity,
        pub fish_entity: super::fish::FishEntity,
        pub fish_from_bucket: super::fish::FromBucket,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for CodEntityBundle {
        fn default() -> Self {
            Self {
                cod_entity: Default::default(),
                schooling_fish_entity: Default::default(),
                fish_entity: Default::default(),
                fish_from_bucket: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::COD,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `cod` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct CodEntity;
}
#[allow(clippy::module_inception)]
pub mod command_block_minecart {
    #![doc = "Parent class: [`abstract_minecart`][super::abstract_minecart]."]
    #[doc = "The bundle of components for spawning `command_block_minecart` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct CommandBlockMinecartEntityBundle {
        pub command_block_minecart_entity:
            super::command_block_minecart::CommandBlockMinecartEntity,
        pub command_block_minecart_command: super::command_block_minecart::Command,
        pub command_block_minecart_last_output: super::command_block_minecart::LastOutput,
        pub abstract_minecart_entity: super::abstract_minecart::AbstractMinecartEntity,
        pub abstract_minecart_damage_wobble_ticks: super::abstract_minecart::DamageWobbleTicks,
        pub abstract_minecart_damage_wobble_side: super::abstract_minecart::DamageWobbleSide,
        pub abstract_minecart_damage_wobble_strength:
            super::abstract_minecart::DamageWobbleStrength,
        pub abstract_minecart_custom_block_id: super::abstract_minecart::CustomBlockId,
        pub abstract_minecart_custom_block_offset: super::abstract_minecart::CustomBlockOffset,
        pub abstract_minecart_custom_block_present: super::abstract_minecart::CustomBlockPresent,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for CommandBlockMinecartEntityBundle {
        fn default() -> Self {
            Self {
                command_block_minecart_entity: Default::default(),
                command_block_minecart_command: Default::default(),
                command_block_minecart_last_output: Default::default(),
                abstract_minecart_entity: Default::default(),
                abstract_minecart_damage_wobble_ticks: Default::default(),
                abstract_minecart_damage_wobble_side: Default::default(),
                abstract_minecart_damage_wobble_strength: Default::default(),
                abstract_minecart_custom_block_id: Default::default(),
                abstract_minecart_custom_block_offset: Default::default(),
                abstract_minecart_custom_block_present: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::COMMAND_BLOCK_MINECART,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Command(pub String);
    #[allow(clippy::derivable_impls)]
    impl Default for Command {
        fn default() -> Self {
            Self("".to_owned())
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LastOutput(pub valence_protocol::Text);
    #[allow(clippy::derivable_impls)]
    impl Default for LastOutput {
        fn default() -> Self {
            Self(valence_protocol::Text::default())
        }
    }
    #[doc = "Marker component for `command_block_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct CommandBlockMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod cow {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `cow` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct CowEntityBundle {
        pub cow_entity: super::cow::CowEntity,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for CowEntityBundle {
        fn default() -> Self {
            Self {
                cow_entity: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::COW,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `cow` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct CowEntity;
}
#[allow(clippy::module_inception)]
pub mod creeper {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `creeper` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct CreeperEntityBundle {
        pub creeper_entity: super::creeper::CreeperEntity,
        pub creeper_fuse_speed: super::creeper::FuseSpeed,
        pub creeper_charged: super::creeper::Charged,
        pub creeper_ignited: super::creeper::Ignited,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for CreeperEntityBundle {
        fn default() -> Self {
            Self {
                creeper_entity: Default::default(),
                creeper_fuse_speed: Default::default(),
                creeper_charged: Default::default(),
                creeper_ignited: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::CREEPER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct FuseSpeed(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for FuseSpeed {
        fn default() -> Self {
            Self(-1i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Charged(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Charged {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Ignited(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Ignited {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `creeper` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct CreeperEntity;
}
#[allow(clippy::module_inception)]
pub mod display {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct StartInterpolation(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for StartInterpolation {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct InterpolationDuration(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for InterpolationDuration {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Translation(pub valence_math::Vec3);
    #[allow(clippy::derivable_impls)]
    impl Default for Translation {
        fn default() -> Self {
            Self(valence_math::Vec3::new(0f32, 0f32, 0f32))
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Scale(pub valence_math::Vec3);
    #[allow(clippy::derivable_impls)]
    impl Default for Scale {
        fn default() -> Self {
            Self(valence_math::Vec3::new(1f32, 1f32, 1f32))
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LeftRotation(pub valence_math::Quat);
    #[allow(clippy::derivable_impls)]
    impl Default for LeftRotation {
        fn default() -> Self {
            Self(valence_math::Quat::from_xyzw(0f32, 0f32, 0f32, 1f32))
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct RightRotation(pub valence_math::Quat);
    #[allow(clippy::derivable_impls)]
    impl Default for RightRotation {
        fn default() -> Self {
            Self(valence_math::Quat::from_xyzw(0f32, 0f32, 0f32, 1f32))
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Billboard(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for Billboard {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Brightness(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Brightness {
        fn default() -> Self {
            Self(-1i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ViewRange(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for ViewRange {
        fn default() -> Self {
            Self(1f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ShadowRadius(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for ShadowRadius {
        fn default() -> Self {
            Self(0f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ShadowStrength(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for ShadowStrength {
        fn default() -> Self {
            Self(1f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Width(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for Width {
        fn default() -> Self {
            Self(0f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Height(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for Height {
        fn default() -> Self {
            Self(0f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct GlowColorOverride(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for GlowColorOverride {
        fn default() -> Self {
            Self(-1i32)
        }
    }
    #[doc = "Marker component for `display` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct DisplayEntity;
}
#[allow(clippy::module_inception)]
pub mod dolphin {
    #![doc = "Parent class: [`water_creature`][super::water_creature]."]
    #[doc = "The bundle of components for spawning `dolphin` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct DolphinEntityBundle {
        pub dolphin_entity: super::dolphin::DolphinEntity,
        pub dolphin_treasure_pos: super::dolphin::TreasurePos,
        pub dolphin_has_fish: super::dolphin::HasFish,
        pub dolphin_moistness: super::dolphin::Moistness,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for DolphinEntityBundle {
        fn default() -> Self {
            Self {
                dolphin_entity: Default::default(),
                dolphin_treasure_pos: Default::default(),
                dolphin_has_fish: Default::default(),
                dolphin_moistness: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::DOLPHIN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TreasurePos(pub valence_protocol::BlockPos);
    #[allow(clippy::derivable_impls)]
    impl Default for TreasurePos {
        fn default() -> Self {
            Self(valence_protocol::BlockPos {
                x: 0i32,
                y: 0i32,
                z: 0i32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HasFish(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for HasFish {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Moistness(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Moistness {
        fn default() -> Self {
            Self(2400i32)
        }
    }
    #[doc = "Marker component for `dolphin` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct DolphinEntity;
}
#[allow(clippy::module_inception)]
pub mod donkey {
    #![doc = "Parent class: [`abstract_donkey`][super::abstract_donkey]."]
    #[doc = "The bundle of components for spawning `donkey` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct DonkeyEntityBundle {
        pub donkey_entity: super::donkey::DonkeyEntity,
        pub abstract_donkey_entity: super::abstract_donkey::AbstractDonkeyEntity,
        pub abstract_donkey_chest: super::abstract_donkey::Chest,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for DonkeyEntityBundle {
        fn default() -> Self {
            Self {
                donkey_entity: Default::default(),
                abstract_donkey_entity: Default::default(),
                abstract_donkey_chest: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::DONKEY,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `donkey` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct DonkeyEntity;
}
#[allow(clippy::module_inception)]
pub mod dragon_fireball {
    #![doc = "Parent class: [`explosive_projectile`][super::explosive_projectile]."]
    #[doc = "The bundle of components for spawning `dragon_fireball` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct DragonFireballEntityBundle {
        pub dragon_fireball_entity: super::dragon_fireball::DragonFireballEntity,
        pub explosive_projectile_entity: super::explosive_projectile::ExplosiveProjectileEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for DragonFireballEntityBundle {
        fn default() -> Self {
            Self {
                dragon_fireball_entity: Default::default(),
                explosive_projectile_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::DRAGON_FIREBALL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `dragon_fireball` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct DragonFireballEntity;
}
#[allow(clippy::module_inception)]
pub mod drowned {
    #![doc = "Parent class: [`zombie`][super::zombie]."]
    #[doc = "The bundle of components for spawning `drowned` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct DrownedEntityBundle {
        pub drowned_entity: super::drowned::DrownedEntity,
        pub zombie_entity: super::zombie::ZombieEntity,
        pub zombie_baby: super::zombie::Baby,
        pub zombie_zombie_type: super::zombie::ZombieType,
        pub zombie_converting_in_water: super::zombie::ConvertingInWater,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for DrownedEntityBundle {
        fn default() -> Self {
            Self {
                drowned_entity: Default::default(),
                zombie_entity: Default::default(),
                zombie_baby: Default::default(),
                zombie_zombie_type: Default::default(),
                zombie_converting_in_water: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::DROWNED,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `drowned` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct DrownedEntity;
}
#[allow(clippy::module_inception)]
pub mod egg {
    #![doc = "Parent class: [`thrown_item`][super::thrown_item]."]
    #[doc = "The bundle of components for spawning `egg` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EggEntityBundle {
        pub egg_entity: super::egg::EggEntity,
        pub thrown_item_entity: super::thrown_item::ThrownItemEntity,
        pub thrown_item_item: super::thrown_item::Item,
        pub thrown_entity: super::thrown::ThrownEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EggEntityBundle {
        fn default() -> Self {
            Self {
                egg_entity: Default::default(),
                thrown_item_entity: Default::default(),
                thrown_item_item: Default::default(),
                thrown_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::EGG,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `egg` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EggEntity;
}
#[allow(clippy::module_inception)]
pub mod elder_guardian {
    #![doc = "Parent class: [`guardian`][super::guardian]."]
    #[doc = "The bundle of components for spawning `elder_guardian` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ElderGuardianEntityBundle {
        pub elder_guardian_entity: super::elder_guardian::ElderGuardianEntity,
        pub guardian_entity: super::guardian::GuardianEntity,
        pub guardian_spikes_retracted: super::guardian::SpikesRetracted,
        pub guardian_beam_target_id: super::guardian::BeamTargetId,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ElderGuardianEntityBundle {
        fn default() -> Self {
            Self {
                elder_guardian_entity: Default::default(),
                guardian_entity: Default::default(),
                guardian_spikes_retracted: Default::default(),
                guardian_beam_target_id: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ELDER_GUARDIAN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `elder_guardian` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ElderGuardianEntity;
}
#[allow(clippy::module_inception)]
pub mod end_crystal {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `end_crystal` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EndCrystalEntityBundle {
        pub end_crystal_entity: super::end_crystal::EndCrystalEntity,
        pub end_crystal_beam_target: super::end_crystal::BeamTarget,
        pub end_crystal_show_bottom: super::end_crystal::ShowBottom,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EndCrystalEntityBundle {
        fn default() -> Self {
            Self {
                end_crystal_entity: Default::default(),
                end_crystal_beam_target: Default::default(),
                end_crystal_show_bottom: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::END_CRYSTAL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BeamTarget(pub Option<valence_protocol::BlockPos>);
    #[allow(clippy::derivable_impls)]
    impl Default for BeamTarget {
        fn default() -> Self {
            Self(None)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ShowBottom(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for ShowBottom {
        fn default() -> Self {
            Self(true)
        }
    }
    #[doc = "Marker component for `end_crystal` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EndCrystalEntity;
}
#[allow(clippy::module_inception)]
pub mod ender_dragon {
    #![doc = "Parent class: [`mob`][super::mob]."]
    #[doc = "The bundle of components for spawning `ender_dragon` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EnderDragonEntityBundle {
        pub ender_dragon_entity: super::ender_dragon::EnderDragonEntity,
        pub ender_dragon_phase_type: super::ender_dragon::PhaseType,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EnderDragonEntityBundle {
        fn default() -> Self {
            Self {
                ender_dragon_entity: Default::default(),
                ender_dragon_phase_type: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ENDER_DRAGON,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PhaseType(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for PhaseType {
        fn default() -> Self {
            Self(10i32)
        }
    }
    #[doc = "Marker component for `ender_dragon` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EnderDragonEntity;
}
#[allow(clippy::module_inception)]
pub mod ender_pearl {
    #![doc = "Parent class: [`thrown_item`][super::thrown_item]."]
    #[doc = "The bundle of components for spawning `ender_pearl` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EnderPearlEntityBundle {
        pub ender_pearl_entity: super::ender_pearl::EnderPearlEntity,
        pub thrown_item_entity: super::thrown_item::ThrownItemEntity,
        pub thrown_item_item: super::thrown_item::Item,
        pub thrown_entity: super::thrown::ThrownEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EnderPearlEntityBundle {
        fn default() -> Self {
            Self {
                ender_pearl_entity: Default::default(),
                thrown_item_entity: Default::default(),
                thrown_item_item: Default::default(),
                thrown_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ENDER_PEARL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `ender_pearl` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EnderPearlEntity;
}
#[allow(clippy::module_inception)]
pub mod enderman {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `enderman` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EndermanEntityBundle {
        pub enderman_entity: super::enderman::EndermanEntity,
        pub enderman_carried_block: super::enderman::CarriedBlock,
        pub enderman_angry: super::enderman::Angry,
        pub enderman_provoked: super::enderman::Provoked,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EndermanEntityBundle {
        fn default() -> Self {
            Self {
                enderman_entity: Default::default(),
                enderman_carried_block: Default::default(),
                enderman_angry: Default::default(),
                enderman_provoked: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ENDERMAN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CarriedBlock(pub valence_protocol::BlockState);
    #[allow(clippy::derivable_impls)]
    impl Default for CarriedBlock {
        fn default() -> Self {
            Self(valence_protocol::BlockState::default())
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Angry(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Angry {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Provoked(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Provoked {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `enderman` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EndermanEntity;
}
#[allow(clippy::module_inception)]
pub mod endermite {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `endermite` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EndermiteEntityBundle {
        pub endermite_entity: super::endermite::EndermiteEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EndermiteEntityBundle {
        fn default() -> Self {
            Self {
                endermite_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ENDERMITE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `endermite` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EndermiteEntity;
}
#[allow(clippy::module_inception)]
pub mod entity {
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Flags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for Flags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Air(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Air {
        fn default() -> Self {
            Self(300i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CustomName(pub Option<valence_protocol::Text>);
    #[allow(clippy::derivable_impls)]
    impl Default for CustomName {
        fn default() -> Self {
            Self(None)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct NameVisible(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for NameVisible {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Silent(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Silent {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct NoGravity(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for NoGravity {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Pose(pub crate::Pose);
    #[allow(clippy::derivable_impls)]
    impl Default for Pose {
        fn default() -> Self {
            Self(crate::Pose::Standing)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct FrozenTicks(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for FrozenTicks {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `entity` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct Entity;
}
#[allow(clippy::module_inception)]
pub mod evoker {
    #![doc = "Parent class: [`spellcasting_illager`][super::spellcasting_illager]."]
    #[doc = "The bundle of components for spawning `evoker` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EvokerEntityBundle {
        pub evoker_entity: super::evoker::EvokerEntity,
        pub spellcasting_illager_entity: super::spellcasting_illager::SpellcastingIllagerEntity,
        pub spellcasting_illager_spell: super::spellcasting_illager::Spell,
        pub illager_entity: super::illager::IllagerEntity,
        pub raider_entity: super::raider::RaiderEntity,
        pub raider_celebrating: super::raider::Celebrating,
        pub patrol_entity: super::patrol::PatrolEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EvokerEntityBundle {
        fn default() -> Self {
            Self {
                evoker_entity: Default::default(),
                spellcasting_illager_entity: Default::default(),
                spellcasting_illager_spell: Default::default(),
                illager_entity: Default::default(),
                raider_entity: Default::default(),
                raider_celebrating: Default::default(),
                patrol_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::EVOKER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `evoker` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EvokerEntity;
}
#[allow(clippy::module_inception)]
pub mod evoker_fangs {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `evoker_fangs` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EvokerFangsEntityBundle {
        pub evoker_fangs_entity: super::evoker_fangs::EvokerFangsEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EvokerFangsEntityBundle {
        fn default() -> Self {
            Self {
                evoker_fangs_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::EVOKER_FANGS,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `evoker_fangs` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EvokerFangsEntity;
}
#[allow(clippy::module_inception)]
pub mod experience_bottle {
    #![doc = "Parent class: [`thrown_item`][super::thrown_item]."]
    #[doc = "The bundle of components for spawning `experience_bottle` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ExperienceBottleEntityBundle {
        pub experience_bottle_entity: super::experience_bottle::ExperienceBottleEntity,
        pub thrown_item_entity: super::thrown_item::ThrownItemEntity,
        pub thrown_item_item: super::thrown_item::Item,
        pub thrown_entity: super::thrown::ThrownEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ExperienceBottleEntityBundle {
        fn default() -> Self {
            Self {
                experience_bottle_entity: Default::default(),
                thrown_item_entity: Default::default(),
                thrown_item_item: Default::default(),
                thrown_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::EXPERIENCE_BOTTLE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `experience_bottle` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ExperienceBottleEntity;
}
#[allow(clippy::module_inception)]
pub mod experience_orb {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `experience_orb` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ExperienceOrbEntityBundle {
        pub experience_orb_entity: super::experience_orb::ExperienceOrbEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ExperienceOrbEntityBundle {
        fn default() -> Self {
            Self {
                experience_orb_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::EXPERIENCE_ORB,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `experience_orb` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ExperienceOrbEntity;
}
#[allow(clippy::module_inception)]
pub mod explosive_projectile {
    #![doc = "Parent class: [`projectile`][super::projectile]."]
    #[doc = "Marker component for `explosive_projectile` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ExplosiveProjectileEntity;
}
#[allow(clippy::module_inception)]
pub mod eye_of_ender {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `eye_of_ender` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct EyeOfEnderEntityBundle {
        pub eye_of_ender_entity: super::eye_of_ender::EyeOfEnderEntity,
        pub eye_of_ender_item: super::eye_of_ender::Item,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for EyeOfEnderEntityBundle {
        fn default() -> Self {
            Self {
                eye_of_ender_entity: Default::default(),
                eye_of_ender_item: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::EYE_OF_ENDER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Item(pub valence_protocol::ItemStack);
    #[allow(clippy::derivable_impls)]
    impl Default for Item {
        fn default() -> Self {
            Self(valence_protocol::ItemStack::default())
        }
    }
    #[doc = "Marker component for `eye_of_ender` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct EyeOfEnderEntity;
}
#[allow(clippy::module_inception)]
pub mod falling_block {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `falling_block` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct FallingBlockEntityBundle {
        pub falling_block_entity: super::falling_block::FallingBlockEntity,
        pub falling_block_block_pos: super::falling_block::BlockPos,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for FallingBlockEntityBundle {
        fn default() -> Self {
            Self {
                falling_block_entity: Default::default(),
                falling_block_block_pos: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::FALLING_BLOCK,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BlockPos(pub valence_protocol::BlockPos);
    #[allow(clippy::derivable_impls)]
    impl Default for BlockPos {
        fn default() -> Self {
            Self(valence_protocol::BlockPos {
                x: 0i32,
                y: 0i32,
                z: 0i32,
            })
        }
    }
    #[doc = "Marker component for `falling_block` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FallingBlockEntity;
}
#[allow(clippy::module_inception)]
pub mod fireball {
    #![doc = "Parent class: [`abstract_fireball`][super::abstract_fireball]."]
    #[doc = "The bundle of components for spawning `fireball` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct FireballEntityBundle {
        pub fireball_entity: super::fireball::FireballEntity,
        pub abstract_fireball_entity: super::abstract_fireball::AbstractFireballEntity,
        pub abstract_fireball_item: super::abstract_fireball::Item,
        pub explosive_projectile_entity: super::explosive_projectile::ExplosiveProjectileEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for FireballEntityBundle {
        fn default() -> Self {
            Self {
                fireball_entity: Default::default(),
                abstract_fireball_entity: Default::default(),
                abstract_fireball_item: Default::default(),
                explosive_projectile_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::FIREBALL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `fireball` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FireballEntity;
}
#[allow(clippy::module_inception)]
pub mod firework_rocket {
    #![doc = "Parent class: [`projectile`][super::projectile]."]
    #[doc = "The bundle of components for spawning `firework_rocket` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct FireworkRocketEntityBundle {
        pub firework_rocket_entity: super::firework_rocket::FireworkRocketEntity,
        pub firework_rocket_item: super::firework_rocket::Item,
        pub firework_rocket_shooter_entity_id: super::firework_rocket::ShooterEntityId,
        pub firework_rocket_shot_at_angle: super::firework_rocket::ShotAtAngle,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for FireworkRocketEntityBundle {
        fn default() -> Self {
            Self {
                firework_rocket_entity: Default::default(),
                firework_rocket_item: Default::default(),
                firework_rocket_shooter_entity_id: Default::default(),
                firework_rocket_shot_at_angle: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::FIREWORK_ROCKET,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Item(pub valence_protocol::ItemStack);
    #[allow(clippy::derivable_impls)]
    impl Default for Item {
        fn default() -> Self {
            Self(valence_protocol::ItemStack::default())
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ShooterEntityId(pub Option<i32>);
    #[allow(clippy::derivable_impls)]
    impl Default for ShooterEntityId {
        fn default() -> Self {
            Self(None)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ShotAtAngle(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for ShotAtAngle {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `firework_rocket` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FireworkRocketEntity;
}
#[allow(clippy::module_inception)]
pub mod fish {
    #![doc = "Parent class: [`water_creature`][super::water_creature]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct FromBucket(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for FromBucket {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `fish` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FishEntity;
}
#[allow(clippy::module_inception)]
pub mod fishing_bobber {
    #![doc = "Parent class: [`projectile`][super::projectile]."]
    #[doc = "The bundle of components for spawning `fishing_bobber` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct FishingBobberEntityBundle {
        pub fishing_bobber_entity: super::fishing_bobber::FishingBobberEntity,
        pub fishing_bobber_hook_entity_id: super::fishing_bobber::HookEntityId,
        pub fishing_bobber_caught_fish: super::fishing_bobber::CaughtFish,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for FishingBobberEntityBundle {
        fn default() -> Self {
            Self {
                fishing_bobber_entity: Default::default(),
                fishing_bobber_hook_entity_id: Default::default(),
                fishing_bobber_caught_fish: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::FISHING_BOBBER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HookEntityId(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for HookEntityId {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CaughtFish(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for CaughtFish {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `fishing_bobber` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FishingBobberEntity;
}
#[allow(clippy::module_inception)]
pub mod flying {
    #![doc = "Parent class: [`mob`][super::mob]."]
    #[doc = "Marker component for `flying` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FlyingEntity;
}
#[allow(clippy::module_inception)]
pub mod fox {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `fox` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct FoxEntityBundle {
        pub fox_entity: super::fox::FoxEntity,
        pub fox_type: super::fox::Type,
        pub fox_fox_flags: super::fox::FoxFlags,
        pub fox_owner: super::fox::Owner,
        pub fox_other_trusted: super::fox::OtherTrusted,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for FoxEntityBundle {
        fn default() -> Self {
            Self {
                fox_entity: Default::default(),
                fox_type: Default::default(),
                fox_fox_flags: Default::default(),
                fox_owner: Default::default(),
                fox_other_trusted: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::FOX,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Type(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Type {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct FoxFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for FoxFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Owner(pub Option<::uuid::Uuid>);
    #[allow(clippy::derivable_impls)]
    impl Default for Owner {
        fn default() -> Self {
            Self(None)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct OtherTrusted(pub Option<::uuid::Uuid>);
    #[allow(clippy::derivable_impls)]
    impl Default for OtherTrusted {
        fn default() -> Self {
            Self(None)
        }
    }
    #[doc = "Marker component for `fox` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FoxEntity;
}
#[allow(clippy::module_inception)]
pub mod frog {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `frog` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct FrogEntityBundle {
        pub frog_entity: super::frog::FrogEntity,
        pub frog_variant: super::frog::Variant,
        pub frog_target: super::frog::Target,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for FrogEntityBundle {
        fn default() -> Self {
            Self {
                frog_entity: Default::default(),
                frog_variant: Default::default(),
                frog_target: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::FROG,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Variant(pub crate::FrogKind);
    #[allow(clippy::derivable_impls)]
    impl Default for Variant {
        fn default() -> Self {
            Self(crate::FrogKind::Temperate)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Target(pub Option<i32>);
    #[allow(clippy::derivable_impls)]
    impl Default for Target {
        fn default() -> Self {
            Self(None)
        }
    }
    #[doc = "Marker component for `frog` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FrogEntity;
}
#[allow(clippy::module_inception)]
pub mod furnace_minecart {
    #![doc = "Parent class: [`abstract_minecart`][super::abstract_minecart]."]
    #[doc = "The bundle of components for spawning `furnace_minecart` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct FurnaceMinecartEntityBundle {
        pub furnace_minecart_entity: super::furnace_minecart::FurnaceMinecartEntity,
        pub furnace_minecart_lit: super::furnace_minecart::Lit,
        pub abstract_minecart_entity: super::abstract_minecart::AbstractMinecartEntity,
        pub abstract_minecart_damage_wobble_ticks: super::abstract_minecart::DamageWobbleTicks,
        pub abstract_minecart_damage_wobble_side: super::abstract_minecart::DamageWobbleSide,
        pub abstract_minecart_damage_wobble_strength:
            super::abstract_minecart::DamageWobbleStrength,
        pub abstract_minecart_custom_block_id: super::abstract_minecart::CustomBlockId,
        pub abstract_minecart_custom_block_offset: super::abstract_minecart::CustomBlockOffset,
        pub abstract_minecart_custom_block_present: super::abstract_minecart::CustomBlockPresent,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for FurnaceMinecartEntityBundle {
        fn default() -> Self {
            Self {
                furnace_minecart_entity: Default::default(),
                furnace_minecart_lit: Default::default(),
                abstract_minecart_entity: Default::default(),
                abstract_minecart_damage_wobble_ticks: Default::default(),
                abstract_minecart_damage_wobble_side: Default::default(),
                abstract_minecart_damage_wobble_strength: Default::default(),
                abstract_minecart_custom_block_id: Default::default(),
                abstract_minecart_custom_block_offset: Default::default(),
                abstract_minecart_custom_block_present: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::FURNACE_MINECART,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Lit(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Lit {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `furnace_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct FurnaceMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod ghast {
    #![doc = "Parent class: [`flying`][super::flying]."]
    #[doc = "The bundle of components for spawning `ghast` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct GhastEntityBundle {
        pub ghast_entity: super::ghast::GhastEntity,
        pub ghast_shooting: super::ghast::Shooting,
        pub flying_entity: super::flying::FlyingEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for GhastEntityBundle {
        fn default() -> Self {
            Self {
                ghast_entity: Default::default(),
                ghast_shooting: Default::default(),
                flying_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::GHAST,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Shooting(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Shooting {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `ghast` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct GhastEntity;
}
#[allow(clippy::module_inception)]
pub mod giant {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `giant` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct GiantEntityBundle {
        pub giant_entity: super::giant::GiantEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for GiantEntityBundle {
        fn default() -> Self {
            Self {
                giant_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::GIANT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `giant` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct GiantEntity;
}
#[allow(clippy::module_inception)]
pub mod glow_item_frame {
    #![doc = "Parent class: [`item_frame`][super::item_frame]."]
    #[doc = "The bundle of components for spawning `glow_item_frame` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct GlowItemFrameEntityBundle {
        pub glow_item_frame_entity: super::glow_item_frame::GlowItemFrameEntity,
        pub item_frame_entity: super::item_frame::ItemFrameEntity,
        pub item_frame_item_stack: super::item_frame::ItemStack,
        pub item_frame_rotation: super::item_frame::Rotation,
        pub abstract_decoration_entity: super::abstract_decoration::AbstractDecorationEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for GlowItemFrameEntityBundle {
        fn default() -> Self {
            Self {
                glow_item_frame_entity: Default::default(),
                item_frame_entity: Default::default(),
                item_frame_item_stack: Default::default(),
                item_frame_rotation: Default::default(),
                abstract_decoration_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::GLOW_ITEM_FRAME,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `glow_item_frame` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct GlowItemFrameEntity;
}
#[allow(clippy::module_inception)]
pub mod glow_squid {
    #![doc = "Parent class: [`squid`][super::squid]."]
    #[doc = "The bundle of components for spawning `glow_squid` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct GlowSquidEntityBundle {
        pub glow_squid_entity: super::glow_squid::GlowSquidEntity,
        pub glow_squid_dark_ticks_remaining: super::glow_squid::DarkTicksRemaining,
        pub squid_entity: super::squid::SquidEntity,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for GlowSquidEntityBundle {
        fn default() -> Self {
            Self {
                glow_squid_entity: Default::default(),
                glow_squid_dark_ticks_remaining: Default::default(),
                squid_entity: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::GLOW_SQUID,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DarkTicksRemaining(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for DarkTicksRemaining {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `glow_squid` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct GlowSquidEntity;
}
#[allow(clippy::module_inception)]
pub mod goat {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `goat` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct GoatEntityBundle {
        pub goat_entity: super::goat::GoatEntity,
        pub goat_screaming: super::goat::Screaming,
        pub goat_left_horn: super::goat::LeftHorn,
        pub goat_right_horn: super::goat::RightHorn,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for GoatEntityBundle {
        fn default() -> Self {
            Self {
                goat_entity: Default::default(),
                goat_screaming: Default::default(),
                goat_left_horn: Default::default(),
                goat_right_horn: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::GOAT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Screaming(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Screaming {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LeftHorn(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for LeftHorn {
        fn default() -> Self {
            Self(true)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct RightHorn(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for RightHorn {
        fn default() -> Self {
            Self(true)
        }
    }
    #[doc = "Marker component for `goat` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct GoatEntity;
}
#[allow(clippy::module_inception)]
pub mod golem {
    #![doc = "Parent class: [`path_aware`][super::path_aware]."]
    #[doc = "Marker component for `golem` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct GolemEntity;
}
#[allow(clippy::module_inception)]
pub mod guardian {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `guardian` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct GuardianEntityBundle {
        pub guardian_entity: super::guardian::GuardianEntity,
        pub guardian_spikes_retracted: super::guardian::SpikesRetracted,
        pub guardian_beam_target_id: super::guardian::BeamTargetId,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for GuardianEntityBundle {
        fn default() -> Self {
            Self {
                guardian_entity: Default::default(),
                guardian_spikes_retracted: Default::default(),
                guardian_beam_target_id: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::GUARDIAN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct SpikesRetracted(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for SpikesRetracted {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BeamTargetId(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for BeamTargetId {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `guardian` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct GuardianEntity;
}
#[allow(clippy::module_inception)]
pub mod hoglin {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `hoglin` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct HoglinEntityBundle {
        pub hoglin_entity: super::hoglin::HoglinEntity,
        pub hoglin_baby: super::hoglin::Baby,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for HoglinEntityBundle {
        fn default() -> Self {
            Self {
                hoglin_entity: Default::default(),
                hoglin_baby: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::HOGLIN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Baby(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Baby {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `hoglin` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct HoglinEntity;
}
#[allow(clippy::module_inception)]
pub mod hopper_minecart {
    #![doc = "Parent class: [`storage_minecart`][super::storage_minecart]."]
    #[doc = "The bundle of components for spawning `hopper_minecart` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct HopperMinecartEntityBundle {
        pub hopper_minecart_entity: super::hopper_minecart::HopperMinecartEntity,
        pub storage_minecart_entity: super::storage_minecart::StorageMinecartEntity,
        pub abstract_minecart_entity: super::abstract_minecart::AbstractMinecartEntity,
        pub abstract_minecart_damage_wobble_ticks: super::abstract_minecart::DamageWobbleTicks,
        pub abstract_minecart_damage_wobble_side: super::abstract_minecart::DamageWobbleSide,
        pub abstract_minecart_damage_wobble_strength:
            super::abstract_minecart::DamageWobbleStrength,
        pub abstract_minecart_custom_block_id: super::abstract_minecart::CustomBlockId,
        pub abstract_minecart_custom_block_offset: super::abstract_minecart::CustomBlockOffset,
        pub abstract_minecart_custom_block_present: super::abstract_minecart::CustomBlockPresent,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for HopperMinecartEntityBundle {
        fn default() -> Self {
            Self {
                hopper_minecart_entity: Default::default(),
                storage_minecart_entity: Default::default(),
                abstract_minecart_entity: Default::default(),
                abstract_minecart_damage_wobble_ticks: Default::default(),
                abstract_minecart_damage_wobble_side: Default::default(),
                abstract_minecart_damage_wobble_strength: Default::default(),
                abstract_minecart_custom_block_id: Default::default(),
                abstract_minecart_custom_block_offset: Default::default(),
                abstract_minecart_custom_block_present: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::HOPPER_MINECART,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `hopper_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct HopperMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod horse {
    #![doc = "Parent class: [`abstract_horse`][super::abstract_horse]."]
    #[doc = "The bundle of components for spawning `horse` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct HorseEntityBundle {
        pub horse_entity: super::horse::HorseEntity,
        pub horse_variant: super::horse::Variant,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for HorseEntityBundle {
        fn default() -> Self {
            Self {
                horse_entity: Default::default(),
                horse_variant: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::HORSE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Variant(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Variant {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `horse` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct HorseEntity;
}
#[allow(clippy::module_inception)]
pub mod hostile {
    #![doc = "Parent class: [`path_aware`][super::path_aware]."]
    #[doc = "Marker component for `hostile` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct HostileEntity;
}
#[allow(clippy::module_inception)]
pub mod husk {
    #![doc = "Parent class: [`zombie`][super::zombie]."]
    #[doc = "The bundle of components for spawning `husk` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct HuskEntityBundle {
        pub husk_entity: super::husk::HuskEntity,
        pub zombie_entity: super::zombie::ZombieEntity,
        pub zombie_baby: super::zombie::Baby,
        pub zombie_zombie_type: super::zombie::ZombieType,
        pub zombie_converting_in_water: super::zombie::ConvertingInWater,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for HuskEntityBundle {
        fn default() -> Self {
            Self {
                husk_entity: Default::default(),
                zombie_entity: Default::default(),
                zombie_baby: Default::default(),
                zombie_zombie_type: Default::default(),
                zombie_converting_in_water: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::HUSK,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `husk` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct HuskEntity;
}
#[allow(clippy::module_inception)]
pub mod illager {
    #![doc = "Parent class: [`raider`][super::raider]."]
    #[doc = "Marker component for `illager` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct IllagerEntity;
}
#[allow(clippy::module_inception)]
pub mod illusioner {
    #![doc = "Parent class: [`spellcasting_illager`][super::spellcasting_illager]."]
    #[doc = "The bundle of components for spawning `illusioner` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct IllusionerEntityBundle {
        pub illusioner_entity: super::illusioner::IllusionerEntity,
        pub spellcasting_illager_entity: super::spellcasting_illager::SpellcastingIllagerEntity,
        pub spellcasting_illager_spell: super::spellcasting_illager::Spell,
        pub illager_entity: super::illager::IllagerEntity,
        pub raider_entity: super::raider::RaiderEntity,
        pub raider_celebrating: super::raider::Celebrating,
        pub patrol_entity: super::patrol::PatrolEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for IllusionerEntityBundle {
        fn default() -> Self {
            Self {
                illusioner_entity: Default::default(),
                spellcasting_illager_entity: Default::default(),
                spellcasting_illager_spell: Default::default(),
                illager_entity: Default::default(),
                raider_entity: Default::default(),
                raider_celebrating: Default::default(),
                patrol_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ILLUSIONER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `illusioner` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct IllusionerEntity;
}
#[allow(clippy::module_inception)]
pub mod interaction {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `interaction` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct InteractionEntityBundle {
        pub interaction_entity: super::interaction::InteractionEntity,
        pub interaction_width: super::interaction::Width,
        pub interaction_height: super::interaction::Height,
        pub interaction_response: super::interaction::Response,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for InteractionEntityBundle {
        fn default() -> Self {
            Self {
                interaction_entity: Default::default(),
                interaction_width: Default::default(),
                interaction_height: Default::default(),
                interaction_response: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::INTERACTION,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Width(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for Width {
        fn default() -> Self {
            Self(1f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Height(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for Height {
        fn default() -> Self {
            Self(1f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Response(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Response {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `interaction` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct InteractionEntity;
}
#[allow(clippy::module_inception)]
pub mod iron_golem {
    #![doc = "Parent class: [`golem`][super::golem]."]
    #[doc = "The bundle of components for spawning `iron_golem` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct IronGolemEntityBundle {
        pub iron_golem_entity: super::iron_golem::IronGolemEntity,
        pub iron_golem_iron_golem_flags: super::iron_golem::IronGolemFlags,
        pub golem_entity: super::golem::GolemEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for IronGolemEntityBundle {
        fn default() -> Self {
            Self {
                iron_golem_entity: Default::default(),
                iron_golem_iron_golem_flags: Default::default(),
                golem_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::IRON_GOLEM,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct IronGolemFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for IronGolemFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `iron_golem` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct IronGolemEntity;
}
#[allow(clippy::module_inception)]
pub mod item_display {
    #![doc = "Parent class: [`display`][super::display]."]
    #[doc = "The bundle of components for spawning `item_display` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ItemDisplayEntityBundle {
        pub item_display_entity: super::item_display::ItemDisplayEntity,
        pub item_display_item: super::item_display::Item,
        pub item_display_item_display: super::item_display::ItemDisplay,
        pub display_entity: super::display::DisplayEntity,
        pub display_start_interpolation: super::display::StartInterpolation,
        pub display_interpolation_duration: super::display::InterpolationDuration,
        pub display_translation: super::display::Translation,
        pub display_scale: super::display::Scale,
        pub display_left_rotation: super::display::LeftRotation,
        pub display_right_rotation: super::display::RightRotation,
        pub display_billboard: super::display::Billboard,
        pub display_brightness: super::display::Brightness,
        pub display_view_range: super::display::ViewRange,
        pub display_shadow_radius: super::display::ShadowRadius,
        pub display_shadow_strength: super::display::ShadowStrength,
        pub display_width: super::display::Width,
        pub display_height: super::display::Height,
        pub display_glow_color_override: super::display::GlowColorOverride,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ItemDisplayEntityBundle {
        fn default() -> Self {
            Self {
                item_display_entity: Default::default(),
                item_display_item: Default::default(),
                item_display_item_display: Default::default(),
                display_entity: Default::default(),
                display_start_interpolation: Default::default(),
                display_interpolation_duration: Default::default(),
                display_translation: Default::default(),
                display_scale: Default::default(),
                display_left_rotation: Default::default(),
                display_right_rotation: Default::default(),
                display_billboard: Default::default(),
                display_brightness: Default::default(),
                display_view_range: Default::default(),
                display_shadow_radius: Default::default(),
                display_shadow_strength: Default::default(),
                display_width: Default::default(),
                display_height: Default::default(),
                display_glow_color_override: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ITEM_DISPLAY,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Item(pub valence_protocol::ItemStack);
    #[allow(clippy::derivable_impls)]
    impl Default for Item {
        fn default() -> Self {
            Self(valence_protocol::ItemStack::default())
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ItemDisplay(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for ItemDisplay {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `item_display` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ItemDisplayEntity;
}
#[allow(clippy::module_inception)]
pub mod item {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `item` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ItemEntityBundle {
        pub item_entity: super::item::ItemEntity,
        pub item_stack: super::item::Stack,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ItemEntityBundle {
        fn default() -> Self {
            Self {
                item_entity: Default::default(),
                item_stack: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ITEM,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Stack(pub valence_protocol::ItemStack);
    #[allow(clippy::derivable_impls)]
    impl Default for Stack {
        fn default() -> Self {
            Self(valence_protocol::ItemStack::default())
        }
    }
    #[doc = "Marker component for `item` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ItemEntity;
}
#[allow(clippy::module_inception)]
pub mod item_frame {
    #![doc = "Parent class: [`abstract_decoration`][super::abstract_decoration]."]
    #[doc = "The bundle of components for spawning `item_frame` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ItemFrameEntityBundle {
        pub item_frame_entity: super::item_frame::ItemFrameEntity,
        pub item_frame_item_stack: super::item_frame::ItemStack,
        pub item_frame_rotation: super::item_frame::Rotation,
        pub abstract_decoration_entity: super::abstract_decoration::AbstractDecorationEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ItemFrameEntityBundle {
        fn default() -> Self {
            Self {
                item_frame_entity: Default::default(),
                item_frame_item_stack: Default::default(),
                item_frame_rotation: Default::default(),
                abstract_decoration_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ITEM_FRAME,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ItemStack(pub valence_protocol::ItemStack);
    #[allow(clippy::derivable_impls)]
    impl Default for ItemStack {
        fn default() -> Self {
            Self(valence_protocol::ItemStack::default())
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Rotation(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Rotation {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `item_frame` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ItemFrameEntity;
}
#[allow(clippy::module_inception)]
pub mod leash_knot {
    #![doc = "Parent class: [`abstract_decoration`][super::abstract_decoration]."]
    #[doc = "The bundle of components for spawning `leash_knot` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct LeashKnotEntityBundle {
        pub leash_knot_entity: super::leash_knot::LeashKnotEntity,
        pub abstract_decoration_entity: super::abstract_decoration::AbstractDecorationEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for LeashKnotEntityBundle {
        fn default() -> Self {
            Self {
                leash_knot_entity: Default::default(),
                abstract_decoration_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::LEASH_KNOT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `leash_knot` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct LeashKnotEntity;
}
#[allow(clippy::module_inception)]
pub mod lightning {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `lightning` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct LightningEntityBundle {
        pub lightning_entity: super::lightning::LightningEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for LightningEntityBundle {
        fn default() -> Self {
            Self {
                lightning_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::LIGHTNING,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `lightning` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct LightningEntity;
}
#[allow(clippy::module_inception)]
pub mod living {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LivingFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for LivingFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Health(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for Health {
        fn default() -> Self {
            Self(6f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PotionSwirlsColor(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for PotionSwirlsColor {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PotionSwirlsAmbient(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for PotionSwirlsAmbient {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct StuckArrowCount(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for StuckArrowCount {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct StingerCount(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for StingerCount {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct SleepingPosition(pub Option<valence_protocol::BlockPos>);
    #[allow(clippy::derivable_impls)]
    impl Default for SleepingPosition {
        fn default() -> Self {
            Self(None)
        }
    }
    #[doc = "Marker component for `living` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct LivingEntity;
}
#[allow(clippy::module_inception)]
pub mod llama {
    #![doc = "Parent class: [`abstract_donkey`][super::abstract_donkey]."]
    #[doc = "The bundle of components for spawning `llama` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct LlamaEntityBundle {
        pub llama_entity: super::llama::LlamaEntity,
        pub llama_strength: super::llama::Strength,
        pub llama_carpet_color: super::llama::CarpetColor,
        pub llama_variant: super::llama::Variant,
        pub abstract_donkey_entity: super::abstract_donkey::AbstractDonkeyEntity,
        pub abstract_donkey_chest: super::abstract_donkey::Chest,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for LlamaEntityBundle {
        fn default() -> Self {
            Self {
                llama_entity: Default::default(),
                llama_strength: Default::default(),
                llama_carpet_color: Default::default(),
                llama_variant: Default::default(),
                abstract_donkey_entity: Default::default(),
                abstract_donkey_chest: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::LLAMA,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Strength(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Strength {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CarpetColor(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for CarpetColor {
        fn default() -> Self {
            Self(-1i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Variant(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Variant {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `llama` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct LlamaEntity;
}
#[allow(clippy::module_inception)]
pub mod llama_spit {
    #![doc = "Parent class: [`projectile`][super::projectile]."]
    #[doc = "The bundle of components for spawning `llama_spit` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct LlamaSpitEntityBundle {
        pub llama_spit_entity: super::llama_spit::LlamaSpitEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for LlamaSpitEntityBundle {
        fn default() -> Self {
            Self {
                llama_spit_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::LLAMA_SPIT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `llama_spit` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct LlamaSpitEntity;
}
#[allow(clippy::module_inception)]
pub mod magma_cube {
    #![doc = "Parent class: [`slime`][super::slime]."]
    #[doc = "The bundle of components for spawning `magma_cube` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct MagmaCubeEntityBundle {
        pub magma_cube_entity: super::magma_cube::MagmaCubeEntity,
        pub slime_entity: super::slime::SlimeEntity,
        pub slime_slime_size: super::slime::SlimeSize,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for MagmaCubeEntityBundle {
        fn default() -> Self {
            Self {
                magma_cube_entity: Default::default(),
                slime_entity: Default::default(),
                slime_slime_size: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::MAGMA_CUBE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `magma_cube` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct MagmaCubeEntity;
}
#[allow(clippy::module_inception)]
pub mod marker {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `marker` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct MarkerEntityBundle {
        pub marker_entity: super::marker::MarkerEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for MarkerEntityBundle {
        fn default() -> Self {
            Self {
                marker_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::MARKER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `marker` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct MarkerEntity;
}
#[allow(clippy::module_inception)]
pub mod merchant {
    #![doc = "Parent class: [`passive`][super::passive]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HeadRollingTimeLeft(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for HeadRollingTimeLeft {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `merchant` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct MerchantEntity;
}
#[allow(clippy::module_inception)]
pub mod minecart {
    #![doc = "Parent class: [`abstract_minecart`][super::abstract_minecart]."]
    #[doc = "The bundle of components for spawning `minecart` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct MinecartEntityBundle {
        pub minecart_entity: super::minecart::MinecartEntity,
        pub abstract_minecart_entity: super::abstract_minecart::AbstractMinecartEntity,
        pub abstract_minecart_damage_wobble_ticks: super::abstract_minecart::DamageWobbleTicks,
        pub abstract_minecart_damage_wobble_side: super::abstract_minecart::DamageWobbleSide,
        pub abstract_minecart_damage_wobble_strength:
            super::abstract_minecart::DamageWobbleStrength,
        pub abstract_minecart_custom_block_id: super::abstract_minecart::CustomBlockId,
        pub abstract_minecart_custom_block_offset: super::abstract_minecart::CustomBlockOffset,
        pub abstract_minecart_custom_block_present: super::abstract_minecart::CustomBlockPresent,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for MinecartEntityBundle {
        fn default() -> Self {
            Self {
                minecart_entity: Default::default(),
                abstract_minecart_entity: Default::default(),
                abstract_minecart_damage_wobble_ticks: Default::default(),
                abstract_minecart_damage_wobble_side: Default::default(),
                abstract_minecart_damage_wobble_strength: Default::default(),
                abstract_minecart_custom_block_id: Default::default(),
                abstract_minecart_custom_block_offset: Default::default(),
                abstract_minecart_custom_block_present: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::MINECART,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct MinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod mob {
    #![doc = "Parent class: [`living`][super::living]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct MobFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for MobFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `mob` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct MobEntity;
}
#[allow(clippy::module_inception)]
pub mod mooshroom {
    #![doc = "Parent class: [`cow`][super::cow]."]
    #[doc = "The bundle of components for spawning `mooshroom` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct MooshroomEntityBundle {
        pub mooshroom_entity: super::mooshroom::MooshroomEntity,
        pub mooshroom_type: super::mooshroom::Type,
        pub cow_entity: super::cow::CowEntity,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for MooshroomEntityBundle {
        fn default() -> Self {
            Self {
                mooshroom_entity: Default::default(),
                mooshroom_type: Default::default(),
                cow_entity: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::MOOSHROOM,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Type(pub String);
    #[allow(clippy::derivable_impls)]
    impl Default for Type {
        fn default() -> Self {
            Self("red".to_owned())
        }
    }
    #[doc = "Marker component for `mooshroom` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct MooshroomEntity;
}
#[allow(clippy::module_inception)]
pub mod mule {
    #![doc = "Parent class: [`abstract_donkey`][super::abstract_donkey]."]
    #[doc = "The bundle of components for spawning `mule` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct MuleEntityBundle {
        pub mule_entity: super::mule::MuleEntity,
        pub abstract_donkey_entity: super::abstract_donkey::AbstractDonkeyEntity,
        pub abstract_donkey_chest: super::abstract_donkey::Chest,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for MuleEntityBundle {
        fn default() -> Self {
            Self {
                mule_entity: Default::default(),
                abstract_donkey_entity: Default::default(),
                abstract_donkey_chest: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::MULE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `mule` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct MuleEntity;
}
#[allow(clippy::module_inception)]
pub mod ocelot {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `ocelot` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct OcelotEntityBundle {
        pub ocelot_entity: super::ocelot::OcelotEntity,
        pub ocelot_trusting: super::ocelot::Trusting,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for OcelotEntityBundle {
        fn default() -> Self {
            Self {
                ocelot_entity: Default::default(),
                ocelot_trusting: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::OCELOT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Trusting(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Trusting {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `ocelot` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct OcelotEntity;
}
#[allow(clippy::module_inception)]
pub mod painting {
    #![doc = "Parent class: [`abstract_decoration`][super::abstract_decoration]."]
    #[doc = "The bundle of components for spawning `painting` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PaintingEntityBundle {
        pub painting_entity: super::painting::PaintingEntity,
        pub painting_variant: super::painting::Variant,
        pub abstract_decoration_entity: super::abstract_decoration::AbstractDecorationEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PaintingEntityBundle {
        fn default() -> Self {
            Self {
                painting_entity: Default::default(),
                painting_variant: Default::default(),
                abstract_decoration_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PAINTING,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Variant(pub crate::PaintingKind);
    #[allow(clippy::derivable_impls)]
    impl Default for Variant {
        fn default() -> Self {
            Self(crate::PaintingKind::Kebab)
        }
    }
    #[doc = "Marker component for `painting` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PaintingEntity;
}
#[allow(clippy::module_inception)]
pub mod panda {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `panda` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PandaEntityBundle {
        pub panda_entity: super::panda::PandaEntity,
        pub panda_ask_for_bamboo_ticks: super::panda::AskForBambooTicks,
        pub panda_sneeze_progress: super::panda::SneezeProgress,
        pub panda_eating_ticks: super::panda::EatingTicks,
        pub panda_main_gene: super::panda::MainGene,
        pub panda_hidden_gene: super::panda::HiddenGene,
        pub panda_panda_flags: super::panda::PandaFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PandaEntityBundle {
        fn default() -> Self {
            Self {
                panda_entity: Default::default(),
                panda_ask_for_bamboo_ticks: Default::default(),
                panda_sneeze_progress: Default::default(),
                panda_eating_ticks: Default::default(),
                panda_main_gene: Default::default(),
                panda_hidden_gene: Default::default(),
                panda_panda_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PANDA,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct AskForBambooTicks(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for AskForBambooTicks {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct SneezeProgress(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for SneezeProgress {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct EatingTicks(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for EatingTicks {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct MainGene(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for MainGene {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HiddenGene(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for HiddenGene {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PandaFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for PandaFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `panda` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PandaEntity;
}
#[allow(clippy::module_inception)]
pub mod parrot {
    #![doc = "Parent class: [`tameable_shoulder`][super::tameable_shoulder]."]
    #[doc = "The bundle of components for spawning `parrot` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ParrotEntityBundle {
        pub parrot_entity: super::parrot::ParrotEntity,
        pub parrot_variant: super::parrot::Variant,
        pub tameable_shoulder_entity: super::tameable_shoulder::TameableShoulderEntity,
        pub tameable_entity: super::tameable::TameableEntity,
        pub tameable_tameable_flags: super::tameable::TameableFlags,
        pub tameable_owner_uuid: super::tameable::OwnerUuid,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ParrotEntityBundle {
        fn default() -> Self {
            Self {
                parrot_entity: Default::default(),
                parrot_variant: Default::default(),
                tameable_shoulder_entity: Default::default(),
                tameable_entity: Default::default(),
                tameable_tameable_flags: Default::default(),
                tameable_owner_uuid: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PARROT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Variant(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Variant {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `parrot` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ParrotEntity;
}
#[allow(clippy::module_inception)]
pub mod passive {
    #![doc = "Parent class: [`path_aware`][super::path_aware]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Child(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Child {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `passive` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PassiveEntity;
}
#[allow(clippy::module_inception)]
pub mod path_aware {
    #![doc = "Parent class: [`mob`][super::mob]."]
    #[doc = "Marker component for `path_aware` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PathAwareEntity;
}
#[allow(clippy::module_inception)]
pub mod patrol {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "Marker component for `patrol` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PatrolEntity;
}
#[allow(clippy::module_inception)]
pub mod persistent_projectile {
    #![doc = "Parent class: [`projectile`][super::projectile]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ProjectileFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for ProjectileFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PierceLevel(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for PierceLevel {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `persistent_projectile` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PersistentProjectileEntity;
}
#[allow(clippy::module_inception)]
pub mod phantom {
    #![doc = "Parent class: [`flying`][super::flying]."]
    #[doc = "The bundle of components for spawning `phantom` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PhantomEntityBundle {
        pub phantom_entity: super::phantom::PhantomEntity,
        pub phantom_size: super::phantom::Size,
        pub flying_entity: super::flying::FlyingEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PhantomEntityBundle {
        fn default() -> Self {
            Self {
                phantom_entity: Default::default(),
                phantom_size: Default::default(),
                flying_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PHANTOM,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Size(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Size {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `phantom` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PhantomEntity;
}
#[allow(clippy::module_inception)]
pub mod pig {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `pig` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PigEntityBundle {
        pub pig_entity: super::pig::PigEntity,
        pub pig_saddled: super::pig::Saddled,
        pub pig_boost_time: super::pig::BoostTime,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PigEntityBundle {
        fn default() -> Self {
            Self {
                pig_entity: Default::default(),
                pig_saddled: Default::default(),
                pig_boost_time: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PIG,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Saddled(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Saddled {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BoostTime(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for BoostTime {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `pig` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PigEntity;
}
#[allow(clippy::module_inception)]
pub mod piglin_brute {
    #![doc = "Parent class: [`abstract_piglin`][super::abstract_piglin]."]
    #[doc = "The bundle of components for spawning `piglin_brute` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PiglinBruteEntityBundle {
        pub piglin_brute_entity: super::piglin_brute::PiglinBruteEntity,
        pub abstract_piglin_entity: super::abstract_piglin::AbstractPiglinEntity,
        pub abstract_piglin_immune_to_zombification: super::abstract_piglin::ImmuneToZombification,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PiglinBruteEntityBundle {
        fn default() -> Self {
            Self {
                piglin_brute_entity: Default::default(),
                abstract_piglin_entity: Default::default(),
                abstract_piglin_immune_to_zombification: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PIGLIN_BRUTE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `piglin_brute` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PiglinBruteEntity;
}
#[allow(clippy::module_inception)]
pub mod piglin {
    #![doc = "Parent class: [`abstract_piglin`][super::abstract_piglin]."]
    #[doc = "The bundle of components for spawning `piglin` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PiglinEntityBundle {
        pub piglin_entity: super::piglin::PiglinEntity,
        pub piglin_baby: super::piglin::Baby,
        pub piglin_charging: super::piglin::Charging,
        pub piglin_dancing: super::piglin::Dancing,
        pub abstract_piglin_entity: super::abstract_piglin::AbstractPiglinEntity,
        pub abstract_piglin_immune_to_zombification: super::abstract_piglin::ImmuneToZombification,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PiglinEntityBundle {
        fn default() -> Self {
            Self {
                piglin_entity: Default::default(),
                piglin_baby: Default::default(),
                piglin_charging: Default::default(),
                piglin_dancing: Default::default(),
                abstract_piglin_entity: Default::default(),
                abstract_piglin_immune_to_zombification: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PIGLIN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Baby(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Baby {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Charging(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Charging {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Dancing(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Dancing {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `piglin` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PiglinEntity;
}
#[allow(clippy::module_inception)]
pub mod pillager {
    #![doc = "Parent class: [`illager`][super::illager]."]
    #[doc = "The bundle of components for spawning `pillager` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PillagerEntityBundle {
        pub pillager_entity: super::pillager::PillagerEntity,
        pub pillager_charging: super::pillager::Charging,
        pub illager_entity: super::illager::IllagerEntity,
        pub raider_entity: super::raider::RaiderEntity,
        pub raider_celebrating: super::raider::Celebrating,
        pub patrol_entity: super::patrol::PatrolEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PillagerEntityBundle {
        fn default() -> Self {
            Self {
                pillager_entity: Default::default(),
                pillager_charging: Default::default(),
                illager_entity: Default::default(),
                raider_entity: Default::default(),
                raider_celebrating: Default::default(),
                patrol_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PILLAGER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Charging(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Charging {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `pillager` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PillagerEntity;
}
#[allow(clippy::module_inception)]
pub mod player {
    #![doc = "Parent class: [`living`][super::living]."]
    #[doc = "The bundle of components for spawning `player` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PlayerEntityBundle {
        pub player_entity: super::player::PlayerEntity,
        pub player_absorption_amount: super::player::AbsorptionAmount,
        pub player_score: super::player::Score,
        pub player_player_model_parts: super::player::PlayerModelParts,
        pub player_main_arm: super::player::MainArm,
        pub player_left_shoulder_entity: super::player::LeftShoulderEntity,
        pub player_right_shoulder_entity: super::player::RightShoulderEntity,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PlayerEntityBundle {
        fn default() -> Self {
            Self {
                player_entity: Default::default(),
                player_absorption_amount: Default::default(),
                player_score: Default::default(),
                player_player_model_parts: Default::default(),
                player_main_arm: Default::default(),
                player_left_shoulder_entity: Default::default(),
                player_right_shoulder_entity: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PLAYER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct AbsorptionAmount(pub f32);
    #[allow(clippy::derivable_impls)]
    impl Default for AbsorptionAmount {
        fn default() -> Self {
            Self(0f32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Score(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Score {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PlayerModelParts(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for PlayerModelParts {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct MainArm(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for MainArm {
        fn default() -> Self {
            Self(1i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LeftShoulderEntity(pub valence_nbt::Compound);
    #[allow(clippy::derivable_impls)]
    impl Default for LeftShoulderEntity {
        fn default() -> Self {
            Self(valence_nbt::Compound::default())
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct RightShoulderEntity(pub valence_nbt::Compound);
    #[allow(clippy::derivable_impls)]
    impl Default for RightShoulderEntity {
        fn default() -> Self {
            Self(valence_nbt::Compound::default())
        }
    }
    #[doc = "Marker component for `player` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PlayerEntity;
}
#[allow(clippy::module_inception)]
pub mod polar_bear {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `polar_bear` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PolarBearEntityBundle {
        pub polar_bear_entity: super::polar_bear::PolarBearEntity,
        pub polar_bear_warning: super::polar_bear::Warning,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PolarBearEntityBundle {
        fn default() -> Self {
            Self {
                polar_bear_entity: Default::default(),
                polar_bear_warning: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::POLAR_BEAR,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Warning(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Warning {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `polar_bear` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PolarBearEntity;
}
#[allow(clippy::module_inception)]
pub mod potion {
    #![doc = "Parent class: [`thrown_item`][super::thrown_item]."]
    #[doc = "The bundle of components for spawning `potion` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PotionEntityBundle {
        pub potion_entity: super::potion::PotionEntity,
        pub thrown_item_entity: super::thrown_item::ThrownItemEntity,
        pub thrown_item_item: super::thrown_item::Item,
        pub thrown_entity: super::thrown::ThrownEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PotionEntityBundle {
        fn default() -> Self {
            Self {
                potion_entity: Default::default(),
                thrown_item_entity: Default::default(),
                thrown_item_item: Default::default(),
                thrown_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::POTION,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `potion` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PotionEntity;
}
#[allow(clippy::module_inception)]
pub mod projectile {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "Marker component for `projectile` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ProjectileEntity;
}
#[allow(clippy::module_inception)]
pub mod pufferfish {
    #![doc = "Parent class: [`fish`][super::fish]."]
    #[doc = "The bundle of components for spawning `pufferfish` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct PufferfishEntityBundle {
        pub pufferfish_entity: super::pufferfish::PufferfishEntity,
        pub pufferfish_puff_state: super::pufferfish::PuffState,
        pub fish_entity: super::fish::FishEntity,
        pub fish_from_bucket: super::fish::FromBucket,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for PufferfishEntityBundle {
        fn default() -> Self {
            Self {
                pufferfish_entity: Default::default(),
                pufferfish_puff_state: Default::default(),
                fish_entity: Default::default(),
                fish_from_bucket: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::PUFFERFISH,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PuffState(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for PuffState {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `pufferfish` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct PufferfishEntity;
}
#[allow(clippy::module_inception)]
pub mod rabbit {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `rabbit` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct RabbitEntityBundle {
        pub rabbit_entity: super::rabbit::RabbitEntity,
        pub rabbit_rabbit_type: super::rabbit::RabbitType,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for RabbitEntityBundle {
        fn default() -> Self {
            Self {
                rabbit_entity: Default::default(),
                rabbit_rabbit_type: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::RABBIT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct RabbitType(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for RabbitType {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `rabbit` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct RabbitEntity;
}
#[allow(clippy::module_inception)]
pub mod raider {
    #![doc = "Parent class: [`patrol`][super::patrol]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Celebrating(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Celebrating {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `raider` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct RaiderEntity;
}
#[allow(clippy::module_inception)]
pub mod ravager {
    #![doc = "Parent class: [`raider`][super::raider]."]
    #[doc = "The bundle of components for spawning `ravager` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct RavagerEntityBundle {
        pub ravager_entity: super::ravager::RavagerEntity,
        pub raider_entity: super::raider::RaiderEntity,
        pub raider_celebrating: super::raider::Celebrating,
        pub patrol_entity: super::patrol::PatrolEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for RavagerEntityBundle {
        fn default() -> Self {
            Self {
                ravager_entity: Default::default(),
                raider_entity: Default::default(),
                raider_celebrating: Default::default(),
                patrol_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::RAVAGER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `ravager` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct RavagerEntity;
}
#[allow(clippy::module_inception)]
pub mod salmon {
    #![doc = "Parent class: [`schooling_fish`][super::schooling_fish]."]
    #[doc = "The bundle of components for spawning `salmon` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SalmonEntityBundle {
        pub salmon_entity: super::salmon::SalmonEntity,
        pub schooling_fish_entity: super::schooling_fish::SchoolingFishEntity,
        pub fish_entity: super::fish::FishEntity,
        pub fish_from_bucket: super::fish::FromBucket,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SalmonEntityBundle {
        fn default() -> Self {
            Self {
                salmon_entity: Default::default(),
                schooling_fish_entity: Default::default(),
                fish_entity: Default::default(),
                fish_from_bucket: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SALMON,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `salmon` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SalmonEntity;
}
#[allow(clippy::module_inception)]
pub mod schooling_fish {
    #![doc = "Parent class: [`fish`][super::fish]."]
    #[doc = "Marker component for `schooling_fish` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SchoolingFishEntity;
}
#[allow(clippy::module_inception)]
pub mod sheep {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `sheep` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SheepEntityBundle {
        pub sheep_entity: super::sheep::SheepEntity,
        pub sheep_color: super::sheep::Color,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SheepEntityBundle {
        fn default() -> Self {
            Self {
                sheep_entity: Default::default(),
                sheep_color: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SHEEP,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Color(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for Color {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `sheep` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SheepEntity;
}
#[allow(clippy::module_inception)]
pub mod shulker_bullet {
    #![doc = "Parent class: [`projectile`][super::projectile]."]
    #[doc = "The bundle of components for spawning `shulker_bullet` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ShulkerBulletEntityBundle {
        pub shulker_bullet_entity: super::shulker_bullet::ShulkerBulletEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ShulkerBulletEntityBundle {
        fn default() -> Self {
            Self {
                shulker_bullet_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SHULKER_BULLET,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `shulker_bullet` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ShulkerBulletEntity;
}
#[allow(clippy::module_inception)]
pub mod shulker {
    #![doc = "Parent class: [`golem`][super::golem]."]
    #[doc = "The bundle of components for spawning `shulker` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ShulkerEntityBundle {
        pub shulker_entity: super::shulker::ShulkerEntity,
        pub shulker_attached_face: super::shulker::AttachedFace,
        pub shulker_peek_amount: super::shulker::PeekAmount,
        pub shulker_color: super::shulker::Color,
        pub golem_entity: super::golem::GolemEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ShulkerEntityBundle {
        fn default() -> Self {
            Self {
                shulker_entity: Default::default(),
                shulker_attached_face: Default::default(),
                shulker_peek_amount: Default::default(),
                shulker_color: Default::default(),
                golem_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SHULKER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct AttachedFace(pub valence_protocol::Direction);
    #[allow(clippy::derivable_impls)]
    impl Default for AttachedFace {
        fn default() -> Self {
            Self(valence_protocol::Direction::Down)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct PeekAmount(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for PeekAmount {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Color(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for Color {
        fn default() -> Self {
            Self(16i8)
        }
    }
    #[doc = "Marker component for `shulker` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ShulkerEntity;
}
#[allow(clippy::module_inception)]
pub mod silverfish {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `silverfish` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SilverfishEntityBundle {
        pub silverfish_entity: super::silverfish::SilverfishEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SilverfishEntityBundle {
        fn default() -> Self {
            Self {
                silverfish_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SILVERFISH,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `silverfish` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SilverfishEntity;
}
#[allow(clippy::module_inception)]
pub mod skeleton {
    #![doc = "Parent class: [`abstract_skeleton`][super::abstract_skeleton]."]
    #[doc = "The bundle of components for spawning `skeleton` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SkeletonEntityBundle {
        pub skeleton_entity: super::skeleton::SkeletonEntity,
        pub skeleton_converting: super::skeleton::Converting,
        pub abstract_skeleton_entity: super::abstract_skeleton::AbstractSkeletonEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SkeletonEntityBundle {
        fn default() -> Self {
            Self {
                skeleton_entity: Default::default(),
                skeleton_converting: Default::default(),
                abstract_skeleton_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SKELETON,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Converting(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Converting {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `skeleton` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SkeletonEntity;
}
#[allow(clippy::module_inception)]
pub mod skeleton_horse {
    #![doc = "Parent class: [`abstract_horse`][super::abstract_horse]."]
    #[doc = "The bundle of components for spawning `skeleton_horse` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SkeletonHorseEntityBundle {
        pub skeleton_horse_entity: super::skeleton_horse::SkeletonHorseEntity,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SkeletonHorseEntityBundle {
        fn default() -> Self {
            Self {
                skeleton_horse_entity: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SKELETON_HORSE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `skeleton_horse` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SkeletonHorseEntity;
}
#[allow(clippy::module_inception)]
pub mod slime {
    #![doc = "Parent class: [`mob`][super::mob]."]
    #[doc = "The bundle of components for spawning `slime` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SlimeEntityBundle {
        pub slime_entity: super::slime::SlimeEntity,
        pub slime_slime_size: super::slime::SlimeSize,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SlimeEntityBundle {
        fn default() -> Self {
            Self {
                slime_entity: Default::default(),
                slime_slime_size: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SLIME,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct SlimeSize(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for SlimeSize {
        fn default() -> Self {
            Self(1i32)
        }
    }
    #[doc = "Marker component for `slime` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SlimeEntity;
}
#[allow(clippy::module_inception)]
pub mod small_fireball {
    #![doc = "Parent class: [`abstract_fireball`][super::abstract_fireball]."]
    #[doc = "The bundle of components for spawning `small_fireball` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SmallFireballEntityBundle {
        pub small_fireball_entity: super::small_fireball::SmallFireballEntity,
        pub abstract_fireball_entity: super::abstract_fireball::AbstractFireballEntity,
        pub abstract_fireball_item: super::abstract_fireball::Item,
        pub explosive_projectile_entity: super::explosive_projectile::ExplosiveProjectileEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SmallFireballEntityBundle {
        fn default() -> Self {
            Self {
                small_fireball_entity: Default::default(),
                abstract_fireball_entity: Default::default(),
                abstract_fireball_item: Default::default(),
                explosive_projectile_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SMALL_FIREBALL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `small_fireball` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SmallFireballEntity;
}
#[allow(clippy::module_inception)]
pub mod sniffer {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `sniffer` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SnifferEntityBundle {
        pub sniffer_entity: super::sniffer::SnifferEntity,
        pub sniffer_state: super::sniffer::State,
        pub sniffer_finish_dig_time: super::sniffer::FinishDigTime,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SnifferEntityBundle {
        fn default() -> Self {
            Self {
                sniffer_entity: Default::default(),
                sniffer_state: Default::default(),
                sniffer_finish_dig_time: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SNIFFER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct State(pub crate::SnifferState);
    #[allow(clippy::derivable_impls)]
    impl Default for State {
        fn default() -> Self {
            Self(crate::SnifferState::Idling)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct FinishDigTime(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for FinishDigTime {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `sniffer` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SnifferEntity;
}
#[allow(clippy::module_inception)]
pub mod snow_golem {
    #![doc = "Parent class: [`golem`][super::golem]."]
    #[doc = "The bundle of components for spawning `snow_golem` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SnowGolemEntityBundle {
        pub snow_golem_entity: super::snow_golem::SnowGolemEntity,
        pub snow_golem_snow_golem_flags: super::snow_golem::SnowGolemFlags,
        pub golem_entity: super::golem::GolemEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SnowGolemEntityBundle {
        fn default() -> Self {
            Self {
                snow_golem_entity: Default::default(),
                snow_golem_snow_golem_flags: Default::default(),
                golem_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SNOW_GOLEM,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct SnowGolemFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for SnowGolemFlags {
        fn default() -> Self {
            Self(16i8)
        }
    }
    #[doc = "Marker component for `snow_golem` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SnowGolemEntity;
}
#[allow(clippy::module_inception)]
pub mod snowball {
    #![doc = "Parent class: [`thrown_item`][super::thrown_item]."]
    #[doc = "The bundle of components for spawning `snowball` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SnowballEntityBundle {
        pub snowball_entity: super::snowball::SnowballEntity,
        pub thrown_item_entity: super::thrown_item::ThrownItemEntity,
        pub thrown_item_item: super::thrown_item::Item,
        pub thrown_entity: super::thrown::ThrownEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SnowballEntityBundle {
        fn default() -> Self {
            Self {
                snowball_entity: Default::default(),
                thrown_item_entity: Default::default(),
                thrown_item_item: Default::default(),
                thrown_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SNOWBALL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `snowball` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SnowballEntity;
}
#[allow(clippy::module_inception)]
pub mod spawner_minecart {
    #![doc = "Parent class: [`abstract_minecart`][super::abstract_minecart]."]
    #[doc = "The bundle of components for spawning `spawner_minecart` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SpawnerMinecartEntityBundle {
        pub spawner_minecart_entity: super::spawner_minecart::SpawnerMinecartEntity,
        pub abstract_minecart_entity: super::abstract_minecart::AbstractMinecartEntity,
        pub abstract_minecart_damage_wobble_ticks: super::abstract_minecart::DamageWobbleTicks,
        pub abstract_minecart_damage_wobble_side: super::abstract_minecart::DamageWobbleSide,
        pub abstract_minecart_damage_wobble_strength:
            super::abstract_minecart::DamageWobbleStrength,
        pub abstract_minecart_custom_block_id: super::abstract_minecart::CustomBlockId,
        pub abstract_minecart_custom_block_offset: super::abstract_minecart::CustomBlockOffset,
        pub abstract_minecart_custom_block_present: super::abstract_minecart::CustomBlockPresent,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SpawnerMinecartEntityBundle {
        fn default() -> Self {
            Self {
                spawner_minecart_entity: Default::default(),
                abstract_minecart_entity: Default::default(),
                abstract_minecart_damage_wobble_ticks: Default::default(),
                abstract_minecart_damage_wobble_side: Default::default(),
                abstract_minecart_damage_wobble_strength: Default::default(),
                abstract_minecart_custom_block_id: Default::default(),
                abstract_minecart_custom_block_offset: Default::default(),
                abstract_minecart_custom_block_present: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SPAWNER_MINECART,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `spawner_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SpawnerMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod spectral_arrow {
    #![doc = "Parent class: [`persistent_projectile`][super::persistent_projectile]."]
    #[doc = "The bundle of components for spawning `spectral_arrow` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SpectralArrowEntityBundle {
        pub spectral_arrow_entity: super::spectral_arrow::SpectralArrowEntity,
        pub persistent_projectile_entity: super::persistent_projectile::PersistentProjectileEntity,
        pub persistent_projectile_projectile_flags: super::persistent_projectile::ProjectileFlags,
        pub persistent_projectile_pierce_level: super::persistent_projectile::PierceLevel,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SpectralArrowEntityBundle {
        fn default() -> Self {
            Self {
                spectral_arrow_entity: Default::default(),
                persistent_projectile_entity: Default::default(),
                persistent_projectile_projectile_flags: Default::default(),
                persistent_projectile_pierce_level: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SPECTRAL_ARROW,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `spectral_arrow` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SpectralArrowEntity;
}
#[allow(clippy::module_inception)]
pub mod spellcasting_illager {
    #![doc = "Parent class: [`illager`][super::illager]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Spell(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for Spell {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `spellcasting_illager` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SpellcastingIllagerEntity;
}
#[allow(clippy::module_inception)]
pub mod spider {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `spider` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SpiderEntityBundle {
        pub spider_entity: super::spider::SpiderEntity,
        pub spider_spider_flags: super::spider::SpiderFlags,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SpiderEntityBundle {
        fn default() -> Self {
            Self {
                spider_entity: Default::default(),
                spider_spider_flags: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SPIDER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct SpiderFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for SpiderFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `spider` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SpiderEntity;
}
#[allow(clippy::module_inception)]
pub mod squid {
    #![doc = "Parent class: [`water_creature`][super::water_creature]."]
    #[doc = "The bundle of components for spawning `squid` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct SquidEntityBundle {
        pub squid_entity: super::squid::SquidEntity,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for SquidEntityBundle {
        fn default() -> Self {
            Self {
                squid_entity: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::SQUID,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `squid` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct SquidEntity;
}
#[allow(clippy::module_inception)]
pub mod storage_minecart {
    #![doc = "Parent class: [`abstract_minecart`][super::abstract_minecart]."]
    #[doc = "Marker component for `storage_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct StorageMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod stray {
    #![doc = "Parent class: [`abstract_skeleton`][super::abstract_skeleton]."]
    #[doc = "The bundle of components for spawning `stray` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct StrayEntityBundle {
        pub stray_entity: super::stray::StrayEntity,
        pub abstract_skeleton_entity: super::abstract_skeleton::AbstractSkeletonEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for StrayEntityBundle {
        fn default() -> Self {
            Self {
                stray_entity: Default::default(),
                abstract_skeleton_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::STRAY,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `stray` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct StrayEntity;
}
#[allow(clippy::module_inception)]
pub mod strider {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `strider` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct StriderEntityBundle {
        pub strider_entity: super::strider::StriderEntity,
        pub strider_boost_time: super::strider::BoostTime,
        pub strider_cold: super::strider::Cold,
        pub strider_saddled: super::strider::Saddled,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for StriderEntityBundle {
        fn default() -> Self {
            Self {
                strider_entity: Default::default(),
                strider_boost_time: Default::default(),
                strider_cold: Default::default(),
                strider_saddled: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::STRIDER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct BoostTime(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for BoostTime {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Cold(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Cold {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Saddled(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Saddled {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `strider` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct StriderEntity;
}
#[allow(clippy::module_inception)]
pub mod tadpole {
    #![doc = "Parent class: [`fish`][super::fish]."]
    #[doc = "The bundle of components for spawning `tadpole` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TadpoleEntityBundle {
        pub tadpole_entity: super::tadpole::TadpoleEntity,
        pub fish_entity: super::fish::FishEntity,
        pub fish_from_bucket: super::fish::FromBucket,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TadpoleEntityBundle {
        fn default() -> Self {
            Self {
                tadpole_entity: Default::default(),
                fish_entity: Default::default(),
                fish_from_bucket: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TADPOLE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `tadpole` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TadpoleEntity;
}
#[allow(clippy::module_inception)]
pub mod tameable {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TameableFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for TameableFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct OwnerUuid(pub Option<::uuid::Uuid>);
    #[allow(clippy::derivable_impls)]
    impl Default for OwnerUuid {
        fn default() -> Self {
            Self(None)
        }
    }
    #[doc = "Marker component for `tameable` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TameableEntity;
}
#[allow(clippy::module_inception)]
pub mod tameable_shoulder {
    #![doc = "Parent class: [`tameable`][super::tameable]."]
    #[doc = "Marker component for `tameable_shoulder` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TameableShoulderEntity;
}
#[allow(clippy::module_inception)]
pub mod text_display {
    #![doc = "Parent class: [`display`][super::display]."]
    #[doc = "The bundle of components for spawning `text_display` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TextDisplayEntityBundle {
        pub text_display_entity: super::text_display::TextDisplayEntity,
        pub text_display_text: super::text_display::Text,
        pub text_display_line_width: super::text_display::LineWidth,
        pub text_display_background: super::text_display::Background,
        pub text_display_text_opacity: super::text_display::TextOpacity,
        pub text_display_text_display_flags: super::text_display::TextDisplayFlags,
        pub display_entity: super::display::DisplayEntity,
        pub display_start_interpolation: super::display::StartInterpolation,
        pub display_interpolation_duration: super::display::InterpolationDuration,
        pub display_translation: super::display::Translation,
        pub display_scale: super::display::Scale,
        pub display_left_rotation: super::display::LeftRotation,
        pub display_right_rotation: super::display::RightRotation,
        pub display_billboard: super::display::Billboard,
        pub display_brightness: super::display::Brightness,
        pub display_view_range: super::display::ViewRange,
        pub display_shadow_radius: super::display::ShadowRadius,
        pub display_shadow_strength: super::display::ShadowStrength,
        pub display_width: super::display::Width,
        pub display_height: super::display::Height,
        pub display_glow_color_override: super::display::GlowColorOverride,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TextDisplayEntityBundle {
        fn default() -> Self {
            Self {
                text_display_entity: Default::default(),
                text_display_text: Default::default(),
                text_display_line_width: Default::default(),
                text_display_background: Default::default(),
                text_display_text_opacity: Default::default(),
                text_display_text_display_flags: Default::default(),
                display_entity: Default::default(),
                display_start_interpolation: Default::default(),
                display_interpolation_duration: Default::default(),
                display_translation: Default::default(),
                display_scale: Default::default(),
                display_left_rotation: Default::default(),
                display_right_rotation: Default::default(),
                display_billboard: Default::default(),
                display_brightness: Default::default(),
                display_view_range: Default::default(),
                display_shadow_radius: Default::default(),
                display_shadow_strength: Default::default(),
                display_width: Default::default(),
                display_height: Default::default(),
                display_glow_color_override: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TEXT_DISPLAY,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Text(pub valence_protocol::Text);
    #[allow(clippy::derivable_impls)]
    impl Default for Text {
        fn default() -> Self {
            Self(valence_protocol::Text::default())
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LineWidth(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for LineWidth {
        fn default() -> Self {
            Self(200i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Background(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Background {
        fn default() -> Self {
            Self(1073741824i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TextOpacity(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for TextOpacity {
        fn default() -> Self {
            Self(-1i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TextDisplayFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for TextDisplayFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `text_display` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TextDisplayEntity;
}
#[allow(clippy::module_inception)]
pub mod thrown {
    #![doc = "Parent class: [`projectile`][super::projectile]."]
    #[doc = "Marker component for `thrown` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ThrownEntity;
}
#[allow(clippy::module_inception)]
pub mod thrown_item {
    #![doc = "Parent class: [`thrown`][super::thrown]."]
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Item(pub valence_protocol::ItemStack);
    #[allow(clippy::derivable_impls)]
    impl Default for Item {
        fn default() -> Self {
            Self(valence_protocol::ItemStack::default())
        }
    }
    #[doc = "Marker component for `thrown_item` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ThrownItemEntity;
}
#[allow(clippy::module_inception)]
pub mod tnt {
    #![doc = "Parent class: [`entity`][super::entity]."]
    #[doc = "The bundle of components for spawning `tnt` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TntEntityBundle {
        pub tnt_entity: super::tnt::TntEntity,
        pub tnt_fuse: super::tnt::Fuse,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TntEntityBundle {
        fn default() -> Self {
            Self {
                tnt_entity: Default::default(),
                tnt_fuse: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TNT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Fuse(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Fuse {
        fn default() -> Self {
            Self(80i32)
        }
    }
    #[doc = "Marker component for `tnt` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TntEntity;
}
#[allow(clippy::module_inception)]
pub mod tnt_minecart {
    #![doc = "Parent class: [`abstract_minecart`][super::abstract_minecart]."]
    #[doc = "The bundle of components for spawning `tnt_minecart` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TntMinecartEntityBundle {
        pub tnt_minecart_entity: super::tnt_minecart::TntMinecartEntity,
        pub abstract_minecart_entity: super::abstract_minecart::AbstractMinecartEntity,
        pub abstract_minecart_damage_wobble_ticks: super::abstract_minecart::DamageWobbleTicks,
        pub abstract_minecart_damage_wobble_side: super::abstract_minecart::DamageWobbleSide,
        pub abstract_minecart_damage_wobble_strength:
            super::abstract_minecart::DamageWobbleStrength,
        pub abstract_minecart_custom_block_id: super::abstract_minecart::CustomBlockId,
        pub abstract_minecart_custom_block_offset: super::abstract_minecart::CustomBlockOffset,
        pub abstract_minecart_custom_block_present: super::abstract_minecart::CustomBlockPresent,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TntMinecartEntityBundle {
        fn default() -> Self {
            Self {
                tnt_minecart_entity: Default::default(),
                abstract_minecart_entity: Default::default(),
                abstract_minecart_damage_wobble_ticks: Default::default(),
                abstract_minecart_damage_wobble_side: Default::default(),
                abstract_minecart_damage_wobble_strength: Default::default(),
                abstract_minecart_custom_block_id: Default::default(),
                abstract_minecart_custom_block_offset: Default::default(),
                abstract_minecart_custom_block_present: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TNT_MINECART,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `tnt_minecart` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TntMinecartEntity;
}
#[allow(clippy::module_inception)]
pub mod trader_llama {
    #![doc = "Parent class: [`llama`][super::llama]."]
    #[doc = "The bundle of components for spawning `trader_llama` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TraderLlamaEntityBundle {
        pub trader_llama_entity: super::trader_llama::TraderLlamaEntity,
        pub llama_entity: super::llama::LlamaEntity,
        pub llama_strength: super::llama::Strength,
        pub llama_carpet_color: super::llama::CarpetColor,
        pub llama_variant: super::llama::Variant,
        pub abstract_donkey_entity: super::abstract_donkey::AbstractDonkeyEntity,
        pub abstract_donkey_chest: super::abstract_donkey::Chest,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TraderLlamaEntityBundle {
        fn default() -> Self {
            Self {
                trader_llama_entity: Default::default(),
                llama_entity: Default::default(),
                llama_strength: Default::default(),
                llama_carpet_color: Default::default(),
                llama_variant: Default::default(),
                abstract_donkey_entity: Default::default(),
                abstract_donkey_chest: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TRADER_LLAMA,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `trader_llama` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TraderLlamaEntity;
}
#[allow(clippy::module_inception)]
pub mod trident {
    #![doc = "Parent class: [`persistent_projectile`][super::persistent_projectile]."]
    #[doc = "The bundle of components for spawning `trident` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TridentEntityBundle {
        pub trident_entity: super::trident::TridentEntity,
        pub trident_loyalty: super::trident::Loyalty,
        pub trident_enchanted: super::trident::Enchanted,
        pub persistent_projectile_entity: super::persistent_projectile::PersistentProjectileEntity,
        pub persistent_projectile_projectile_flags: super::persistent_projectile::ProjectileFlags,
        pub persistent_projectile_pierce_level: super::persistent_projectile::PierceLevel,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TridentEntityBundle {
        fn default() -> Self {
            Self {
                trident_entity: Default::default(),
                trident_loyalty: Default::default(),
                trident_enchanted: Default::default(),
                persistent_projectile_entity: Default::default(),
                persistent_projectile_projectile_flags: Default::default(),
                persistent_projectile_pierce_level: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TRIDENT,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Loyalty(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for Loyalty {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Enchanted(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Enchanted {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `trident` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TridentEntity;
}
#[allow(clippy::module_inception)]
pub mod tropical_fish {
    #![doc = "Parent class: [`schooling_fish`][super::schooling_fish]."]
    #[doc = "The bundle of components for spawning `tropical_fish` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TropicalFishEntityBundle {
        pub tropical_fish_entity: super::tropical_fish::TropicalFishEntity,
        pub tropical_fish_variant: super::tropical_fish::Variant,
        pub schooling_fish_entity: super::schooling_fish::SchoolingFishEntity,
        pub fish_entity: super::fish::FishEntity,
        pub fish_from_bucket: super::fish::FromBucket,
        pub water_creature_entity: super::water_creature::WaterCreatureEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TropicalFishEntityBundle {
        fn default() -> Self {
            Self {
                tropical_fish_entity: Default::default(),
                tropical_fish_variant: Default::default(),
                schooling_fish_entity: Default::default(),
                fish_entity: Default::default(),
                fish_from_bucket: Default::default(),
                water_creature_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TROPICAL_FISH,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Variant(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Variant {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `tropical_fish` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TropicalFishEntity;
}
#[allow(clippy::module_inception)]
pub mod turtle {
    #![doc = "Parent class: [`animal`][super::animal]."]
    #[doc = "The bundle of components for spawning `turtle` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct TurtleEntityBundle {
        pub turtle_entity: super::turtle::TurtleEntity,
        pub turtle_home_pos: super::turtle::HomePos,
        pub turtle_has_egg: super::turtle::HasEgg,
        pub turtle_digging_sand: super::turtle::DiggingSand,
        pub turtle_travel_pos: super::turtle::TravelPos,
        pub turtle_land_bound: super::turtle::LandBound,
        pub turtle_actively_traveling: super::turtle::ActivelyTraveling,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for TurtleEntityBundle {
        fn default() -> Self {
            Self {
                turtle_entity: Default::default(),
                turtle_home_pos: Default::default(),
                turtle_has_egg: Default::default(),
                turtle_digging_sand: Default::default(),
                turtle_travel_pos: Default::default(),
                turtle_land_bound: Default::default(),
                turtle_actively_traveling: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::TURTLE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HomePos(pub valence_protocol::BlockPos);
    #[allow(clippy::derivable_impls)]
    impl Default for HomePos {
        fn default() -> Self {
            Self(valence_protocol::BlockPos {
                x: 0i32,
                y: 0i32,
                z: 0i32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct HasEgg(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for HasEgg {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct DiggingSand(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for DiggingSand {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TravelPos(pub valence_protocol::BlockPos);
    #[allow(clippy::derivable_impls)]
    impl Default for TravelPos {
        fn default() -> Self {
            Self(valence_protocol::BlockPos {
                x: 0i32,
                y: 0i32,
                z: 0i32,
            })
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct LandBound(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for LandBound {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ActivelyTraveling(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for ActivelyTraveling {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `turtle` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct TurtleEntity;
}
#[allow(clippy::module_inception)]
pub mod vex {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `vex` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct VexEntityBundle {
        pub vex_entity: super::vex::VexEntity,
        pub vex_vex_flags: super::vex::VexFlags,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for VexEntityBundle {
        fn default() -> Self {
            Self {
                vex_entity: Default::default(),
                vex_vex_flags: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::VEX,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct VexFlags(pub i8);
    #[allow(clippy::derivable_impls)]
    impl Default for VexFlags {
        fn default() -> Self {
            Self(0i8)
        }
    }
    #[doc = "Marker component for `vex` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct VexEntity;
}
#[allow(clippy::module_inception)]
pub mod villager {
    #![doc = "Parent class: [`merchant`][super::merchant]."]
    #[doc = "The bundle of components for spawning `villager` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct VillagerEntityBundle {
        pub villager_entity: super::villager::VillagerEntity,
        pub villager_villager_data: super::villager::VillagerData,
        pub merchant_entity: super::merchant::MerchantEntity,
        pub merchant_head_rolling_time_left: super::merchant::HeadRollingTimeLeft,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for VillagerEntityBundle {
        fn default() -> Self {
            Self {
                villager_entity: Default::default(),
                villager_villager_data: Default::default(),
                merchant_entity: Default::default(),
                merchant_head_rolling_time_left: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::VILLAGER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct VillagerData(pub crate::VillagerData);
    #[allow(clippy::derivable_impls)]
    impl Default for VillagerData {
        fn default() -> Self {
            Self(crate::VillagerData {
                kind: crate::VillagerKind::Plains,
                profession: crate::VillagerProfession::None,
                level: 1i32,
            })
        }
    }
    #[doc = "Marker component for `villager` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct VillagerEntity;
}
#[allow(clippy::module_inception)]
pub mod vindicator {
    #![doc = "Parent class: [`illager`][super::illager]."]
    #[doc = "The bundle of components for spawning `vindicator` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct VindicatorEntityBundle {
        pub vindicator_entity: super::vindicator::VindicatorEntity,
        pub illager_entity: super::illager::IllagerEntity,
        pub raider_entity: super::raider::RaiderEntity,
        pub raider_celebrating: super::raider::Celebrating,
        pub patrol_entity: super::patrol::PatrolEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for VindicatorEntityBundle {
        fn default() -> Self {
            Self {
                vindicator_entity: Default::default(),
                illager_entity: Default::default(),
                raider_entity: Default::default(),
                raider_celebrating: Default::default(),
                patrol_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::VINDICATOR,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `vindicator` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct VindicatorEntity;
}
#[allow(clippy::module_inception)]
pub mod wandering_trader {
    #![doc = "Parent class: [`merchant`][super::merchant]."]
    #[doc = "The bundle of components for spawning `wandering_trader` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct WanderingTraderEntityBundle {
        pub wandering_trader_entity: super::wandering_trader::WanderingTraderEntity,
        pub merchant_entity: super::merchant::MerchantEntity,
        pub merchant_head_rolling_time_left: super::merchant::HeadRollingTimeLeft,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for WanderingTraderEntityBundle {
        fn default() -> Self {
            Self {
                wandering_trader_entity: Default::default(),
                merchant_entity: Default::default(),
                merchant_head_rolling_time_left: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::WANDERING_TRADER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `wandering_trader` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WanderingTraderEntity;
}
#[allow(clippy::module_inception)]
pub mod warden {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `warden` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct WardenEntityBundle {
        pub warden_entity: super::warden::WardenEntity,
        pub warden_anger: super::warden::Anger,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for WardenEntityBundle {
        fn default() -> Self {
            Self {
                warden_entity: Default::default(),
                warden_anger: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::WARDEN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Anger(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for Anger {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `warden` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WardenEntity;
}
#[allow(clippy::module_inception)]
pub mod water_creature {
    #![doc = "Parent class: [`path_aware`][super::path_aware]."]
    #[doc = "Marker component for `water_creature` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WaterCreatureEntity;
}
#[allow(clippy::module_inception)]
pub mod witch {
    #![doc = "Parent class: [`raider`][super::raider]."]
    #[doc = "The bundle of components for spawning `witch` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct WitchEntityBundle {
        pub witch_entity: super::witch::WitchEntity,
        pub witch_drinking: super::witch::Drinking,
        pub raider_entity: super::raider::RaiderEntity,
        pub raider_celebrating: super::raider::Celebrating,
        pub patrol_entity: super::patrol::PatrolEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for WitchEntityBundle {
        fn default() -> Self {
            Self {
                witch_entity: Default::default(),
                witch_drinking: Default::default(),
                raider_entity: Default::default(),
                raider_celebrating: Default::default(),
                patrol_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::WITCH,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Drinking(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Drinking {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `witch` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WitchEntity;
}
#[allow(clippy::module_inception)]
pub mod wither {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `wither` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct WitherEntityBundle {
        pub wither_entity: super::wither::WitherEntity,
        pub wither_tracked_entity_id_1: super::wither::TrackedEntityId1,
        pub wither_tracked_entity_id_2: super::wither::TrackedEntityId2,
        pub wither_tracked_entity_id_3: super::wither::TrackedEntityId3,
        pub wither_invul_timer: super::wither::InvulTimer,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for WitherEntityBundle {
        fn default() -> Self {
            Self {
                wither_entity: Default::default(),
                wither_tracked_entity_id_1: Default::default(),
                wither_tracked_entity_id_2: Default::default(),
                wither_tracked_entity_id_3: Default::default(),
                wither_invul_timer: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::WITHER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackedEntityId1(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackedEntityId1 {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackedEntityId2(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackedEntityId2 {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct TrackedEntityId3(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for TrackedEntityId3 {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct InvulTimer(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for InvulTimer {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `wither` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WitherEntity;
}
#[allow(clippy::module_inception)]
pub mod wither_skeleton {
    #![doc = "Parent class: [`abstract_skeleton`][super::abstract_skeleton]."]
    #[doc = "The bundle of components for spawning `wither_skeleton` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct WitherSkeletonEntityBundle {
        pub wither_skeleton_entity: super::wither_skeleton::WitherSkeletonEntity,
        pub abstract_skeleton_entity: super::abstract_skeleton::AbstractSkeletonEntity,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for WitherSkeletonEntityBundle {
        fn default() -> Self {
            Self {
                wither_skeleton_entity: Default::default(),
                abstract_skeleton_entity: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::WITHER_SKELETON,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `wither_skeleton` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WitherSkeletonEntity;
}
#[allow(clippy::module_inception)]
pub mod wither_skull {
    #![doc = "Parent class: [`explosive_projectile`][super::explosive_projectile]."]
    #[doc = "The bundle of components for spawning `wither_skull` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct WitherSkullEntityBundle {
        pub wither_skull_entity: super::wither_skull::WitherSkullEntity,
        pub wither_skull_charged: super::wither_skull::Charged,
        pub explosive_projectile_entity: super::explosive_projectile::ExplosiveProjectileEntity,
        pub projectile_entity: super::projectile::ProjectileEntity,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for WitherSkullEntityBundle {
        fn default() -> Self {
            Self {
                wither_skull_entity: Default::default(),
                wither_skull_charged: Default::default(),
                explosive_projectile_entity: Default::default(),
                projectile_entity: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::WITHER_SKULL,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Charged(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Charged {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `wither_skull` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WitherSkullEntity;
}
#[allow(clippy::module_inception)]
pub mod wolf {
    #![doc = "Parent class: [`tameable`][super::tameable]."]
    #[doc = "The bundle of components for spawning `wolf` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct WolfEntityBundle {
        pub wolf_entity: super::wolf::WolfEntity,
        pub wolf_begging: super::wolf::Begging,
        pub wolf_collar_color: super::wolf::CollarColor,
        pub wolf_anger_time: super::wolf::AngerTime,
        pub tameable_entity: super::tameable::TameableEntity,
        pub tameable_tameable_flags: super::tameable::TameableFlags,
        pub tameable_owner_uuid: super::tameable::OwnerUuid,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for WolfEntityBundle {
        fn default() -> Self {
            Self {
                wolf_entity: Default::default(),
                wolf_begging: Default::default(),
                wolf_collar_color: Default::default(),
                wolf_anger_time: Default::default(),
                tameable_entity: Default::default(),
                tameable_tameable_flags: Default::default(),
                tameable_owner_uuid: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::WOLF,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Begging(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Begging {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct CollarColor(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for CollarColor {
        fn default() -> Self {
            Self(14i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct AngerTime(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for AngerTime {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[doc = "Marker component for `wolf` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct WolfEntity;
}
#[allow(clippy::module_inception)]
pub mod zoglin {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `zoglin` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ZoglinEntityBundle {
        pub zoglin_entity: super::zoglin::ZoglinEntity,
        pub zoglin_baby: super::zoglin::Baby,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ZoglinEntityBundle {
        fn default() -> Self {
            Self {
                zoglin_entity: Default::default(),
                zoglin_baby: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ZOGLIN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Baby(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Baby {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `zoglin` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ZoglinEntity;
}
#[allow(clippy::module_inception)]
pub mod zombie {
    #![doc = "Parent class: [`hostile`][super::hostile]."]
    #[doc = "The bundle of components for spawning `zombie` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ZombieEntityBundle {
        pub zombie_entity: super::zombie::ZombieEntity,
        pub zombie_baby: super::zombie::Baby,
        pub zombie_zombie_type: super::zombie::ZombieType,
        pub zombie_converting_in_water: super::zombie::ConvertingInWater,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ZombieEntityBundle {
        fn default() -> Self {
            Self {
                zombie_entity: Default::default(),
                zombie_baby: Default::default(),
                zombie_zombie_type: Default::default(),
                zombie_converting_in_water: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ZOMBIE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Baby(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Baby {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ZombieType(pub i32);
    #[allow(clippy::derivable_impls)]
    impl Default for ZombieType {
        fn default() -> Self {
            Self(0i32)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct ConvertingInWater(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for ConvertingInWater {
        fn default() -> Self {
            Self(false)
        }
    }
    #[doc = "Marker component for `zombie` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ZombieEntity;
}
#[allow(clippy::module_inception)]
pub mod zombie_horse {
    #![doc = "Parent class: [`abstract_horse`][super::abstract_horse]."]
    #[doc = "The bundle of components for spawning `zombie_horse` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ZombieHorseEntityBundle {
        pub zombie_horse_entity: super::zombie_horse::ZombieHorseEntity,
        pub abstract_horse_entity: super::abstract_horse::AbstractHorseEntity,
        pub abstract_horse_horse_flags: super::abstract_horse::HorseFlags,
        pub animal_entity: super::animal::AnimalEntity,
        pub passive_entity: super::passive::PassiveEntity,
        pub passive_child: super::passive::Child,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ZombieHorseEntityBundle {
        fn default() -> Self {
            Self {
                zombie_horse_entity: Default::default(),
                abstract_horse_entity: Default::default(),
                abstract_horse_horse_flags: Default::default(),
                animal_entity: Default::default(),
                passive_entity: Default::default(),
                passive_child: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ZOMBIE_HORSE,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `zombie_horse` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ZombieHorseEntity;
}
#[allow(clippy::module_inception)]
pub mod zombie_villager {
    #![doc = "Parent class: [`zombie`][super::zombie]."]
    #[doc = "The bundle of components for spawning `zombie_villager` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ZombieVillagerEntityBundle {
        pub zombie_villager_entity: super::zombie_villager::ZombieVillagerEntity,
        pub zombie_villager_converting: super::zombie_villager::Converting,
        pub zombie_villager_villager_data: super::zombie_villager::VillagerData,
        pub zombie_entity: super::zombie::ZombieEntity,
        pub zombie_baby: super::zombie::Baby,
        pub zombie_zombie_type: super::zombie::ZombieType,
        pub zombie_converting_in_water: super::zombie::ConvertingInWater,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ZombieVillagerEntityBundle {
        fn default() -> Self {
            Self {
                zombie_villager_entity: Default::default(),
                zombie_villager_converting: Default::default(),
                zombie_villager_villager_data: Default::default(),
                zombie_entity: Default::default(),
                zombie_baby: Default::default(),
                zombie_zombie_type: Default::default(),
                zombie_converting_in_water: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ZOMBIE_VILLAGER,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct Converting(pub bool);
    #[allow(clippy::derivable_impls)]
    impl Default for Converting {
        fn default() -> Self {
            Self(false)
        }
    }
    #[derive(
        bevy_ecs :: component :: Component,
        PartialEq,
        Clone,
        Debug,
        :: derive_more :: Deref,
        :: derive_more :: DerefMut,
    )]
    pub struct VillagerData(pub crate::VillagerData);
    #[allow(clippy::derivable_impls)]
    impl Default for VillagerData {
        fn default() -> Self {
            Self(crate::VillagerData {
                kind: crate::VillagerKind::Plains,
                profession: crate::VillagerProfession::Weaponsmith,
                level: 1i32,
            })
        }
    }
    #[doc = "Marker component for `zombie_villager` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ZombieVillagerEntity;
}
#[allow(clippy::module_inception)]
pub mod zombified_piglin {
    #![doc = "Parent class: [`zombie`][super::zombie]."]
    #[doc = "The bundle of components for spawning `zombified_piglin` entities."]
    #[derive(bevy_ecs :: bundle :: Bundle, Debug)]
    pub struct ZombifiedPiglinEntityBundle {
        pub zombified_piglin_entity: super::zombified_piglin::ZombifiedPiglinEntity,
        pub zombie_entity: super::zombie::ZombieEntity,
        pub zombie_baby: super::zombie::Baby,
        pub zombie_zombie_type: super::zombie::ZombieType,
        pub zombie_converting_in_water: super::zombie::ConvertingInWater,
        pub hostile_entity: super::hostile::HostileEntity,
        pub path_aware_entity: super::path_aware::PathAwareEntity,
        pub mob_entity: super::mob::MobEntity,
        pub mob_mob_flags: super::mob::MobFlags,
        pub living_entity: super::living::LivingEntity,
        pub living_living_flags: super::living::LivingFlags,
        pub living_health: super::living::Health,
        pub living_potion_swirls_color: super::living::PotionSwirlsColor,
        pub living_potion_swirls_ambient: super::living::PotionSwirlsAmbient,
        pub living_stuck_arrow_count: super::living::StuckArrowCount,
        pub living_stinger_count: super::living::StingerCount,
        pub living_sleeping_position: super::living::SleepingPosition,
        pub entity: super::entity::Entity,
        pub entity_flags: super::entity::Flags,
        pub entity_air: super::entity::Air,
        pub entity_custom_name: super::entity::CustomName,
        pub entity_name_visible: super::entity::NameVisible,
        pub entity_silent: super::entity::Silent,
        pub entity_no_gravity: super::entity::NoGravity,
        pub entity_pose: super::entity::Pose,
        pub entity_frozen_ticks: super::entity::FrozenTicks,
        pub kind: super::EntityKind,
        pub id: super::EntityId,
        pub uuid: super::UniqueId,
        pub layer: super::EntityLayerId,
        pub old_layer: super::OldEntityLayerId,
        pub position: super::Position,
        pub old_position: super::OldPosition,
        pub look: super::Look,
        pub head_yaw: super::HeadYaw,
        pub on_ground: super::OnGround,
        pub velocity: super::Velocity,
        pub statuses: super::EntityStatuses,
        pub animations: super::EntityAnimations,
        pub object_data: super::ObjectData,
        pub tracked_data: super::tracked_data::TrackedData,
    }
    impl Default for ZombifiedPiglinEntityBundle {
        fn default() -> Self {
            Self {
                zombified_piglin_entity: Default::default(),
                zombie_entity: Default::default(),
                zombie_baby: Default::default(),
                zombie_zombie_type: Default::default(),
                zombie_converting_in_water: Default::default(),
                hostile_entity: Default::default(),
                path_aware_entity: Default::default(),
                mob_entity: Default::default(),
                mob_mob_flags: Default::default(),
                living_entity: Default::default(),
                living_living_flags: Default::default(),
                living_health: Default::default(),
                living_potion_swirls_color: Default::default(),
                living_potion_swirls_ambient: Default::default(),
                living_stuck_arrow_count: Default::default(),
                living_stinger_count: Default::default(),
                living_sleeping_position: Default::default(),
                entity: Default::default(),
                entity_flags: Default::default(),
                entity_air: Default::default(),
                entity_custom_name: Default::default(),
                entity_name_visible: Default::default(),
                entity_silent: Default::default(),
                entity_no_gravity: Default::default(),
                entity_pose: Default::default(),
                entity_frozen_ticks: Default::default(),
                kind: super::EntityKind::ZOMBIFIED_PIGLIN,
                id: Default::default(),
                uuid: Default::default(),
                layer: Default::default(),
                old_layer: Default::default(),
                position: Default::default(),
                old_position: Default::default(),
                look: Default::default(),
                head_yaw: Default::default(),
                on_ground: Default::default(),
                velocity: Default::default(),
                statuses: Default::default(),
                animations: Default::default(),
                object_data: Default::default(),
                tracked_data: Default::default(),
            }
        }
    }
    #[doc = "Marker component for `zombified_piglin` entities."]
    #[derive(bevy_ecs :: component :: Component, Copy, Clone, Default, Debug)]
    pub struct ZombifiedPiglinEntity;
}
#[doc = r" Identifies the type of an entity."]
#[doc = r" As a component, the entity kind should not be modified."]
#[derive(
    Component, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, :: derive_more :: Deref,
)]
pub struct EntityKind(i32);
impl EntityKind {
    pub const ALLAY: EntityKind = EntityKind(0i32);
    pub const AREA_EFFECT_CLOUD: EntityKind = EntityKind(1i32);
    pub const ARMOR_STAND: EntityKind = EntityKind(2i32);
    pub const ARROW: EntityKind = EntityKind(3i32);
    pub const AXOLOTL: EntityKind = EntityKind(4i32);
    pub const BAT: EntityKind = EntityKind(5i32);
    pub const BEE: EntityKind = EntityKind(6i32);
    pub const BLAZE: EntityKind = EntityKind(7i32);
    pub const BLOCK_DISPLAY: EntityKind = EntityKind(8i32);
    pub const BOAT: EntityKind = EntityKind(9i32);
    pub const CAMEL: EntityKind = EntityKind(10i32);
    pub const CAT: EntityKind = EntityKind(11i32);
    pub const CAVE_SPIDER: EntityKind = EntityKind(12i32);
    pub const CHEST_BOAT: EntityKind = EntityKind(13i32);
    pub const CHEST_MINECART: EntityKind = EntityKind(14i32);
    pub const CHICKEN: EntityKind = EntityKind(15i32);
    pub const COD: EntityKind = EntityKind(16i32);
    pub const COMMAND_BLOCK_MINECART: EntityKind = EntityKind(17i32);
    pub const COW: EntityKind = EntityKind(18i32);
    pub const CREEPER: EntityKind = EntityKind(19i32);
    pub const DOLPHIN: EntityKind = EntityKind(20i32);
    pub const DONKEY: EntityKind = EntityKind(21i32);
    pub const DRAGON_FIREBALL: EntityKind = EntityKind(22i32);
    pub const DROWNED: EntityKind = EntityKind(23i32);
    pub const EGG: EntityKind = EntityKind(24i32);
    pub const ELDER_GUARDIAN: EntityKind = EntityKind(25i32);
    pub const END_CRYSTAL: EntityKind = EntityKind(26i32);
    pub const ENDER_DRAGON: EntityKind = EntityKind(27i32);
    pub const ENDER_PEARL: EntityKind = EntityKind(28i32);
    pub const ENDERMAN: EntityKind = EntityKind(29i32);
    pub const ENDERMITE: EntityKind = EntityKind(30i32);
    pub const EVOKER: EntityKind = EntityKind(31i32);
    pub const EVOKER_FANGS: EntityKind = EntityKind(32i32);
    pub const EXPERIENCE_BOTTLE: EntityKind = EntityKind(33i32);
    pub const EXPERIENCE_ORB: EntityKind = EntityKind(34i32);
    pub const EYE_OF_ENDER: EntityKind = EntityKind(35i32);
    pub const FALLING_BLOCK: EntityKind = EntityKind(36i32);
    pub const FIREBALL: EntityKind = EntityKind(57i32);
    pub const FIREWORK_ROCKET: EntityKind = EntityKind(37i32);
    pub const FISHING_BOBBER: EntityKind = EntityKind(123i32);
    pub const FOX: EntityKind = EntityKind(38i32);
    pub const FROG: EntityKind = EntityKind(39i32);
    pub const FURNACE_MINECART: EntityKind = EntityKind(40i32);
    pub const GHAST: EntityKind = EntityKind(41i32);
    pub const GIANT: EntityKind = EntityKind(42i32);
    pub const GLOW_ITEM_FRAME: EntityKind = EntityKind(43i32);
    pub const GLOW_SQUID: EntityKind = EntityKind(44i32);
    pub const GOAT: EntityKind = EntityKind(45i32);
    pub const GUARDIAN: EntityKind = EntityKind(46i32);
    pub const HOGLIN: EntityKind = EntityKind(47i32);
    pub const HOPPER_MINECART: EntityKind = EntityKind(48i32);
    pub const HORSE: EntityKind = EntityKind(49i32);
    pub const HUSK: EntityKind = EntityKind(50i32);
    pub const ILLUSIONER: EntityKind = EntityKind(51i32);
    pub const INTERACTION: EntityKind = EntityKind(52i32);
    pub const IRON_GOLEM: EntityKind = EntityKind(53i32);
    pub const ITEM_DISPLAY: EntityKind = EntityKind(55i32);
    pub const ITEM: EntityKind = EntityKind(54i32);
    pub const ITEM_FRAME: EntityKind = EntityKind(56i32);
    pub const LEASH_KNOT: EntityKind = EntityKind(58i32);
    pub const LIGHTNING: EntityKind = EntityKind(59i32);
    pub const LLAMA: EntityKind = EntityKind(60i32);
    pub const LLAMA_SPIT: EntityKind = EntityKind(61i32);
    pub const MAGMA_CUBE: EntityKind = EntityKind(62i32);
    pub const MARKER: EntityKind = EntityKind(63i32);
    pub const MINECART: EntityKind = EntityKind(64i32);
    pub const MOOSHROOM: EntityKind = EntityKind(65i32);
    pub const MULE: EntityKind = EntityKind(66i32);
    pub const OCELOT: EntityKind = EntityKind(67i32);
    pub const PAINTING: EntityKind = EntityKind(68i32);
    pub const PANDA: EntityKind = EntityKind(69i32);
    pub const PARROT: EntityKind = EntityKind(70i32);
    pub const PHANTOM: EntityKind = EntityKind(71i32);
    pub const PIG: EntityKind = EntityKind(72i32);
    pub const PIGLIN_BRUTE: EntityKind = EntityKind(74i32);
    pub const PIGLIN: EntityKind = EntityKind(73i32);
    pub const PILLAGER: EntityKind = EntityKind(75i32);
    pub const PLAYER: EntityKind = EntityKind(122i32);
    pub const POLAR_BEAR: EntityKind = EntityKind(76i32);
    pub const POTION: EntityKind = EntityKind(77i32);
    pub const PUFFERFISH: EntityKind = EntityKind(78i32);
    pub const RABBIT: EntityKind = EntityKind(79i32);
    pub const RAVAGER: EntityKind = EntityKind(80i32);
    pub const SALMON: EntityKind = EntityKind(81i32);
    pub const SHEEP: EntityKind = EntityKind(82i32);
    pub const SHULKER_BULLET: EntityKind = EntityKind(84i32);
    pub const SHULKER: EntityKind = EntityKind(83i32);
    pub const SILVERFISH: EntityKind = EntityKind(85i32);
    pub const SKELETON: EntityKind = EntityKind(86i32);
    pub const SKELETON_HORSE: EntityKind = EntityKind(87i32);
    pub const SLIME: EntityKind = EntityKind(88i32);
    pub const SMALL_FIREBALL: EntityKind = EntityKind(89i32);
    pub const SNIFFER: EntityKind = EntityKind(90i32);
    pub const SNOW_GOLEM: EntityKind = EntityKind(91i32);
    pub const SNOWBALL: EntityKind = EntityKind(92i32);
    pub const SPAWNER_MINECART: EntityKind = EntityKind(93i32);
    pub const SPECTRAL_ARROW: EntityKind = EntityKind(94i32);
    pub const SPIDER: EntityKind = EntityKind(95i32);
    pub const SQUID: EntityKind = EntityKind(96i32);
    pub const STRAY: EntityKind = EntityKind(97i32);
    pub const STRIDER: EntityKind = EntityKind(98i32);
    pub const TADPOLE: EntityKind = EntityKind(99i32);
    pub const TEXT_DISPLAY: EntityKind = EntityKind(100i32);
    pub const TNT: EntityKind = EntityKind(101i32);
    pub const TNT_MINECART: EntityKind = EntityKind(102i32);
    pub const TRADER_LLAMA: EntityKind = EntityKind(103i32);
    pub const TRIDENT: EntityKind = EntityKind(104i32);
    pub const TROPICAL_FISH: EntityKind = EntityKind(105i32);
    pub const TURTLE: EntityKind = EntityKind(106i32);
    pub const VEX: EntityKind = EntityKind(107i32);
    pub const VILLAGER: EntityKind = EntityKind(108i32);
    pub const VINDICATOR: EntityKind = EntityKind(109i32);
    pub const WANDERING_TRADER: EntityKind = EntityKind(110i32);
    pub const WARDEN: EntityKind = EntityKind(111i32);
    pub const WITCH: EntityKind = EntityKind(112i32);
    pub const WITHER: EntityKind = EntityKind(113i32);
    pub const WITHER_SKELETON: EntityKind = EntityKind(114i32);
    pub const WITHER_SKULL: EntityKind = EntityKind(115i32);
    pub const WOLF: EntityKind = EntityKind(116i32);
    pub const ZOGLIN: EntityKind = EntityKind(117i32);
    pub const ZOMBIE: EntityKind = EntityKind(118i32);
    pub const ZOMBIE_HORSE: EntityKind = EntityKind(119i32);
    pub const ZOMBIE_VILLAGER: EntityKind = EntityKind(120i32);
    pub const ZOMBIFIED_PIGLIN: EntityKind = EntityKind(121i32);
    pub const fn new(inner: i32) -> Self {
        Self(inner)
    }
    pub const fn get(self) -> i32 {
        self.0
    }
    pub const fn translation_key(self) -> Option<&'static str> {
        match self {
            EntityKind::ALLAY => Some("entity.minecraft.allay"),
            EntityKind::AREA_EFFECT_CLOUD => Some("entity.minecraft.area_effect_cloud"),
            EntityKind::ARMOR_STAND => Some("entity.minecraft.armor_stand"),
            EntityKind::ARROW => Some("entity.minecraft.arrow"),
            EntityKind::AXOLOTL => Some("entity.minecraft.axolotl"),
            EntityKind::BAT => Some("entity.minecraft.bat"),
            EntityKind::BEE => Some("entity.minecraft.bee"),
            EntityKind::BLAZE => Some("entity.minecraft.blaze"),
            EntityKind::BLOCK_DISPLAY => Some("entity.minecraft.block_display"),
            EntityKind::BOAT => Some("entity.minecraft.boat"),
            EntityKind::CAMEL => Some("entity.minecraft.camel"),
            EntityKind::CAT => Some("entity.minecraft.cat"),
            EntityKind::CAVE_SPIDER => Some("entity.minecraft.cave_spider"),
            EntityKind::CHEST_BOAT => Some("entity.minecraft.chest_boat"),
            EntityKind::CHEST_MINECART => Some("entity.minecraft.chest_minecart"),
            EntityKind::CHICKEN => Some("entity.minecraft.chicken"),
            EntityKind::COD => Some("entity.minecraft.cod"),
            EntityKind::COMMAND_BLOCK_MINECART => Some("entity.minecraft.command_block_minecart"),
            EntityKind::COW => Some("entity.minecraft.cow"),
            EntityKind::CREEPER => Some("entity.minecraft.creeper"),
            EntityKind::DOLPHIN => Some("entity.minecraft.dolphin"),
            EntityKind::DONKEY => Some("entity.minecraft.donkey"),
            EntityKind::DRAGON_FIREBALL => Some("entity.minecraft.dragon_fireball"),
            EntityKind::DROWNED => Some("entity.minecraft.drowned"),
            EntityKind::EGG => Some("entity.minecraft.egg"),
            EntityKind::ELDER_GUARDIAN => Some("entity.minecraft.elder_guardian"),
            EntityKind::END_CRYSTAL => Some("entity.minecraft.end_crystal"),
            EntityKind::ENDER_DRAGON => Some("entity.minecraft.ender_dragon"),
            EntityKind::ENDER_PEARL => Some("entity.minecraft.ender_pearl"),
            EntityKind::ENDERMAN => Some("entity.minecraft.enderman"),
            EntityKind::ENDERMITE => Some("entity.minecraft.endermite"),
            EntityKind::EVOKER => Some("entity.minecraft.evoker"),
            EntityKind::EVOKER_FANGS => Some("entity.minecraft.evoker_fangs"),
            EntityKind::EXPERIENCE_BOTTLE => Some("entity.minecraft.experience_bottle"),
            EntityKind::EXPERIENCE_ORB => Some("entity.minecraft.experience_orb"),
            EntityKind::EYE_OF_ENDER => Some("entity.minecraft.eye_of_ender"),
            EntityKind::FALLING_BLOCK => Some("entity.minecraft.falling_block"),
            EntityKind::FIREBALL => Some("entity.minecraft.fireball"),
            EntityKind::FIREWORK_ROCKET => Some("entity.minecraft.firework_rocket"),
            EntityKind::FISHING_BOBBER => Some("entity.minecraft.fishing_bobber"),
            EntityKind::FOX => Some("entity.minecraft.fox"),
            EntityKind::FROG => Some("entity.minecraft.frog"),
            EntityKind::FURNACE_MINECART => Some("entity.minecraft.furnace_minecart"),
            EntityKind::GHAST => Some("entity.minecraft.ghast"),
            EntityKind::GIANT => Some("entity.minecraft.giant"),
            EntityKind::GLOW_ITEM_FRAME => Some("entity.minecraft.glow_item_frame"),
            EntityKind::GLOW_SQUID => Some("entity.minecraft.glow_squid"),
            EntityKind::GOAT => Some("entity.minecraft.goat"),
            EntityKind::GUARDIAN => Some("entity.minecraft.guardian"),
            EntityKind::HOGLIN => Some("entity.minecraft.hoglin"),
            EntityKind::HOPPER_MINECART => Some("entity.minecraft.hopper_minecart"),
            EntityKind::HORSE => Some("entity.minecraft.horse"),
            EntityKind::HUSK => Some("entity.minecraft.husk"),
            EntityKind::ILLUSIONER => Some("entity.minecraft.illusioner"),
            EntityKind::INTERACTION => Some("entity.minecraft.interaction"),
            EntityKind::IRON_GOLEM => Some("entity.minecraft.iron_golem"),
            EntityKind::ITEM_DISPLAY => Some("entity.minecraft.item_display"),
            EntityKind::ITEM => Some("entity.minecraft.item"),
            EntityKind::ITEM_FRAME => Some("entity.minecraft.item_frame"),
            EntityKind::LEASH_KNOT => Some("entity.minecraft.leash_knot"),
            EntityKind::LIGHTNING => Some("entity.minecraft.lightning_bolt"),
            EntityKind::LLAMA => Some("entity.minecraft.llama"),
            EntityKind::LLAMA_SPIT => Some("entity.minecraft.llama_spit"),
            EntityKind::MAGMA_CUBE => Some("entity.minecraft.magma_cube"),
            EntityKind::MARKER => Some("entity.minecraft.marker"),
            EntityKind::MINECART => Some("entity.minecraft.minecart"),
            EntityKind::MOOSHROOM => Some("entity.minecraft.mooshroom"),
            EntityKind::MULE => Some("entity.minecraft.mule"),
            EntityKind::OCELOT => Some("entity.minecraft.ocelot"),
            EntityKind::PAINTING => Some("entity.minecraft.painting"),
            EntityKind::PANDA => Some("entity.minecraft.panda"),
            EntityKind::PARROT => Some("entity.minecraft.parrot"),
            EntityKind::PHANTOM => Some("entity.minecraft.phantom"),
            EntityKind::PIG => Some("entity.minecraft.pig"),
            EntityKind::PIGLIN_BRUTE => Some("entity.minecraft.piglin_brute"),
            EntityKind::PIGLIN => Some("entity.minecraft.piglin"),
            EntityKind::PILLAGER => Some("entity.minecraft.pillager"),
            EntityKind::PLAYER => Some("entity.minecraft.player"),
            EntityKind::POLAR_BEAR => Some("entity.minecraft.polar_bear"),
            EntityKind::POTION => Some("entity.minecraft.potion"),
            EntityKind::PUFFERFISH => Some("entity.minecraft.pufferfish"),
            EntityKind::RABBIT => Some("entity.minecraft.rabbit"),
            EntityKind::RAVAGER => Some("entity.minecraft.ravager"),
            EntityKind::SALMON => Some("entity.minecraft.salmon"),
            EntityKind::SHEEP => Some("entity.minecraft.sheep"),
            EntityKind::SHULKER_BULLET => Some("entity.minecraft.shulker_bullet"),
            EntityKind::SHULKER => Some("entity.minecraft.shulker"),
            EntityKind::SILVERFISH => Some("entity.minecraft.silverfish"),
            EntityKind::SKELETON => Some("entity.minecraft.skeleton"),
            EntityKind::SKELETON_HORSE => Some("entity.minecraft.skeleton_horse"),
            EntityKind::SLIME => Some("entity.minecraft.slime"),
            EntityKind::SMALL_FIREBALL => Some("entity.minecraft.small_fireball"),
            EntityKind::SNIFFER => Some("entity.minecraft.sniffer"),
            EntityKind::SNOW_GOLEM => Some("entity.minecraft.snow_golem"),
            EntityKind::SNOWBALL => Some("entity.minecraft.snowball"),
            EntityKind::SPAWNER_MINECART => Some("entity.minecraft.spawner_minecart"),
            EntityKind::SPECTRAL_ARROW => Some("entity.minecraft.spectral_arrow"),
            EntityKind::SPIDER => Some("entity.minecraft.spider"),
            EntityKind::SQUID => Some("entity.minecraft.squid"),
            EntityKind::STRAY => Some("entity.minecraft.stray"),
            EntityKind::STRIDER => Some("entity.minecraft.strider"),
            EntityKind::TADPOLE => Some("entity.minecraft.tadpole"),
            EntityKind::TEXT_DISPLAY => Some("entity.minecraft.text_display"),
            EntityKind::TNT => Some("entity.minecraft.tnt"),
            EntityKind::TNT_MINECART => Some("entity.minecraft.tnt_minecart"),
            EntityKind::TRADER_LLAMA => Some("entity.minecraft.trader_llama"),
            EntityKind::TRIDENT => Some("entity.minecraft.trident"),
            EntityKind::TROPICAL_FISH => Some("entity.minecraft.tropical_fish"),
            EntityKind::TURTLE => Some("entity.minecraft.turtle"),
            EntityKind::VEX => Some("entity.minecraft.vex"),
            EntityKind::VILLAGER => Some("entity.minecraft.villager"),
            EntityKind::VINDICATOR => Some("entity.minecraft.vindicator"),
            EntityKind::WANDERING_TRADER => Some("entity.minecraft.wandering_trader"),
            EntityKind::WARDEN => Some("entity.minecraft.warden"),
            EntityKind::WITCH => Some("entity.minecraft.witch"),
            EntityKind::WITHER => Some("entity.minecraft.wither"),
            EntityKind::WITHER_SKELETON => Some("entity.minecraft.wither_skeleton"),
            EntityKind::WITHER_SKULL => Some("entity.minecraft.wither_skull"),
            EntityKind::WOLF => Some("entity.minecraft.wolf"),
            EntityKind::ZOGLIN => Some("entity.minecraft.zoglin"),
            EntityKind::ZOMBIE => Some("entity.minecraft.zombie"),
            EntityKind::ZOMBIE_HORSE => Some("entity.minecraft.zombie_horse"),
            EntityKind::ZOMBIE_VILLAGER => Some("entity.minecraft.zombie_villager"),
            EntityKind::ZOMBIFIED_PIGLIN => Some("entity.minecraft.zombified_piglin"),
            _ => None,
        }
    }
}
impl std::fmt::Debug for EntityKind {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EntityKind::ALLAY => write!(f, "{} ({})", 0i32, "ALLAY"),
            EntityKind::AREA_EFFECT_CLOUD => write!(f, "{} ({})", 1i32, "AREA_EFFECT_CLOUD"),
            EntityKind::ARMOR_STAND => write!(f, "{} ({})", 2i32, "ARMOR_STAND"),
            EntityKind::ARROW => write!(f, "{} ({})", 3i32, "ARROW"),
            EntityKind::AXOLOTL => write!(f, "{} ({})", 4i32, "AXOLOTL"),
            EntityKind::BAT => write!(f, "{} ({})", 5i32, "BAT"),
            EntityKind::BEE => write!(f, "{} ({})", 6i32, "BEE"),
            EntityKind::BLAZE => write!(f, "{} ({})", 7i32, "BLAZE"),
            EntityKind::BLOCK_DISPLAY => write!(f, "{} ({})", 8i32, "BLOCK_DISPLAY"),
            EntityKind::BOAT => write!(f, "{} ({})", 9i32, "BOAT"),
            EntityKind::CAMEL => write!(f, "{} ({})", 10i32, "CAMEL"),
            EntityKind::CAT => write!(f, "{} ({})", 11i32, "CAT"),
            EntityKind::CAVE_SPIDER => write!(f, "{} ({})", 12i32, "CAVE_SPIDER"),
            EntityKind::CHEST_BOAT => write!(f, "{} ({})", 13i32, "CHEST_BOAT"),
            EntityKind::CHEST_MINECART => write!(f, "{} ({})", 14i32, "CHEST_MINECART"),
            EntityKind::CHICKEN => write!(f, "{} ({})", 15i32, "CHICKEN"),
            EntityKind::COD => write!(f, "{} ({})", 16i32, "COD"),
            EntityKind::COMMAND_BLOCK_MINECART => {
                write!(f, "{} ({})", 17i32, "COMMAND_BLOCK_MINECART")
            }
            EntityKind::COW => write!(f, "{} ({})", 18i32, "COW"),
            EntityKind::CREEPER => write!(f, "{} ({})", 19i32, "CREEPER"),
            EntityKind::DOLPHIN => write!(f, "{} ({})", 20i32, "DOLPHIN"),
            EntityKind::DONKEY => write!(f, "{} ({})", 21i32, "DONKEY"),
            EntityKind::DRAGON_FIREBALL => write!(f, "{} ({})", 22i32, "DRAGON_FIREBALL"),
            EntityKind::DROWNED => write!(f, "{} ({})", 23i32, "DROWNED"),
            EntityKind::EGG => write!(f, "{} ({})", 24i32, "EGG"),
            EntityKind::ELDER_GUARDIAN => write!(f, "{} ({})", 25i32, "ELDER_GUARDIAN"),
            EntityKind::END_CRYSTAL => write!(f, "{} ({})", 26i32, "END_CRYSTAL"),
            EntityKind::ENDER_DRAGON => write!(f, "{} ({})", 27i32, "ENDER_DRAGON"),
            EntityKind::ENDER_PEARL => write!(f, "{} ({})", 28i32, "ENDER_PEARL"),
            EntityKind::ENDERMAN => write!(f, "{} ({})", 29i32, "ENDERMAN"),
            EntityKind::ENDERMITE => write!(f, "{} ({})", 30i32, "ENDERMITE"),
            EntityKind::EVOKER => write!(f, "{} ({})", 31i32, "EVOKER"),
            EntityKind::EVOKER_FANGS => write!(f, "{} ({})", 32i32, "EVOKER_FANGS"),
            EntityKind::EXPERIENCE_BOTTLE => write!(f, "{} ({})", 33i32, "EXPERIENCE_BOTTLE"),
            EntityKind::EXPERIENCE_ORB => write!(f, "{} ({})", 34i32, "EXPERIENCE_ORB"),
            EntityKind::EYE_OF_ENDER => write!(f, "{} ({})", 35i32, "EYE_OF_ENDER"),
            EntityKind::FALLING_BLOCK => write!(f, "{} ({})", 36i32, "FALLING_BLOCK"),
            EntityKind::FIREBALL => write!(f, "{} ({})", 57i32, "FIREBALL"),
            EntityKind::FIREWORK_ROCKET => write!(f, "{} ({})", 37i32, "FIREWORK_ROCKET"),
            EntityKind::FISHING_BOBBER => write!(f, "{} ({})", 123i32, "FISHING_BOBBER"),
            EntityKind::FOX => write!(f, "{} ({})", 38i32, "FOX"),
            EntityKind::FROG => write!(f, "{} ({})", 39i32, "FROG"),
            EntityKind::FURNACE_MINECART => write!(f, "{} ({})", 40i32, "FURNACE_MINECART"),
            EntityKind::GHAST => write!(f, "{} ({})", 41i32, "GHAST"),
            EntityKind::GIANT => write!(f, "{} ({})", 42i32, "GIANT"),
            EntityKind::GLOW_ITEM_FRAME => write!(f, "{} ({})", 43i32, "GLOW_ITEM_FRAME"),
            EntityKind::GLOW_SQUID => write!(f, "{} ({})", 44i32, "GLOW_SQUID"),
            EntityKind::GOAT => write!(f, "{} ({})", 45i32, "GOAT"),
            EntityKind::GUARDIAN => write!(f, "{} ({})", 46i32, "GUARDIAN"),
            EntityKind::HOGLIN => write!(f, "{} ({})", 47i32, "HOGLIN"),
            EntityKind::HOPPER_MINECART => write!(f, "{} ({})", 48i32, "HOPPER_MINECART"),
            EntityKind::HORSE => write!(f, "{} ({})", 49i32, "HORSE"),
            EntityKind::HUSK => write!(f, "{} ({})", 50i32, "HUSK"),
            EntityKind::ILLUSIONER => write!(f, "{} ({})", 51i32, "ILLUSIONER"),
            EntityKind::INTERACTION => write!(f, "{} ({})", 52i32, "INTERACTION"),
            EntityKind::IRON_GOLEM => write!(f, "{} ({})", 53i32, "IRON_GOLEM"),
            EntityKind::ITEM_DISPLAY => write!(f, "{} ({})", 55i32, "ITEM_DISPLAY"),
            EntityKind::ITEM => write!(f, "{} ({})", 54i32, "ITEM"),
            EntityKind::ITEM_FRAME => write!(f, "{} ({})", 56i32, "ITEM_FRAME"),
            EntityKind::LEASH_KNOT => write!(f, "{} ({})", 58i32, "LEASH_KNOT"),
            EntityKind::LIGHTNING => write!(f, "{} ({})", 59i32, "LIGHTNING"),
            EntityKind::LLAMA => write!(f, "{} ({})", 60i32, "LLAMA"),
            EntityKind::LLAMA_SPIT => write!(f, "{} ({})", 61i32, "LLAMA_SPIT"),
            EntityKind::MAGMA_CUBE => write!(f, "{} ({})", 62i32, "MAGMA_CUBE"),
            EntityKind::MARKER => write!(f, "{} ({})", 63i32, "MARKER"),
            EntityKind::MINECART => write!(f, "{} ({})", 64i32, "MINECART"),
            EntityKind::MOOSHROOM => write!(f, "{} ({})", 65i32, "MOOSHROOM"),
            EntityKind::MULE => write!(f, "{} ({})", 66i32, "MULE"),
            EntityKind::OCELOT => write!(f, "{} ({})", 67i32, "OCELOT"),
            EntityKind::PAINTING => write!(f, "{} ({})", 68i32, "PAINTING"),
            EntityKind::PANDA => write!(f, "{} ({})", 69i32, "PANDA"),
            EntityKind::PARROT => write!(f, "{} ({})", 70i32, "PARROT"),
            EntityKind::PHANTOM => write!(f, "{} ({})", 71i32, "PHANTOM"),
            EntityKind::PIG => write!(f, "{} ({})", 72i32, "PIG"),
            EntityKind::PIGLIN_BRUTE => write!(f, "{} ({})", 74i32, "PIGLIN_BRUTE"),
            EntityKind::PIGLIN => write!(f, "{} ({})", 73i32, "PIGLIN"),
            EntityKind::PILLAGER => write!(f, "{} ({})", 75i32, "PILLAGER"),
            EntityKind::PLAYER => write!(f, "{} ({})", 122i32, "PLAYER"),
            EntityKind::POLAR_BEAR => write!(f, "{} ({})", 76i32, "POLAR_BEAR"),
            EntityKind::POTION => write!(f, "{} ({})", 77i32, "POTION"),
            EntityKind::PUFFERFISH => write!(f, "{} ({})", 78i32, "PUFFERFISH"),
            EntityKind::RABBIT => write!(f, "{} ({})", 79i32, "RABBIT"),
            EntityKind::RAVAGER => write!(f, "{} ({})", 80i32, "RAVAGER"),
            EntityKind::SALMON => write!(f, "{} ({})", 81i32, "SALMON"),
            EntityKind::SHEEP => write!(f, "{} ({})", 82i32, "SHEEP"),
            EntityKind::SHULKER_BULLET => write!(f, "{} ({})", 84i32, "SHULKER_BULLET"),
            EntityKind::SHULKER => write!(f, "{} ({})", 83i32, "SHULKER"),
            EntityKind::SILVERFISH => write!(f, "{} ({})", 85i32, "SILVERFISH"),
            EntityKind::SKELETON => write!(f, "{} ({})", 86i32, "SKELETON"),
            EntityKind::SKELETON_HORSE => write!(f, "{} ({})", 87i32, "SKELETON_HORSE"),
            EntityKind::SLIME => write!(f, "{} ({})", 88i32, "SLIME"),
            EntityKind::SMALL_FIREBALL => write!(f, "{} ({})", 89i32, "SMALL_FIREBALL"),
            EntityKind::SNIFFER => write!(f, "{} ({})", 90i32, "SNIFFER"),
            EntityKind::SNOW_GOLEM => write!(f, "{} ({})", 91i32, "SNOW_GOLEM"),
            EntityKind::SNOWBALL => write!(f, "{} ({})", 92i32, "SNOWBALL"),
            EntityKind::SPAWNER_MINECART => write!(f, "{} ({})", 93i32, "SPAWNER_MINECART"),
            EntityKind::SPECTRAL_ARROW => write!(f, "{} ({})", 94i32, "SPECTRAL_ARROW"),
            EntityKind::SPIDER => write!(f, "{} ({})", 95i32, "SPIDER"),
            EntityKind::SQUID => write!(f, "{} ({})", 96i32, "SQUID"),
            EntityKind::STRAY => write!(f, "{} ({})", 97i32, "STRAY"),
            EntityKind::STRIDER => write!(f, "{} ({})", 98i32, "STRIDER"),
            EntityKind::TADPOLE => write!(f, "{} ({})", 99i32, "TADPOLE"),
            EntityKind::TEXT_DISPLAY => write!(f, "{} ({})", 100i32, "TEXT_DISPLAY"),
            EntityKind::TNT => write!(f, "{} ({})", 101i32, "TNT"),
            EntityKind::TNT_MINECART => write!(f, "{} ({})", 102i32, "TNT_MINECART"),
            EntityKind::TRADER_LLAMA => write!(f, "{} ({})", 103i32, "TRADER_LLAMA"),
            EntityKind::TRIDENT => write!(f, "{} ({})", 104i32, "TRIDENT"),
            EntityKind::TROPICAL_FISH => write!(f, "{} ({})", 105i32, "TROPICAL_FISH"),
            EntityKind::TURTLE => write!(f, "{} ({})", 106i32, "TURTLE"),
            EntityKind::VEX => write!(f, "{} ({})", 107i32, "VEX"),
            EntityKind::VILLAGER => write!(f, "{} ({})", 108i32, "VILLAGER"),
            EntityKind::VINDICATOR => write!(f, "{} ({})", 109i32, "VINDICATOR"),
            EntityKind::WANDERING_TRADER => write!(f, "{} ({})", 110i32, "WANDERING_TRADER"),
            EntityKind::WARDEN => write!(f, "{} ({})", 111i32, "WARDEN"),
            EntityKind::WITCH => write!(f, "{} ({})", 112i32, "WITCH"),
            EntityKind::WITHER => write!(f, "{} ({})", 113i32, "WITHER"),
            EntityKind::WITHER_SKELETON => write!(f, "{} ({})", 114i32, "WITHER_SKELETON"),
            EntityKind::WITHER_SKULL => write!(f, "{} ({})", 115i32, "WITHER_SKULL"),
            EntityKind::WOLF => write!(f, "{} ({})", 116i32, "WOLF"),
            EntityKind::ZOGLIN => write!(f, "{} ({})", 117i32, "ZOGLIN"),
            EntityKind::ZOMBIE => write!(f, "{} ({})", 118i32, "ZOMBIE"),
            EntityKind::ZOMBIE_HORSE => write!(f, "{} ({})", 119i32, "ZOMBIE_HORSE"),
            EntityKind::ZOMBIE_VILLAGER => write!(f, "{} ({})", 120i32, "ZOMBIE_VILLAGER"),
            EntityKind::ZOMBIFIED_PIGLIN => write!(f, "{} ({})", 121i32, "ZOMBIFIED_PIGLIN"),
            EntityKind(other) => write!(f, "{other}"),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EntityStatus {
    AddBreedingParticles = 18isize,
    AddCloudParticles = 43isize,
    AddDeathParticles = 60isize,
    AddDolphinHappyVillagerParticles = 38isize,
    AddNegativePlayerReactionParticles = 6isize,
    AddPortalParticles = 46isize,
    AddPositivePlayerReactionParticles = 7isize,
    AddSplashParticles = 42isize,
    AddSprintingParticlesOrResetSpawnerMinecartSpawnDelay = 1isize,
    AddVillagerAngryParticles = 13isize,
    AddVillagerHappyParticles = 14isize,
    AddVillagerHeartParticles = 12isize,
    AddWitchParticles = 15isize,
    BlockWithShield = 29isize,
    BreakChest = 50isize,
    BreakFeet = 52isize,
    BreakHead = 49isize,
    BreakLegs = 51isize,
    BreakMainhand = 47isize,
    BreakOffhand = 48isize,
    BreakShield = 30isize,
    ConsumeItem = 9isize,
    CreateEatingParticles = 45isize,
    DripHoney = 53isize,
    DripRichHoney = 54isize,
    EarsTwitch = 61isize,
    ExplodeFireworkClient = 17isize,
    FinishRam = 59isize,
    HitArmorStand = 32isize,
    LookAtVillager = 11isize,
    PlayAttackSound = 4isize,
    PlayCureZombieVillagerSound = 16isize,
    PlayDeathSoundOrAddProjectileHitParticles = 3isize,
    PlayGuardianAttackSound = 21isize,
    PlaySpawnEffects = 20isize,
    PrepareRam = 58isize,
    PullHookedEntity = 31isize,
    ResetSquidThrustTimer = 19isize,
    ResetWolfShake = 56isize,
    SetOpLevel0 = 24isize,
    SetOpLevel1 = 25isize,
    SetOpLevel2 = 26isize,
    SetOpLevel3 = 27isize,
    SetOpLevel4 = 28isize,
    SetSheepEatGrassTimerOrPrimeTntMinecart = 10isize,
    ShakeOffWater = 8isize,
    SonicBoom = 62isize,
    StartDigging = 63isize,
    StopAttack = 5isize,
    StopLookingAtVillager = 34isize,
    StunRavager = 39isize,
    SwapHands = 55isize,
    TameOcelotFailed = 40isize,
    TameOcelotSuccess = 41isize,
    UseFullDebugInfo = 23isize,
    UseReducedDebugInfo = 22isize,
    UseTotemOfUndying = 35isize,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EntityAnimation {
    Crit = 4isize,
    EnchantedHit = 5isize,
    SwingMainHand = 0isize,
    SwingOffHand = 3isize,
    WakeUp = 2isize,
}
fn add_tracked_data_systems(app: &mut App) {
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_donkey_chest(
        mut query: Query<
            (&abstract_donkey::Chest, &mut tracked_data::TrackedData),
            Changed<abstract_donkey::Chest>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_fireball_item(
        mut query: Query<
            (&abstract_fireball::Item, &mut tracked_data::TrackedData),
            Changed<abstract_fireball::Item>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 7u8, Some(&value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 7u8, Some(&value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_horse_horse_flags(
        mut query: Query<
            (&abstract_horse::HorseFlags, &mut tracked_data::TrackedData),
            Changed<abstract_horse::HorseFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_minecart_damage_wobble_ticks(
        mut query: Query<
            (
                &abstract_minecart::DamageWobbleTicks,
                &mut tracked_data::TrackedData,
            ),
            Changed<abstract_minecart::DamageWobbleTicks>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_minecart_damage_wobble_side(
        mut query: Query<
            (
                &abstract_minecart::DamageWobbleSide,
                &mut tracked_data::TrackedData,
            ),
            Changed<abstract_minecart::DamageWobbleSide>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_minecart_damage_wobble_strength(
        mut query: Query<
            (
                &abstract_minecart::DamageWobbleStrength,
                &mut tracked_data::TrackedData,
            ),
            Changed<abstract_minecart::DamageWobbleStrength>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_minecart_custom_block_id(
        mut query: Query<
            (
                &abstract_minecart::CustomBlockId,
                &mut tracked_data::TrackedData,
            ),
            Changed<abstract_minecart::CustomBlockId>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(11u8);
            } else {
                tracked_data.insert_init_value(11u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(11u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_minecart_custom_block_offset(
        mut query: Query<
            (
                &abstract_minecart::CustomBlockOffset,
                &mut tracked_data::TrackedData,
            ),
            Changed<abstract_minecart::CustomBlockOffset>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(12u8);
            } else {
                tracked_data.insert_init_value(12u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(12u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_minecart_custom_block_present(
        mut query: Query<
            (
                &abstract_minecart::CustomBlockPresent,
                &mut tracked_data::TrackedData,
            ),
            Changed<abstract_minecart::CustomBlockPresent>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(13u8);
            } else {
                tracked_data.insert_init_value(13u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(13u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_abstract_piglin_immune_to_zombification(
        mut query: Query<
            (
                &abstract_piglin::ImmuneToZombification,
                &mut tracked_data::TrackedData,
            ),
            Changed<abstract_piglin::ImmuneToZombification>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_allay_dancing(
        mut query: Query<
            (&allay::Dancing, &mut tracked_data::TrackedData),
            Changed<allay::Dancing>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_allay_can_duplicate(
        mut query: Query<
            (&allay::CanDuplicate, &mut tracked_data::TrackedData),
            Changed<allay::CanDuplicate>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_area_effect_cloud_radius(
        mut query: Query<
            (&area_effect_cloud::Radius, &mut tracked_data::TrackedData),
            Changed<area_effect_cloud::Radius>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_area_effect_cloud_color(
        mut query: Query<
            (&area_effect_cloud::Color, &mut tracked_data::TrackedData),
            Changed<area_effect_cloud::Color>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_area_effect_cloud_waiting(
        mut query: Query<
            (&area_effect_cloud::Waiting, &mut tracked_data::TrackedData),
            Changed<area_effect_cloud::Waiting>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_area_effect_cloud_particle_id(
        mut query: Query<
            (
                &area_effect_cloud::ParticleId,
                &mut tracked_data::TrackedData,
            ),
            Changed<area_effect_cloud::ParticleId>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(11u8);
            } else {
                tracked_data.insert_init_value(11u8, 17u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(11u8, 17u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_armor_stand_armor_stand_flags(
        mut query: Query<
            (
                &armor_stand::ArmorStandFlags,
                &mut tracked_data::TrackedData,
            ),
            Changed<armor_stand::ArmorStandFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(15u8);
            } else {
                tracked_data.insert_init_value(15u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(15u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_armor_stand_tracker_head_rotation(
        mut query: Query<
            (
                &armor_stand::TrackerHeadRotation,
                &mut tracked_data::TrackedData,
            ),
            Changed<armor_stand::TrackerHeadRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 9u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 9u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_armor_stand_tracker_body_rotation(
        mut query: Query<
            (
                &armor_stand::TrackerBodyRotation,
                &mut tracked_data::TrackedData,
            ),
            Changed<armor_stand::TrackerBodyRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 9u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 9u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_armor_stand_tracker_left_arm_rotation(
        mut query: Query<
            (
                &armor_stand::TrackerLeftArmRotation,
                &mut tracked_data::TrackedData,
            ),
            Changed<armor_stand::TrackerLeftArmRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 9u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 9u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_armor_stand_tracker_right_arm_rotation(
        mut query: Query<
            (
                &armor_stand::TrackerRightArmRotation,
                &mut tracked_data::TrackedData,
            ),
            Changed<armor_stand::TrackerRightArmRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 9u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 9u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_armor_stand_tracker_left_leg_rotation(
        mut query: Query<
            (
                &armor_stand::TrackerLeftLegRotation,
                &mut tracked_data::TrackedData,
            ),
            Changed<armor_stand::TrackerLeftLegRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 9u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 9u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_armor_stand_tracker_right_leg_rotation(
        mut query: Query<
            (
                &armor_stand::TrackerRightLegRotation,
                &mut tracked_data::TrackedData,
            ),
            Changed<armor_stand::TrackerRightLegRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(21u8);
            } else {
                tracked_data.insert_init_value(21u8, 9u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(21u8, 9u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_arrow_color(
        mut query: Query<(&arrow::Color, &mut tracked_data::TrackedData), Changed<arrow::Color>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_axolotl_variant(
        mut query: Query<
            (&axolotl::Variant, &mut tracked_data::TrackedData),
            Changed<axolotl::Variant>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_axolotl_playing_dead(
        mut query: Query<
            (&axolotl::PlayingDead, &mut tracked_data::TrackedData),
            Changed<axolotl::PlayingDead>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_axolotl_from_bucket(
        mut query: Query<
            (&axolotl::FromBucket, &mut tracked_data::TrackedData),
            Changed<axolotl::FromBucket>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_bat_bat_flags(
        mut query: Query<(&bat::BatFlags, &mut tracked_data::TrackedData), Changed<bat::BatFlags>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_bee_bee_flags(
        mut query: Query<(&bee::BeeFlags, &mut tracked_data::TrackedData), Changed<bee::BeeFlags>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_bee_anger(
        mut query: Query<(&bee::Anger, &mut tracked_data::TrackedData), Changed<bee::Anger>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_blaze_blaze_flags(
        mut query: Query<
            (&blaze::BlazeFlags, &mut tracked_data::TrackedData),
            Changed<blaze::BlazeFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_block_display_block_state(
        mut query: Query<
            (&block_display::BlockState, &mut tracked_data::TrackedData),
            Changed<block_display::BlockState>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(22u8);
            } else {
                tracked_data.insert_init_value(22u8, 14u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(22u8, 14u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_boat_damage_wobble_ticks(
        mut query: Query<
            (&boat::DamageWobbleTicks, &mut tracked_data::TrackedData),
            Changed<boat::DamageWobbleTicks>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_boat_damage_wobble_side(
        mut query: Query<
            (&boat::DamageWobbleSide, &mut tracked_data::TrackedData),
            Changed<boat::DamageWobbleSide>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_boat_damage_wobble_strength(
        mut query: Query<
            (&boat::DamageWobbleStrength, &mut tracked_data::TrackedData),
            Changed<boat::DamageWobbleStrength>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_boat_boat_type(
        mut query: Query<
            (&boat::BoatType, &mut tracked_data::TrackedData),
            Changed<boat::BoatType>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(11u8);
            } else {
                tracked_data.insert_init_value(11u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(11u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_boat_left_paddle_moving(
        mut query: Query<
            (&boat::LeftPaddleMoving, &mut tracked_data::TrackedData),
            Changed<boat::LeftPaddleMoving>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(12u8);
            } else {
                tracked_data.insert_init_value(12u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(12u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_boat_right_paddle_moving(
        mut query: Query<
            (&boat::RightPaddleMoving, &mut tracked_data::TrackedData),
            Changed<boat::RightPaddleMoving>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(13u8);
            } else {
                tracked_data.insert_init_value(13u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(13u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_boat_bubble_wobble_ticks(
        mut query: Query<
            (&boat::BubbleWobbleTicks, &mut tracked_data::TrackedData),
            Changed<boat::BubbleWobbleTicks>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(14u8);
            } else {
                tracked_data.insert_init_value(14u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(14u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_camel_dashing(
        mut query: Query<
            (&camel::Dashing, &mut tracked_data::TrackedData),
            Changed<camel::Dashing>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_camel_last_pose_tick(
        mut query: Query<
            (&camel::LastPoseTick, &mut tracked_data::TrackedData),
            Changed<camel::LastPoseTick>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 2u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 2u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_cat_cat_variant(
        mut query: Query<
            (&cat::CatVariant, &mut tracked_data::TrackedData),
            Changed<cat::CatVariant>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 21u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 21u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_cat_in_sleeping_pose(
        mut query: Query<
            (&cat::InSleepingPose, &mut tracked_data::TrackedData),
            Changed<cat::InSleepingPose>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_cat_head_down(
        mut query: Query<(&cat::HeadDown, &mut tracked_data::TrackedData), Changed<cat::HeadDown>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(21u8);
            } else {
                tracked_data.insert_init_value(21u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(21u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_cat_collar_color(
        mut query: Query<
            (&cat::CollarColor, &mut tracked_data::TrackedData),
            Changed<cat::CollarColor>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(22u8);
            } else {
                tracked_data.insert_init_value(22u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(22u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_command_block_minecart_command(
        mut query: Query<
            (
                &command_block_minecart::Command,
                &mut tracked_data::TrackedData,
            ),
            Changed<command_block_minecart::Command>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(14u8);
            } else {
                tracked_data.insert_init_value(14u8, 4u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(14u8, 4u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_command_block_minecart_last_output(
        mut query: Query<
            (
                &command_block_minecart::LastOutput,
                &mut tracked_data::TrackedData,
            ),
            Changed<command_block_minecart::LastOutput>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(15u8);
            } else {
                tracked_data.insert_init_value(15u8, 5u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(15u8, 5u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_creeper_fuse_speed(
        mut query: Query<
            (&creeper::FuseSpeed, &mut tracked_data::TrackedData),
            Changed<creeper::FuseSpeed>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_creeper_charged(
        mut query: Query<
            (&creeper::Charged, &mut tracked_data::TrackedData),
            Changed<creeper::Charged>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_creeper_ignited(
        mut query: Query<
            (&creeper::Ignited, &mut tracked_data::TrackedData),
            Changed<creeper::Ignited>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_start_interpolation(
        mut query: Query<
            (&display::StartInterpolation, &mut tracked_data::TrackedData),
            Changed<display::StartInterpolation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_interpolation_duration(
        mut query: Query<
            (
                &display::InterpolationDuration,
                &mut tracked_data::TrackedData,
            ),
            Changed<display::InterpolationDuration>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_translation(
        mut query: Query<
            (&display::Translation, &mut tracked_data::TrackedData),
            Changed<display::Translation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 26u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 26u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_scale(
        mut query: Query<
            (&display::Scale, &mut tracked_data::TrackedData),
            Changed<display::Scale>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(11u8);
            } else {
                tracked_data.insert_init_value(11u8, 26u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(11u8, 26u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_left_rotation(
        mut query: Query<
            (&display::LeftRotation, &mut tracked_data::TrackedData),
            Changed<display::LeftRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(12u8);
            } else {
                tracked_data.insert_init_value(12u8, 27u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(12u8, 27u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_right_rotation(
        mut query: Query<
            (&display::RightRotation, &mut tracked_data::TrackedData),
            Changed<display::RightRotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(13u8);
            } else {
                tracked_data.insert_init_value(13u8, 27u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(13u8, 27u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_billboard(
        mut query: Query<
            (&display::Billboard, &mut tracked_data::TrackedData),
            Changed<display::Billboard>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(14u8);
            } else {
                tracked_data.insert_init_value(14u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(14u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_brightness(
        mut query: Query<
            (&display::Brightness, &mut tracked_data::TrackedData),
            Changed<display::Brightness>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(15u8);
            } else {
                tracked_data.insert_init_value(15u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(15u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_view_range(
        mut query: Query<
            (&display::ViewRange, &mut tracked_data::TrackedData),
            Changed<display::ViewRange>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_shadow_radius(
        mut query: Query<
            (&display::ShadowRadius, &mut tracked_data::TrackedData),
            Changed<display::ShadowRadius>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_shadow_strength(
        mut query: Query<
            (&display::ShadowStrength, &mut tracked_data::TrackedData),
            Changed<display::ShadowStrength>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_width(
        mut query: Query<
            (&display::Width, &mut tracked_data::TrackedData),
            Changed<display::Width>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_height(
        mut query: Query<
            (&display::Height, &mut tracked_data::TrackedData),
            Changed<display::Height>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_display_glow_color_override(
        mut query: Query<
            (&display::GlowColorOverride, &mut tracked_data::TrackedData),
            Changed<display::GlowColorOverride>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(21u8);
            } else {
                tracked_data.insert_init_value(21u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(21u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_dolphin_treasure_pos(
        mut query: Query<
            (&dolphin::TreasurePos, &mut tracked_data::TrackedData),
            Changed<dolphin::TreasurePos>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 10u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 10u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_dolphin_has_fish(
        mut query: Query<
            (&dolphin::HasFish, &mut tracked_data::TrackedData),
            Changed<dolphin::HasFish>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_dolphin_moistness(
        mut query: Query<
            (&dolphin::Moistness, &mut tracked_data::TrackedData),
            Changed<dolphin::Moistness>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_end_crystal_beam_target(
        mut query: Query<
            (&end_crystal::BeamTarget, &mut tracked_data::TrackedData),
            Changed<end_crystal::BeamTarget>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 11u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 11u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_end_crystal_show_bottom(
        mut query: Query<
            (&end_crystal::ShowBottom, &mut tracked_data::TrackedData),
            Changed<end_crystal::ShowBottom>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_ender_dragon_phase_type(
        mut query: Query<
            (&ender_dragon::PhaseType, &mut tracked_data::TrackedData),
            Changed<ender_dragon::PhaseType>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_enderman_carried_block(
        mut query: Query<
            (&enderman::CarriedBlock, &mut tracked_data::TrackedData),
            Changed<enderman::CarriedBlock>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 15u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 15u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_enderman_angry(
        mut query: Query<
            (&enderman::Angry, &mut tracked_data::TrackedData),
            Changed<enderman::Angry>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_enderman_provoked(
        mut query: Query<
            (&enderman::Provoked, &mut tracked_data::TrackedData),
            Changed<enderman::Provoked>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_flags(
        mut query: Query<(&entity::Flags, &mut tracked_data::TrackedData), Changed<entity::Flags>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(0u8);
            } else {
                tracked_data.insert_init_value(0u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(0u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_air(
        mut query: Query<(&entity::Air, &mut tracked_data::TrackedData), Changed<entity::Air>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(1u8);
            } else {
                tracked_data.insert_init_value(1u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(1u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_custom_name(
        mut query: Query<
            (&entity::CustomName, &mut tracked_data::TrackedData),
            Changed<entity::CustomName>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(2u8);
            } else {
                tracked_data.insert_init_value(2u8, 6u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(2u8, 6u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_name_visible(
        mut query: Query<
            (&entity::NameVisible, &mut tracked_data::TrackedData),
            Changed<entity::NameVisible>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(3u8);
            } else {
                tracked_data.insert_init_value(3u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(3u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_silent(
        mut query: Query<
            (&entity::Silent, &mut tracked_data::TrackedData),
            Changed<entity::Silent>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(4u8);
            } else {
                tracked_data.insert_init_value(4u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(4u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_no_gravity(
        mut query: Query<
            (&entity::NoGravity, &mut tracked_data::TrackedData),
            Changed<entity::NoGravity>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(5u8);
            } else {
                tracked_data.insert_init_value(5u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(5u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_pose(
        mut query: Query<(&entity::Pose, &mut tracked_data::TrackedData), Changed<entity::Pose>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(6u8);
            } else {
                tracked_data.insert_init_value(6u8, 20u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(6u8, 20u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_entity_frozen_ticks(
        mut query: Query<
            (&entity::FrozenTicks, &mut tracked_data::TrackedData),
            Changed<entity::FrozenTicks>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(7u8);
            } else {
                tracked_data.insert_init_value(7u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(7u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_eye_of_ender_item(
        mut query: Query<
            (&eye_of_ender::Item, &mut tracked_data::TrackedData),
            Changed<eye_of_ender::Item>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 7u8, Some(&value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 7u8, Some(&value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_falling_block_block_pos(
        mut query: Query<
            (&falling_block::BlockPos, &mut tracked_data::TrackedData),
            Changed<falling_block::BlockPos>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 10u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 10u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_firework_rocket_item(
        mut query: Query<
            (&firework_rocket::Item, &mut tracked_data::TrackedData),
            Changed<firework_rocket::Item>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 7u8, Some(&value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 7u8, Some(&value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_firework_rocket_shooter_entity_id(
        mut query: Query<
            (
                &firework_rocket::ShooterEntityId,
                &mut tracked_data::TrackedData,
            ),
            Changed<firework_rocket::ShooterEntityId>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 19u8, OptionalInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 19u8, OptionalInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_firework_rocket_shot_at_angle(
        mut query: Query<
            (
                &firework_rocket::ShotAtAngle,
                &mut tracked_data::TrackedData,
            ),
            Changed<firework_rocket::ShotAtAngle>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_fish_from_bucket(
        mut query: Query<
            (&fish::FromBucket, &mut tracked_data::TrackedData),
            Changed<fish::FromBucket>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_fishing_bobber_hook_entity_id(
        mut query: Query<
            (
                &fishing_bobber::HookEntityId,
                &mut tracked_data::TrackedData,
            ),
            Changed<fishing_bobber::HookEntityId>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_fishing_bobber_caught_fish(
        mut query: Query<
            (&fishing_bobber::CaughtFish, &mut tracked_data::TrackedData),
            Changed<fishing_bobber::CaughtFish>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_fox_type(
        mut query: Query<(&fox::Type, &mut tracked_data::TrackedData), Changed<fox::Type>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_fox_fox_flags(
        mut query: Query<(&fox::FoxFlags, &mut tracked_data::TrackedData), Changed<fox::FoxFlags>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_fox_owner(
        mut query: Query<(&fox::Owner, &mut tracked_data::TrackedData), Changed<fox::Owner>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 13u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 13u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_fox_other_trusted(
        mut query: Query<
            (&fox::OtherTrusted, &mut tracked_data::TrackedData),
            Changed<fox::OtherTrusted>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 13u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 13u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_frog_variant(
        mut query: Query<(&frog::Variant, &mut tracked_data::TrackedData), Changed<frog::Variant>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 22u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 22u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_frog_target(
        mut query: Query<(&frog::Target, &mut tracked_data::TrackedData), Changed<frog::Target>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 19u8, OptionalInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 19u8, OptionalInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_furnace_minecart_lit(
        mut query: Query<
            (&furnace_minecart::Lit, &mut tracked_data::TrackedData),
            Changed<furnace_minecart::Lit>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(14u8);
            } else {
                tracked_data.insert_init_value(14u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(14u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_ghast_shooting(
        mut query: Query<
            (&ghast::Shooting, &mut tracked_data::TrackedData),
            Changed<ghast::Shooting>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_glow_squid_dark_ticks_remaining(
        mut query: Query<
            (
                &glow_squid::DarkTicksRemaining,
                &mut tracked_data::TrackedData,
            ),
            Changed<glow_squid::DarkTicksRemaining>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_goat_screaming(
        mut query: Query<
            (&goat::Screaming, &mut tracked_data::TrackedData),
            Changed<goat::Screaming>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_goat_left_horn(
        mut query: Query<
            (&goat::LeftHorn, &mut tracked_data::TrackedData),
            Changed<goat::LeftHorn>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_goat_right_horn(
        mut query: Query<
            (&goat::RightHorn, &mut tracked_data::TrackedData),
            Changed<goat::RightHorn>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_guardian_spikes_retracted(
        mut query: Query<
            (&guardian::SpikesRetracted, &mut tracked_data::TrackedData),
            Changed<guardian::SpikesRetracted>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_guardian_beam_target_id(
        mut query: Query<
            (&guardian::BeamTargetId, &mut tracked_data::TrackedData),
            Changed<guardian::BeamTargetId>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_hoglin_baby(
        mut query: Query<(&hoglin::Baby, &mut tracked_data::TrackedData), Changed<hoglin::Baby>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_horse_variant(
        mut query: Query<
            (&horse::Variant, &mut tracked_data::TrackedData),
            Changed<horse::Variant>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_interaction_width(
        mut query: Query<
            (&interaction::Width, &mut tracked_data::TrackedData),
            Changed<interaction::Width>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_interaction_height(
        mut query: Query<
            (&interaction::Height, &mut tracked_data::TrackedData),
            Changed<interaction::Height>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_interaction_response(
        mut query: Query<
            (&interaction::Response, &mut tracked_data::TrackedData),
            Changed<interaction::Response>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_iron_golem_iron_golem_flags(
        mut query: Query<
            (&iron_golem::IronGolemFlags, &mut tracked_data::TrackedData),
            Changed<iron_golem::IronGolemFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_item_display_item(
        mut query: Query<
            (&item_display::Item, &mut tracked_data::TrackedData),
            Changed<item_display::Item>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(22u8);
            } else {
                tracked_data.insert_init_value(22u8, 7u8, Some(&value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(22u8, 7u8, Some(&value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_item_display_item_display(
        mut query: Query<
            (&item_display::ItemDisplay, &mut tracked_data::TrackedData),
            Changed<item_display::ItemDisplay>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(23u8);
            } else {
                tracked_data.insert_init_value(23u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(23u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_item_stack(
        mut query: Query<(&item::Stack, &mut tracked_data::TrackedData), Changed<item::Stack>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 7u8, Some(&value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 7u8, Some(&value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_item_frame_item_stack(
        mut query: Query<
            (&item_frame::ItemStack, &mut tracked_data::TrackedData),
            Changed<item_frame::ItemStack>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 7u8, Some(&value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 7u8, Some(&value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_item_frame_rotation(
        mut query: Query<
            (&item_frame::Rotation, &mut tracked_data::TrackedData),
            Changed<item_frame::Rotation>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_living_living_flags(
        mut query: Query<
            (&living::LivingFlags, &mut tracked_data::TrackedData),
            Changed<living::LivingFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_living_health(
        mut query: Query<
            (&living::Health, &mut tracked_data::TrackedData),
            Changed<living::Health>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_living_potion_swirls_color(
        mut query: Query<
            (&living::PotionSwirlsColor, &mut tracked_data::TrackedData),
            Changed<living::PotionSwirlsColor>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_living_potion_swirls_ambient(
        mut query: Query<
            (&living::PotionSwirlsAmbient, &mut tracked_data::TrackedData),
            Changed<living::PotionSwirlsAmbient>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(11u8);
            } else {
                tracked_data.insert_init_value(11u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(11u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_living_stuck_arrow_count(
        mut query: Query<
            (&living::StuckArrowCount, &mut tracked_data::TrackedData),
            Changed<living::StuckArrowCount>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(12u8);
            } else {
                tracked_data.insert_init_value(12u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(12u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_living_stinger_count(
        mut query: Query<
            (&living::StingerCount, &mut tracked_data::TrackedData),
            Changed<living::StingerCount>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(13u8);
            } else {
                tracked_data.insert_init_value(13u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(13u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_living_sleeping_position(
        mut query: Query<
            (&living::SleepingPosition, &mut tracked_data::TrackedData),
            Changed<living::SleepingPosition>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(14u8);
            } else {
                tracked_data.insert_init_value(14u8, 11u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(14u8, 11u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_llama_strength(
        mut query: Query<
            (&llama::Strength, &mut tracked_data::TrackedData),
            Changed<llama::Strength>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_llama_carpet_color(
        mut query: Query<
            (&llama::CarpetColor, &mut tracked_data::TrackedData),
            Changed<llama::CarpetColor>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_llama_variant(
        mut query: Query<
            (&llama::Variant, &mut tracked_data::TrackedData),
            Changed<llama::Variant>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(21u8);
            } else {
                tracked_data.insert_init_value(21u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(21u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_merchant_head_rolling_time_left(
        mut query: Query<
            (
                &merchant::HeadRollingTimeLeft,
                &mut tracked_data::TrackedData,
            ),
            Changed<merchant::HeadRollingTimeLeft>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_mob_mob_flags(
        mut query: Query<(&mob::MobFlags, &mut tracked_data::TrackedData), Changed<mob::MobFlags>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(15u8);
            } else {
                tracked_data.insert_init_value(15u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(15u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_mooshroom_type(
        mut query: Query<
            (&mooshroom::Type, &mut tracked_data::TrackedData),
            Changed<mooshroom::Type>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 4u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 4u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_ocelot_trusting(
        mut query: Query<
            (&ocelot::Trusting, &mut tracked_data::TrackedData),
            Changed<ocelot::Trusting>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_painting_variant(
        mut query: Query<
            (&painting::Variant, &mut tracked_data::TrackedData),
            Changed<painting::Variant>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 24u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 24u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_panda_ask_for_bamboo_ticks(
        mut query: Query<
            (&panda::AskForBambooTicks, &mut tracked_data::TrackedData),
            Changed<panda::AskForBambooTicks>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_panda_sneeze_progress(
        mut query: Query<
            (&panda::SneezeProgress, &mut tracked_data::TrackedData),
            Changed<panda::SneezeProgress>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_panda_eating_ticks(
        mut query: Query<
            (&panda::EatingTicks, &mut tracked_data::TrackedData),
            Changed<panda::EatingTicks>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_panda_main_gene(
        mut query: Query<
            (&panda::MainGene, &mut tracked_data::TrackedData),
            Changed<panda::MainGene>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_panda_hidden_gene(
        mut query: Query<
            (&panda::HiddenGene, &mut tracked_data::TrackedData),
            Changed<panda::HiddenGene>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(21u8);
            } else {
                tracked_data.insert_init_value(21u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(21u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_panda_panda_flags(
        mut query: Query<
            (&panda::PandaFlags, &mut tracked_data::TrackedData),
            Changed<panda::PandaFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(22u8);
            } else {
                tracked_data.insert_init_value(22u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(22u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_parrot_variant(
        mut query: Query<
            (&parrot::Variant, &mut tracked_data::TrackedData),
            Changed<parrot::Variant>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_passive_child(
        mut query: Query<
            (&passive::Child, &mut tracked_data::TrackedData),
            Changed<passive::Child>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_persistent_projectile_projectile_flags(
        mut query: Query<
            (
                &persistent_projectile::ProjectileFlags,
                &mut tracked_data::TrackedData,
            ),
            Changed<persistent_projectile::ProjectileFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_persistent_projectile_pierce_level(
        mut query: Query<
            (
                &persistent_projectile::PierceLevel,
                &mut tracked_data::TrackedData,
            ),
            Changed<persistent_projectile::PierceLevel>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(9u8);
            } else {
                tracked_data.insert_init_value(9u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(9u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_phantom_size(
        mut query: Query<(&phantom::Size, &mut tracked_data::TrackedData), Changed<phantom::Size>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_pig_saddled(
        mut query: Query<(&pig::Saddled, &mut tracked_data::TrackedData), Changed<pig::Saddled>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_pig_boost_time(
        mut query: Query<
            (&pig::BoostTime, &mut tracked_data::TrackedData),
            Changed<pig::BoostTime>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_piglin_baby(
        mut query: Query<(&piglin::Baby, &mut tracked_data::TrackedData), Changed<piglin::Baby>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_piglin_charging(
        mut query: Query<
            (&piglin::Charging, &mut tracked_data::TrackedData),
            Changed<piglin::Charging>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_piglin_dancing(
        mut query: Query<
            (&piglin::Dancing, &mut tracked_data::TrackedData),
            Changed<piglin::Dancing>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_pillager_charging(
        mut query: Query<
            (&pillager::Charging, &mut tracked_data::TrackedData),
            Changed<pillager::Charging>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_player_absorption_amount(
        mut query: Query<
            (&player::AbsorptionAmount, &mut tracked_data::TrackedData),
            Changed<player::AbsorptionAmount>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(15u8);
            } else {
                tracked_data.insert_init_value(15u8, 3u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(15u8, 3u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_player_score(
        mut query: Query<(&player::Score, &mut tracked_data::TrackedData), Changed<player::Score>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_player_player_model_parts(
        mut query: Query<
            (&player::PlayerModelParts, &mut tracked_data::TrackedData),
            Changed<player::PlayerModelParts>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_player_main_arm(
        mut query: Query<
            (&player::MainArm, &mut tracked_data::TrackedData),
            Changed<player::MainArm>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_player_left_shoulder_entity(
        mut query: Query<
            (&player::LeftShoulderEntity, &mut tracked_data::TrackedData),
            Changed<player::LeftShoulderEntity>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 16u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 16u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_player_right_shoulder_entity(
        mut query: Query<
            (&player::RightShoulderEntity, &mut tracked_data::TrackedData),
            Changed<player::RightShoulderEntity>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 16u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 16u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_polar_bear_warning(
        mut query: Query<
            (&polar_bear::Warning, &mut tracked_data::TrackedData),
            Changed<polar_bear::Warning>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_pufferfish_puff_state(
        mut query: Query<
            (&pufferfish::PuffState, &mut tracked_data::TrackedData),
            Changed<pufferfish::PuffState>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_rabbit_rabbit_type(
        mut query: Query<
            (&rabbit::RabbitType, &mut tracked_data::TrackedData),
            Changed<rabbit::RabbitType>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_raider_celebrating(
        mut query: Query<
            (&raider::Celebrating, &mut tracked_data::TrackedData),
            Changed<raider::Celebrating>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_sheep_color(
        mut query: Query<(&sheep::Color, &mut tracked_data::TrackedData), Changed<sheep::Color>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_shulker_attached_face(
        mut query: Query<
            (&shulker::AttachedFace, &mut tracked_data::TrackedData),
            Changed<shulker::AttachedFace>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 12u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 12u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_shulker_peek_amount(
        mut query: Query<
            (&shulker::PeekAmount, &mut tracked_data::TrackedData),
            Changed<shulker::PeekAmount>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_shulker_color(
        mut query: Query<
            (&shulker::Color, &mut tracked_data::TrackedData),
            Changed<shulker::Color>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_skeleton_converting(
        mut query: Query<
            (&skeleton::Converting, &mut tracked_data::TrackedData),
            Changed<skeleton::Converting>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_slime_slime_size(
        mut query: Query<
            (&slime::SlimeSize, &mut tracked_data::TrackedData),
            Changed<slime::SlimeSize>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_sniffer_state(
        mut query: Query<
            (&sniffer::State, &mut tracked_data::TrackedData),
            Changed<sniffer::State>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 25u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 25u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_sniffer_finish_dig_time(
        mut query: Query<
            (&sniffer::FinishDigTime, &mut tracked_data::TrackedData),
            Changed<sniffer::FinishDigTime>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_snow_golem_snow_golem_flags(
        mut query: Query<
            (&snow_golem::SnowGolemFlags, &mut tracked_data::TrackedData),
            Changed<snow_golem::SnowGolemFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_spellcasting_illager_spell(
        mut query: Query<
            (&spellcasting_illager::Spell, &mut tracked_data::TrackedData),
            Changed<spellcasting_illager::Spell>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_spider_spider_flags(
        mut query: Query<
            (&spider::SpiderFlags, &mut tracked_data::TrackedData),
            Changed<spider::SpiderFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_strider_boost_time(
        mut query: Query<
            (&strider::BoostTime, &mut tracked_data::TrackedData),
            Changed<strider::BoostTime>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_strider_cold(
        mut query: Query<(&strider::Cold, &mut tracked_data::TrackedData), Changed<strider::Cold>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_strider_saddled(
        mut query: Query<
            (&strider::Saddled, &mut tracked_data::TrackedData),
            Changed<strider::Saddled>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_tameable_tameable_flags(
        mut query: Query<
            (&tameable::TameableFlags, &mut tracked_data::TrackedData),
            Changed<tameable::TameableFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_tameable_owner_uuid(
        mut query: Query<
            (&tameable::OwnerUuid, &mut tracked_data::TrackedData),
            Changed<tameable::OwnerUuid>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 13u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 13u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_text_display_text(
        mut query: Query<
            (&text_display::Text, &mut tracked_data::TrackedData),
            Changed<text_display::Text>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(22u8);
            } else {
                tracked_data.insert_init_value(22u8, 5u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(22u8, 5u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_text_display_line_width(
        mut query: Query<
            (&text_display::LineWidth, &mut tracked_data::TrackedData),
            Changed<text_display::LineWidth>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(23u8);
            } else {
                tracked_data.insert_init_value(23u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(23u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_text_display_background(
        mut query: Query<
            (&text_display::Background, &mut tracked_data::TrackedData),
            Changed<text_display::Background>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(24u8);
            } else {
                tracked_data.insert_init_value(24u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(24u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_text_display_text_opacity(
        mut query: Query<
            (&text_display::TextOpacity, &mut tracked_data::TrackedData),
            Changed<text_display::TextOpacity>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(25u8);
            } else {
                tracked_data.insert_init_value(25u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(25u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_text_display_text_display_flags(
        mut query: Query<
            (
                &text_display::TextDisplayFlags,
                &mut tracked_data::TrackedData,
            ),
            Changed<text_display::TextDisplayFlags>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(26u8);
            } else {
                tracked_data.insert_init_value(26u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(26u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_thrown_item_item(
        mut query: Query<
            (&thrown_item::Item, &mut tracked_data::TrackedData),
            Changed<thrown_item::Item>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 7u8, Some(&value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 7u8, Some(&value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_tnt_fuse(
        mut query: Query<(&tnt::Fuse, &mut tracked_data::TrackedData), Changed<tnt::Fuse>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_trident_loyalty(
        mut query: Query<
            (&trident::Loyalty, &mut tracked_data::TrackedData),
            Changed<trident::Loyalty>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(10u8);
            } else {
                tracked_data.insert_init_value(10u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(10u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_trident_enchanted(
        mut query: Query<
            (&trident::Enchanted, &mut tracked_data::TrackedData),
            Changed<trident::Enchanted>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(11u8);
            } else {
                tracked_data.insert_init_value(11u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(11u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_tropical_fish_variant(
        mut query: Query<
            (&tropical_fish::Variant, &mut tracked_data::TrackedData),
            Changed<tropical_fish::Variant>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_turtle_home_pos(
        mut query: Query<
            (&turtle::HomePos, &mut tracked_data::TrackedData),
            Changed<turtle::HomePos>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 10u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 10u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_turtle_has_egg(
        mut query: Query<
            (&turtle::HasEgg, &mut tracked_data::TrackedData),
            Changed<turtle::HasEgg>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_turtle_digging_sand(
        mut query: Query<
            (&turtle::DiggingSand, &mut tracked_data::TrackedData),
            Changed<turtle::DiggingSand>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_turtle_travel_pos(
        mut query: Query<
            (&turtle::TravelPos, &mut tracked_data::TrackedData),
            Changed<turtle::TravelPos>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 10u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 10u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_turtle_land_bound(
        mut query: Query<
            (&turtle::LandBound, &mut tracked_data::TrackedData),
            Changed<turtle::LandBound>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(21u8);
            } else {
                tracked_data.insert_init_value(21u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(21u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_turtle_actively_traveling(
        mut query: Query<
            (&turtle::ActivelyTraveling, &mut tracked_data::TrackedData),
            Changed<turtle::ActivelyTraveling>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(22u8);
            } else {
                tracked_data.insert_init_value(22u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(22u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_vex_vex_flags(
        mut query: Query<(&vex::VexFlags, &mut tracked_data::TrackedData), Changed<vex::VexFlags>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 0u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 0u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_villager_villager_data(
        mut query: Query<
            (&villager::VillagerData, &mut tracked_data::TrackedData),
            Changed<villager::VillagerData>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 18u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 18u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_warden_anger(
        mut query: Query<(&warden::Anger, &mut tracked_data::TrackedData), Changed<warden::Anger>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_witch_drinking(
        mut query: Query<
            (&witch::Drinking, &mut tracked_data::TrackedData),
            Changed<witch::Drinking>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wither_tracked_entity_id_1(
        mut query: Query<
            (&wither::TrackedEntityId1, &mut tracked_data::TrackedData),
            Changed<wither::TrackedEntityId1>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wither_tracked_entity_id_2(
        mut query: Query<
            (&wither::TrackedEntityId2, &mut tracked_data::TrackedData),
            Changed<wither::TrackedEntityId2>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wither_tracked_entity_id_3(
        mut query: Query<
            (&wither::TrackedEntityId3, &mut tracked_data::TrackedData),
            Changed<wither::TrackedEntityId3>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wither_invul_timer(
        mut query: Query<
            (&wither::InvulTimer, &mut tracked_data::TrackedData),
            Changed<wither::InvulTimer>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wither_skull_charged(
        mut query: Query<
            (&wither_skull::Charged, &mut tracked_data::TrackedData),
            Changed<wither_skull::Charged>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(8u8);
            } else {
                tracked_data.insert_init_value(8u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(8u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wolf_begging(
        mut query: Query<(&wolf::Begging, &mut tracked_data::TrackedData), Changed<wolf::Begging>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wolf_collar_color(
        mut query: Query<
            (&wolf::CollarColor, &mut tracked_data::TrackedData),
            Changed<wolf::CollarColor>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_wolf_anger_time(
        mut query: Query<
            (&wolf::AngerTime, &mut tracked_data::TrackedData),
            Changed<wolf::AngerTime>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(21u8);
            } else {
                tracked_data.insert_init_value(21u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(21u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_zoglin_baby(
        mut query: Query<(&zoglin::Baby, &mut tracked_data::TrackedData), Changed<zoglin::Baby>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_zombie_baby(
        mut query: Query<(&zombie::Baby, &mut tracked_data::TrackedData), Changed<zombie::Baby>>,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(16u8);
            } else {
                tracked_data.insert_init_value(16u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(16u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_zombie_zombie_type(
        mut query: Query<
            (&zombie::ZombieType, &mut tracked_data::TrackedData),
            Changed<zombie::ZombieType>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(17u8);
            } else {
                tracked_data.insert_init_value(17u8, 1u8, VarInt(value.0));
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(17u8, 1u8, VarInt(value.0));
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_zombie_converting_in_water(
        mut query: Query<
            (&zombie::ConvertingInWater, &mut tracked_data::TrackedData),
            Changed<zombie::ConvertingInWater>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(18u8);
            } else {
                tracked_data.insert_init_value(18u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(18u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_zombie_villager_converting(
        mut query: Query<
            (&zombie_villager::Converting, &mut tracked_data::TrackedData),
            Changed<zombie_villager::Converting>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(19u8);
            } else {
                tracked_data.insert_init_value(19u8, 8u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(19u8, 8u8, &value.0);
            }
        }
    }
    #[allow(clippy::needless_borrow)]
    #[allow(clippy::suspicious_else_formatting)]
    fn update_zombie_villager_villager_data(
        mut query: Query<
            (
                &zombie_villager::VillagerData,
                &mut tracked_data::TrackedData,
            ),
            Changed<zombie_villager::VillagerData>,
        >,
    ) {
        for (value, mut tracked_data) in &mut query {
            if *value == Default::default() {
                tracked_data.remove_init_value(20u8);
            } else {
                tracked_data.insert_init_value(20u8, 18u8, &value.0);
            }
            if !tracked_data.is_added() {
                tracked_data.append_update_value(20u8, 18u8, &value.0);
            }
        }
    }
    app.add_systems(
        PostUpdate,
        update_abstract_donkey_chest
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_fireball_item
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_horse_horse_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_minecart_damage_wobble_ticks
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_minecart_damage_wobble_side
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_minecart_damage_wobble_strength
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_minecart_custom_block_id
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_minecart_custom_block_offset
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_minecart_custom_block_present
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_abstract_piglin_immune_to_zombification
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_allay_dancing
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_allay_can_duplicate
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_area_effect_cloud_radius
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_area_effect_cloud_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_area_effect_cloud_waiting
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_area_effect_cloud_particle_id
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_armor_stand_armor_stand_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_armor_stand_tracker_head_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_armor_stand_tracker_body_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_armor_stand_tracker_left_arm_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_armor_stand_tracker_right_arm_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_armor_stand_tracker_left_leg_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_armor_stand_tracker_right_leg_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_arrow_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_axolotl_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_axolotl_playing_dead
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_axolotl_from_bucket
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_bat_bat_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_bee_bee_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_bee_anger
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_blaze_blaze_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_block_display_block_state
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_boat_damage_wobble_ticks
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_boat_damage_wobble_side
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_boat_damage_wobble_strength
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_boat_boat_type
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_boat_left_paddle_moving
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_boat_right_paddle_moving
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_boat_bubble_wobble_ticks
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_camel_dashing
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_camel_last_pose_tick
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_cat_cat_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_cat_in_sleeping_pose
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_cat_head_down
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_cat_collar_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_command_block_minecart_command
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_command_block_minecart_last_output
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_creeper_fuse_speed
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_creeper_charged
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_creeper_ignited
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_start_interpolation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_interpolation_duration
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_translation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_scale
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_left_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_right_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_billboard
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_brightness
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_view_range
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_shadow_radius
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_shadow_strength
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_width
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_height
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_display_glow_color_override
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_dolphin_treasure_pos
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_dolphin_has_fish
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_dolphin_moistness
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_end_crystal_beam_target
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_end_crystal_show_bottom
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_ender_dragon_phase_type
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_enderman_carried_block
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_enderman_angry
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_enderman_provoked
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_air
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_custom_name
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_name_visible
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_silent
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_no_gravity
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_pose
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_entity_frozen_ticks
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_eye_of_ender_item
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_falling_block_block_pos
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_firework_rocket_item
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_firework_rocket_shooter_entity_id
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_firework_rocket_shot_at_angle
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_fish_from_bucket
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_fishing_bobber_hook_entity_id
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_fishing_bobber_caught_fish
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_fox_type
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_fox_fox_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_fox_owner
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_fox_other_trusted
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_frog_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_frog_target
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_furnace_minecart_lit
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_ghast_shooting
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_glow_squid_dark_ticks_remaining
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_goat_screaming
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_goat_left_horn
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_goat_right_horn
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_guardian_spikes_retracted
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_guardian_beam_target_id
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_hoglin_baby
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_horse_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_interaction_width
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_interaction_height
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_interaction_response
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_iron_golem_iron_golem_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_item_display_item
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_item_display_item_display
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_item_stack
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_item_frame_item_stack
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_item_frame_rotation
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_living_living_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_living_health
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_living_potion_swirls_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_living_potion_swirls_ambient
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_living_stuck_arrow_count
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_living_stinger_count
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_living_sleeping_position
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_llama_strength
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_llama_carpet_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_llama_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_merchant_head_rolling_time_left
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_mob_mob_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_mooshroom_type
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_ocelot_trusting
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_painting_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_panda_ask_for_bamboo_ticks
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_panda_sneeze_progress
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_panda_eating_ticks
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_panda_main_gene
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_panda_hidden_gene
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_panda_panda_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_parrot_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_passive_child
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_persistent_projectile_projectile_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_persistent_projectile_pierce_level
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_phantom_size
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_pig_saddled
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_pig_boost_time
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_piglin_baby
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_piglin_charging
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_piglin_dancing
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_pillager_charging
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_player_absorption_amount
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_player_score
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_player_player_model_parts
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_player_main_arm
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_player_left_shoulder_entity
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_player_right_shoulder_entity
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_polar_bear_warning
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_pufferfish_puff_state
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_rabbit_rabbit_type
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_raider_celebrating
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_sheep_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_shulker_attached_face
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_shulker_peek_amount
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_shulker_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_skeleton_converting
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_slime_slime_size
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_sniffer_state
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_sniffer_finish_dig_time
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_snow_golem_snow_golem_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_spellcasting_illager_spell
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_spider_spider_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_strider_boost_time
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_strider_cold
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_strider_saddled
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_tameable_tameable_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_tameable_owner_uuid
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_text_display_text
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_text_display_line_width
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_text_display_background
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_text_display_text_opacity
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_text_display_text_display_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_thrown_item_item
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_tnt_fuse
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_trident_loyalty
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_trident_enchanted
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_tropical_fish_variant
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_turtle_home_pos
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_turtle_has_egg
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_turtle_digging_sand
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_turtle_travel_pos
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_turtle_land_bound
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_turtle_actively_traveling
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_vex_vex_flags
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_villager_villager_data
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_warden_anger
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_witch_drinking
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wither_tracked_entity_id_1
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wither_tracked_entity_id_2
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wither_tracked_entity_id_3
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wither_invul_timer
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wither_skull_charged
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wolf_begging
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wolf_collar_color
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_wolf_anger_time
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_zoglin_baby
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_zombie_baby
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_zombie_zombie_type
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_zombie_converting_in_water
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_zombie_villager_converting
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
    app.add_systems(
        PostUpdate,
        update_zombie_villager_villager_data
            .in_set(UpdateTrackedDataSet)
            .ambiguous_with(UpdateTrackedDataSet),
    );
}
