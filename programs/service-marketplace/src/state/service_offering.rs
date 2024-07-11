use anchor_lang::prelude::*;
use crate::ServiceOfferingError;

#[account]
pub struct ServiceOffering {
    // The public key of the vendor offering the service
    pub vendor: Pubkey,

    // The public key of the associated NFT asset
    pub asset_id: Pubkey,

    // The type of service (OneTime, potentially Subscription in the future)
    pub service_type: ServiceType,

    // The number of times this service has been sold
    pub num_sold: u64,

    // The maximum number of times this service can be sold (0 for unlimited)
    pub max_quantity: u64,

    // Whether the service offering is currently active and available for purchase
    pub active: bool,

    // The price of the service in lamports (1 SOL = 1_000_000_000 lamports)
    pub sol_price: u64,

    // Timestamp when the service was created (useful for sorting and tracking)
    pub created_at: i64,

    // Optional expiration timestamp for time-limited offerings
    pub expires_at: Option<i64>,

    // The bump used in PDA derivation
    pub bump: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum ServiceType {
    OneTime,
    // Subscription, // TODO: Add subscription
}

impl Default for ServiceType {
    fn default() -> Self {
        Self::OneTime
    }
}

impl ServiceOffering {
    pub fn get_size() -> usize {
        8 +     // discriminator
        32 +    // vendor
        32 +    // asset_id
        2 +     // service_type
        8 +     // num_sold
        8 +     // max_quantity
        1 +     // active
        8 +     // sol_price
        8 +     // created_at
        9 +     // expires_at (1 byte for Option enum + 8 bytes for i64)
        1      // bump
    }

    pub fn create(
        &mut self,
        vendor: Pubkey,
        asset_id: Pubkey,
        max_quantity: u64,
        sol_price: u64,
        expires_at: Option<i64>,
        bump: u8,
    ) {
        self.vendor = vendor;
        self.asset_id = asset_id;
        self.service_type = ServiceType::default();
        self.num_sold = 0;
        self.active = true;
        self.sol_price = sol_price;
        self.max_quantity = max_quantity;
        self.created_at = Clock::get().unwrap().unix_timestamp;
        self.expires_at = expires_at;
        self.bump = bump;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn update_sol_price(&mut self, new_price: u64) {
        self.sol_price = new_price;
    }

    pub fn update_max_quantity(&mut self, new_quantity: u64) {
        self.max_quantity = new_quantity;
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expires_at {
            Clock::get().unwrap().unix_timestamp >= expiry
        } else {
            false
        }
    }

    pub fn is_sold_out(&self) -> bool {
        self.max_quantity > 0 && self.num_sold >= self.max_quantity
    }

    pub fn is_active(&self) -> bool {
        self.active && !self.is_expired() && !self.is_sold_out()
    }

    pub fn increment_sold(&mut self) -> Result<()> {
        require!(self.is_active(), ServiceOfferingError::ServiceNotActive);
        require!(!self.is_sold_out(), ServiceOfferingError::SoldOut);

        self.num_sold += 1;
        Ok(())
    }
}

/*

Several pieces of data to store in the Asset (NFT) account (not in the ServiceOffering account):
- name
- description
- image
- T&Cs
- soulbound
- royalties

*/