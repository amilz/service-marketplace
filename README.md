
## 2-sided marketplace model for Services
_[Task](TASK.md)_

This is 90% complete, but I am AFK for the next few days so submitting with what I have so far.

## Local Development

I am using:
- Anchor v0.30.1
- Solana CLI v1.18.18
- Node v20.9.0

Clone the repo and run `yarn` to install dependencies.

Run `anchor test` to run the tests.

[Test Video](https://youtu.be/7irC1Z0Wv8o)

## State

The Program includes two state structs: `ServiceOffering` and `Listing`. 

### ServiceOffering

The [`ServiceOffering`](/programs/service-marketplace/src/state/service_offering.rs) struct represents a service offering, which includes details such as the vendor, the maximum number of services that can be sold, the price of each service, and whether the service offering is currently active.
Seeds:
- `service_offering` string literal
- `vendor` public key
- `offering_name` string literal

### Listing

The [`Listing`](/programs/service-marketplace/src/state/listing.rs) struct represents a listing for a service, which includes details such as the seller, the asset being sold, the price of the listing, and the expiration timestamp.
Seeds:
- `listing` string literal
- `asset` public key
- `seller` public key

## Instructions

The program includes the following instructions:

### Create Service Offering

This [instruction](/programs/service-marketplace/src/instructions/create_service_offering.rs) creates a new service offering PDA and Group Asset NFT. The offering PDA governs the "minting" of new service NFTs. The NFTs are minted using [Nifty Asset Standard](https://nifty-oss.org/) due to their low data size, no-fees, and high flexibility.

Input Parameters:
- `offering_name`: The name of the service offering.
- `max_quantity`: The maximum number of services that can be sold.
- `sol_price`: The price of each service in lamports.
- `expires_at`: The timestamp at which the service offering expires.
- `symbol`: The symbol of the service offering.
- `description`: A description of the service offering.
- `uri`: The URI of the service offering.
- `image`: The image URI of the service offering.
- `royalty_basis_points`: The basis points of royalty collection for resales.
- `terms_of_service_uri`: The URI of the terms of service.
- `is_transferrable`: Whether the service offering is transferable or not.

### Buy Service

This [instruction](/programs/service-marketplace/src/instructions/buy_service.rs) purchases a service offering. Effectively, the instruction "mints" a new asset to the group asset NFT, owned by the buyer.

Input Parameters:
- `offering_name`: The name of the service offering to purchase.

### List Asset

This [instruction](/programs/service-marketplace/src/instructions/list_asset.rs) creates a new listing for an asset. The listing is created with a price and an optional expiration timestamp.

Input Parameters:
- `price`: The price of the listing in lamports.
- `expires_at`: The timestamp at which the listing expires.

### Buy Listing

This [instruction](/programs/service-marketplace/src/instructions/buy_listing.rs) purchases a listing for an asset. The instruction transfers the payment to the seller, and transfers the royalties to the royalty receiver.

Input Parameters:
- n/a

## TODO
- Add additional tests (deserialization of Assets data, run fail checks, etc.)
- Deserialize group data to have royalty payments dynamically
- Refactor and clean up code
- Add delist instruction
