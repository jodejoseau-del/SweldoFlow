# SweldoFlow
> Real-time streaming earned wage access on Stellar for enterprise employees in the Philippines.

## Problem & Solution
* **Problem:** BPO employees in Metro Manila incur 15–20% interest rates from payday lenders to cover daily expenses before monthly paychecks.
* **Solution:** SweldoFlow allows employers to escrow salary USDC into a Soroban contract, enabling workers to continuously stream earned wages directly to their Stellar wallet for negligible fees.

## Timeline
Hackathon MVP build executed over a 48-hour sprint.

## Stellar Features Used
* **USDC asset transfers**
* **Soroban smart contracts**
* **Trustlines**
* **Anchor API (off-ramping)**

## Vision & Purpose
Empower the 1.5 million Philippine BPO workforce with instant liquidity, breaking dependence on predatory micro-lenders through open financial infrastructure.

## Prerequisites
* Rust v1.75+
* Soroban CLI v21.0.0+

## How to Build
```bash
soroban contract build

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CDKWO7FE3X7KAVZRGUPF52A4RYVQ6VK2SVKCO4EZ5G25YIYSOBZAGQWT` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CDKWO7FE3X7KAVZRGUPF52A4RYVQ6VK2SVKCO4EZ5G25YIYSOBZAGQWT) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/b42dc00af88f159992da8501640a3242399d64a56652cbed41791776239aa78b) |
| Deployed | 2026-08-24 08:11:58 UTC |
| Wallet | freighter (`GB6S…VJMT`) |
