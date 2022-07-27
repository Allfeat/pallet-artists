# Artists Pallet

## Overview

The Artists Pallet allows an user (that means an AccountId) to submit a candidacy to become an Artist on the AllFeat Blockchain. 

## Interface

### Dispatchable functions

### Getters

- `get_candidate` - Returns a candidate for a given AccountId.
- `get_artist` - Returns an artist for a given AccountId.

#### For general users

- `submit_candidacy` - Submit a candidacy to become an artist.
- `withdraw_candidacy` - Remove a candidacy (Can be called only by the candidacy owner).

### For admin users (root or staff)

- `approve_candidacy` - Upgrade an account from Candidate (removed) to Artist.

License: Unlicense