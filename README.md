# V1 Auth Relinquishment Contract

[![Test](https://github.com/radixnameservice/rns-core-v2/actions/workflows/test.yml/badge.svg)](https://github.com/radixnameservice/rns-core-v2/actions/workflows/test.yml) [![Security audit](https://github.com/radixnameservice/rns-core-v2/actions/workflows/format.yml/badge.svg)](https://github.com/radixnameservice/rns-core-v2/actions/workflows/format.yml) [![Static Badge](https://img.shields.io/badge/Scrypto-v1.3.0-blue)](https://github.com/radixdlt/radixdlt-scrypto) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A minimal, standalone Scrypto contract for permanently locking RNS V1 admin and upgrade badges.

## Purpose

This contract demonstrates irreversible commitment to the RNS V2 upgrade by providing a permanent vault for V1 authorization badges. Once deposited, badges can never be withdrawn.

## Design Principles

- **Zero admin capability** — No owner role, no upgrade path, no special privileges
- **Single responsibility** — Only accepts and holds V1 badges forever
- **Validated deposits** — Only accepts the specific V1 badge resources specified at instantiation
- **No withdrawal** — Badges are permanently locked with no retrieval mechanism
- **Risk isolation** — Separated from the main RNS core contract to minimize risk

## Installation

```bash
cd rns_v1_badge_lockers
scrypto build
```

## Instantiation

```
CALL_FUNCTION
    Address("package_ADDRESS")
    "V1AuthRelinquishment"
    "instantiate"
    Address("resource_V1_ADMIN_BADGE_ADDRESS")
    Address("resource_V1_UPGRADE_BADGE_ADDRESS")
;
```

## Methods

### `lock_admin_badges`

Permanently locks V1 admin badges into the contract.

```
CALL_METHOD
    Address("account_ADDRESS")
    "withdraw"
    Address("resource_V1_ADMIN_BADGE_ADDRESS")
    Decimal("1")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_V1_ADMIN_BADGE_ADDRESS")
    Bucket("admin_badges")
;
CALL_METHOD
    Address("component_ADDRESS")
    "lock_admin_badges"
    Bucket("admin_badges")
;
```

### `lock_upgrade_badges`

Permanently locks V1 upgrade badges into the contract.

```
CALL_METHOD
    Address("account_ADDRESS")
    "withdraw"
    Address("resource_V1_UPGRADE_BADGE_ADDRESS")
    Decimal("1")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_V1_UPGRADE_BADGE_ADDRESS")
    Bucket("upgrade_badges")
;
CALL_METHOD
    Address("component_ADDRESS")
    "lock_upgrade_badges"
    Bucket("upgrade_badges")
;
```

### `get_lock_status`

Returns the current lock status showing how many V1 badges are locked.

```
CALL_METHOD
    Address("component_ADDRESS")
    "get_lock_status"
;
```

Returns:
```rust
V1LockStatus {
    admin_badges_locked: Decimal,
    upgrade_badges_locked: Decimal,
    admin_badge_resource: ResourceAddress,
    upgrade_badge_resource: ResourceAddress,
}
```

## Events

### `V1AdminBadgesLockedEvent`

Emitted when admin badges are locked:
- `badges_locked`: Number of badges locked in this transaction
- `total_locked_now`: Total admin badges now locked in the contract
- `timestamp`: When the lock occurred

### `V1UpgradeBadgeLockedEvent`

Emitted when upgrade badges are locked:
- `badges_locked`: Number of badges locked in this transaction
- `total_locked_now`: Total upgrade badges now locked in the contract
- `timestamp`: When the lock occurred

## Testing

```bash
scrypto test
```

---

## License

This project is licensed under the **MIT License** - see below for details.

```
MIT License

Copyright (c) 2026 Radix Name Service

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## Disclaimer

**USE AT YOUR OWN RISK.** This software is provided for informational and educational purposes only.

- This software interacts with blockchain networks and smart contracts. Blockchain transactions are **irreversible**. Once executed, transactions cannot be undone, cancelled, or refunded.
- This software has **whilst this has been independently audited by 3rd party developers, no official insurance backed auditors have verified the codebase**. While reasonable efforts have been made to ensure correctness, no guarantees are made regarding the security, reliability, or accuracy of this code.
- **Do not deploy to mainnet** without conducting your own thorough security review and testing.
- The authors and contributors make **no representations or warranties** regarding the functionality, security, or suitability of this software for any particular purpose.
- Users are solely responsible for understanding the risks associated with blockchain technology, smart contracts, and decentralized applications.

---

## Limitation of Liability

**TO THE MAXIMUM EXTENT PERMITTED BY APPLICABLE LAW:**

1. **NO LIABILITY FOR DAMAGES.** In no event shall the authors, copyright holders, contributors, or any affiliated parties be liable for any direct, indirect, incidental, special, exemplary, consequential, or punitive damages whatsoever (including, without limitation, damages for loss of profits, data, use, goodwill, or other intangible losses) arising out of or in connection with:
   - The use or inability to use this software
   - Any errors, bugs, or vulnerabilities in the code
   - Any unauthorized access to or alteration of your data or transactions
   - Any loss of cryptocurrency, tokens, digital assets, or funds
   - Any smart contract failures, exploits, or unexpected behavior
   - Any actions taken based on the content of this repository

2. **ASSUMPTION OF RISK.** By using this software, you expressly acknowledge and agree that:
   - You use this software entirely at your own risk
   - You have conducted your own due diligence and security review
   - You understand the inherent risks of blockchain technology and smart contracts
   - You are solely responsible for any and all consequences of your use of this software

3. **NO PROFESSIONAL ADVICE.** Nothing in this repository constitutes legal, financial, investment, or technical advice. Consult qualified professionals before making any decisions based on this software.

4. **INDEMNIFICATION.** You agree to indemnify, defend, and hold harmless the authors, copyright holders, and contributors from any claims, damages, losses, or expenses arising from your use of this software or violation of these terms.

5. **JURISDICTIONAL LIMITATIONS.** Some jurisdictions do not allow the exclusion or limitation of certain warranties or liabilities. In such jurisdictions, the above limitations shall apply to the maximum extent permitted by law.

**BY USING THIS SOFTWARE, YOU ACKNOWLEDGE THAT YOU HAVE READ, UNDERSTOOD, AND AGREE TO BE BOUND BY THESE TERMS.**