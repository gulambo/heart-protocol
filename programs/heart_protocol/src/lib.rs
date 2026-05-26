use anchor_lang::prelude::*;

declare_id!("HrtProto1111111111111111111111111111111111");

#[program]
pub mod heart_protocol {
    use super::*;

    pub fn register_unit(ctx: Context<RegisterUnit>, unit_id: String) -> Result<()> {
        let unit = &mut ctx.accounts.robot_unit;
        unit.unit_id = unit_id;
        unit.is_active = true;
        unit.status = UnitStatus::Normal;
        unit.authorized_supervisor = *ctx.accounts.supervisor.key;
        Ok(())
    }

    pub fn request_override(ctx: Context<RequestOverride>, hazard_code: u8) -> Result<()> {
        let unit = &mut ctx.accounts.robot_unit;
        unit.status = UnitStatus::Paused; 
        
        emit!(SafetyOverrideRequested {
            unit_id: unit.unit_id.clone(),
            hazard_code,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }

    pub fn authorize_poh(ctx: Context<AuthorizePoH>, _zk_proof_hash: [u8; 32]) -> Result<()> {
        let unit = &mut ctx.accounts.robot_unit;
        require!(unit.status == UnitStatus::Paused, ProtocolError::UnitNotPaused);
        require!(unit.authorized_supervisor == *ctx.accounts.supervisor.key, ProtocolError::UnauthorizedHeart);

        unit.status = UnitStatus::Normal; 
        
        emit!(PoHAuthorizationGranted {
            unit_id: unit.unit_id.clone(),
            supervisor: *ctx.accounts.supervisor.key,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum UnitStatus {
    Normal,
    Paused,
    Maintenance,
}

#[account]
pub struct RobotUnit {
    pub unit_id: String,
    pub is_active: bool,
    pub status: UnitStatus,
    pub authorized_supervisor: Pubkey,
}

#[derive(Accounts)]
pub struct RegisterUnit<'info> {
    #[account(init, payer = supervisor, space = 8 + 32 + 1 + 2 + 32)]
    pub robot_unit: Account<'info, RobotUnit>,
    #[account(mut)]
    pub supervisor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestOverride<'info> {
    #[account(mut)]
    pub robot_unit: Account<'info, RobotUnit>,
}

#[derive(Accounts)]
pub struct AuthorizePoH<'info> {
    #[account(mut)]
    pub robot_unit: Account<'info, RobotUnit>,
    pub supervisor: Signer<'info>,
}

#[event]
pub struct SafetyOverrideRequested {
    pub unit_id: String,
    pub hazard_code: u8,
    pub timestamp: i64,
}

#[event]
pub struct PoHAuthorizationGranted {
    pub unit_id: String,
    pub supervisor: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum ProtocolError {
    #[msg("The target robotic unit is not in a paused safety state.")]
    UnitNotPaused,
    #[msg("The biometric signature does not match the assigned human supervisor.")]
    UnauthorizedHeart,
}
