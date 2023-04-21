// internal imports
use crate::prelude::*;

#[allow(clippy::struct_excessive_bools)]
#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiParticleSystemController {
    pub base: NiTimeController,
    pub speed: f32,
    pub speed_variation: f32,
    pub declination_angle: f32,
    pub declination_variation: f32,
    pub planar_angle: f32,
    pub planar_angle_variation: f32,
    pub initial_normal: Vec3,
    pub initial_color: ColorA,
    pub initial_size: f32,
    pub emit_start_time: f32,
    pub emit_stop_time: f32,
    pub reset_particle_system: bool,
    pub birth_rate: f32,
    pub lifespan: f32,
    pub lifespan_variation: f32,
    pub use_birth_rate: bool,
    pub spawn_on_death: bool,
    pub emitter_width: f32,
    pub emitter_height: f32,
    pub emitter_depth: f32,
    pub emitter: NiLink<NiAVObject>,
    pub spawn_generations: u16,
    pub spawn_percentage: f32,
    pub spawn_multiplier: u16,
    pub spawned_speed_chaos: f32,
    pub spawned_direction_chaos: f32,
    pub particles: Vec<NiPerParticleData>,
    pub num_active_particles: u16,
    pub emitter_modifier: NiLink<NiEmitterModifier>,
    pub particle_modifier: NiLink<NiParticleModifier>,
    pub particle_collider: NiLink<NiParticleCollider>,
    pub compute_dynamic_bounding_volume: bool,
}

impl Load for NiParticleSystemController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let speed = stream.load()?;
        let speed_variation = stream.load()?;
        let declination_angle = stream.load()?;
        let declination_variation = stream.load()?;
        let planar_angle = stream.load()?;
        let planar_angle_variation = stream.load()?;
        let initial_normal = stream.load()?;
        let initial_color = stream.load()?;
        let initial_size = stream.load()?;
        let emit_start_time = stream.load()?;
        let emit_stop_time = stream.load()?;
        let reset_particle_system = stream.load::<u8>()? != 0;
        let birth_rate = stream.load()?;
        let lifespan = stream.load()?;
        let lifespan_variation = stream.load()?;
        let use_birth_rate = stream.load::<u8>()? != 0;
        let spawn_on_death = stream.load::<u8>()? != 0;
        let emitter_width = stream.load()?;
        let emitter_height = stream.load()?;
        let emitter_depth = stream.load()?;
        let emitter = stream.load()?;
        let spawn_generations = stream.load()?;
        let spawn_percentage = stream.load()?;
        let spawn_multiplier = stream.load()?;
        let spawned_speed_chaos = stream.load()?;
        let spawned_direction_chaos = stream.load()?;
        let num_particles: u16 = stream.load()?;
        let particles = stream.load_seq(num_particles)?;
        let num_active_particles = stream.load()?;
        let emitter_modifier = stream.load()?;
        let particle_modifier = stream.load()?;
        let particle_collider = stream.load()?;
        let compute_dynamic_bounding_volume = stream.load::<u8>()? != 0;
        Ok(Self {
            base,
            speed,
            speed_variation,
            declination_angle,
            declination_variation,
            planar_angle,
            planar_angle_variation,
            initial_normal,
            initial_color,
            initial_size,
            emit_start_time,
            emit_stop_time,
            reset_particle_system,
            birth_rate,
            lifespan,
            lifespan_variation,
            use_birth_rate,
            spawn_on_death,
            emitter_width,
            emitter_height,
            emitter_depth,
            emitter,
            spawn_generations,
            spawn_percentage,
            spawn_multiplier,
            spawned_speed_chaos,
            spawned_direction_chaos,
            particles,
            num_active_particles,
            emitter_modifier,
            particle_modifier,
            particle_collider,
            compute_dynamic_bounding_volume,
        })
    }
}

impl Save for NiParticleSystemController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.speed)?;
        stream.save(&self.speed_variation)?;
        stream.save(&self.declination_angle)?;
        stream.save(&self.declination_variation)?;
        stream.save(&self.planar_angle)?;
        stream.save(&self.planar_angle_variation)?;
        stream.save(&self.initial_normal)?;
        stream.save(&self.initial_color)?;
        stream.save(&self.initial_size)?;
        stream.save(&self.emit_start_time)?;
        stream.save(&self.emit_stop_time)?;
        stream.save_as::<u8>(self.reset_particle_system)?;
        stream.save(&self.birth_rate)?;
        stream.save(&self.lifespan)?;
        stream.save(&self.lifespan_variation)?;
        stream.save_as::<u8>(self.use_birth_rate)?;
        stream.save_as::<u8>(self.spawn_on_death)?;
        stream.save(&self.emitter_width)?;
        stream.save(&self.emitter_height)?;
        stream.save(&self.emitter_depth)?;
        stream.save(&self.emitter)?;
        stream.save(&self.spawn_generations)?;
        stream.save(&self.spawn_percentage)?;
        stream.save(&self.spawn_multiplier)?;
        stream.save(&self.spawned_speed_chaos)?;
        stream.save(&self.spawned_direction_chaos)?;
        stream.save_as::<u16>(self.particles.len())?;
        stream.save_seq(&self.particles)?;
        stream.save(&self.num_active_particles)?;
        stream.save(&self.emitter_modifier)?;
        stream.save(&self.particle_modifier)?;
        stream.save(&self.particle_collider)?;
        stream.save_as::<u8>(self.compute_dynamic_bounding_volume)?;
        Ok(())
    }
}
