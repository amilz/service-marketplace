use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    // The public key of the seller (owner of the NFT)
    pub seller: Pubkey,

    // The public key of the asset being sold
    pub asset_id: Pubkey,

    // The price of the NFT in lamports
    pub price: u64,

    // Timestamp when the listing was created
    pub created_at: i64,

    // Optional expiration timestamp for time-limited listings
    pub expires_at: Option<i64>,

    // The bump used in PDA derivation
    pub bump: u8,
}

impl Listing {
    pub fn get_size() -> usize {
        8 +     // discriminator
        32 +    // seller
        32 +    // asset_id
        8 +     // price
        8 +     // created_at
        9 +     // expires_at (1 byte for Option enum + 8 bytes for i64)
        1       // bump
    }

    pub fn create(
        &mut self,
        seller: Pubkey,
        asset_id: Pubkey,
        price: u64,
        expires_at: Option<i64>,
        bump: u8,
    ) {
        self.seller = seller;
        self.asset_id = asset_id;
        self.price = price;
        self.created_at = Clock::get().unwrap().unix_timestamp;
        self.expires_at = expires_at;
        self.bump = bump;
    }

    pub fn update_price(&mut self, new_price: u64) {
        self.price = new_price;
    }

    fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expires_at {
            Clock::get().unwrap().unix_timestamp >= expiry
        } else {
            false
        }
    }

    pub fn is_active(&self) -> bool {
        !self.is_expired()
    }
}
