// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiStencilProperty {
    pub base: NiProperty,
    pub stencil_enabled: bool,
    pub stencil_function: StencilTestFunction,
    pub stencil_ref: u32,
    #[default(0xFFFFFFFF)]
    pub stencil_mask: u32,
    pub fail_action: Action,
    pub pass_z_fail_action: Action,
    pub pass_action: Action,
    pub draw_mode: DrawMode,
}

impl Load for NiStencilProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let stencil_enabled = stream.load::<u8>()? != 0;
        let stencil_function = stream.load()?;
        let stencil_ref = stream.load()?;
        let stencil_mask = stream.load()?;
        let fail_action = stream.load()?;
        let pass_z_fail_action = stream.load()?;
        let pass_action = stream.load()?;
        let draw_mode = stream.load()?;
        Ok(Self {
            base,
            stencil_enabled,
            stencil_function,
            stencil_ref,
            stencil_mask,
            fail_action,
            pass_z_fail_action,
            pass_action,
            draw_mode,
        })
    }
}

impl Save for NiStencilProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u8>(self.stencil_enabled)?;
        stream.save(&self.stencil_function)?;
        stream.save(&self.stencil_ref)?;
        stream.save(&self.stencil_mask)?;
        stream.save(&self.fail_action)?;
        stream.save(&self.pass_z_fail_action)?;
        stream.save(&self.pass_action)?;
        stream.save(&self.draw_mode)?;
        Ok(())
    }
}
