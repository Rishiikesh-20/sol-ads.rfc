# sol-ads.rfc
Submissions repository for `ASOC4` - Ads Exchange on Solana

> [!NOTE]
All discussions regarding `ASOC4: Ads Exchange on Solana` shall take place in https://github.com/orgs/acm-avv/discussions/12.

## Overview
In-order to be eligible to work on this project as **Request for Code** under the banner of **Amrita Summer of Code, 2025**, you are required to form a team of size 1-4 and have all the members register at [amsoc.vercel.app](https://amsoc.vercel.app)

## Project Manager Details
@adithya-menon-r
```json
"Name": "Adithya Menon R",
"Year": "3rd",
"Roll": "CB.EN.U4CSE23xxx",
"GitHub": "@adithya-menon-r",
```

## How to Apply
Type out a message in https://github.com/orgs/acm-avv/discussions/12 with the following details:
1. Team Name
2. Team Members' Names, Roll-Numbers and respective GitHub usernames
3. Tag the project manager as **@username**

## Guidelines
1. Keep all discussions limited to this discussion channel by tagging the project manager via **@username**
2. Do not try to contact the project manager personally unless they are open to it.
4. Maintain decorum and avoid any misbehavior with the project manager. This can be subjected to disqualification.
5. Send us an update every week with regards to your progress for your respective project. If we do not receive an update for more than 10 days then your team will be disqualified automatically.

---
## Project Description

The **Ads Exchange** project aims to create a decentralized platform that commoditizes ad-spaces, making them available as assets on a marketplace, similar to Google Ads but built on decentralized principles. The project will be split into four distinct repositories.

* **Overall Vision:**
    * To build a decentralized ad exchange platform.
    * Commoditize ad-spaces and enable their rental as assets on a marketplace.
    * Facilitate micro-transactions for ad placements based on billable hours.

* **Repository 1: UI Components for Ad Display**
    * **Purpose:** To provide reusable UI components specifically designed for displaying advertisements on various websites.
    * **Functionality:** These components will be integrated into different web environments to render booked ads.

* **Repository 2: Exchange Frontend**
    * **Purpose:** To serve as the user interface for the ad marketplace.
    * **Functionality:**
        * Lists all available ad spaces.
        * Provides mechanisms for users to rent out ad spaces for specified durations (e.g., X number of hours).

* **Repository 3: Solana Blockchain Interaction**
    * **Purpose:** To handle all blockchain-related operations for the ad exchange.
    * **Functionality:**
        * Facilitates micro-transactions based on billable ad hours.
        * Interacts with the Solana blockchain for secure and transparent payment processing.

* **Repository 4: Ad Delivery & Integration Logic**
    * **Purpose:** To manage the interaction between individual UI components deployed on various websites and the ad booking data.
    * **Functionality:**
        * Determines which ads need to be shown at different hours of the day based on active bookings.
        * Ensures dynamic delivery of advertisements to the respective UI components.
