---
version: '1.0'
name: 'Trak — Monolithic Tri-Chromatic Minimalism'
description: 'Radical brutalist-minimalist design system for Trak, a daily execution and recurring task provisioning web application. Strictly limited to 3 colors (#FFFFFF, #000000, #737373), 0px border radius, and total axial centering.'
colors:
  white: '#FFFFFF'
  black: '#000000'
  gray: '#737373'
typography:
  display:
    fontFamily: 'Space Grotesk'
    fontSize: '28px'
    fontWeight: 700
    lineHeight: '32px'
    letterSpacing: '-0.03em'
  body:
    fontFamily: 'Space Grotesk'
    fontSize: '15px'
    fontWeight: 500
    lineHeight: '22px'
  mono:
    fontFamily: 'IBM Plex Mono'
    fontSize: '11px'
    fontWeight: 500
    lineHeight: '16px'
    letterSpacing: '0.08em'
rounded:
  none: '0px'
spacing:
  spine: '440px'
  cell: '12px'
  section: '32px'
---

# Trak — Frontend Design Specification

## Pass 1 vs. Pass 2 Critique & Revision

1. **Color Discipline (Revised from 5-shade neutral scale to Strict 3-Color Law):**
   - *Pass 1 flaw:* Using soft off-white (`#F5F5F5`) surfaces and light gray (`#E5E5E5`) borders dilutes monochromatic tension.
   - *Pass 2 fix:* Literally only **three hex values** exist in CSS and Tailwind: Pure White (`#FFFFFF`), Pure Black (`#000000`), and Mid-Gray (`#737373`). Visual hierarchy is encoded solely via color inversion (`bg-black text-white` vs. `bg-white text-black`), border weight (`1px` vs. `2px`), and strike-through state.
2. **Spatial Alignment (Revised from Top-Nav + Left-Aligned Rows to Total Axial Centering):**
   - *Pass 1 flaw:* A top bar with left/right split navigation and left-aligned checkboxes reads like a generic SaaS template.
   - *Pass 2 fix:* Every screen sits inside a centered vertical & horizontal monolith (`min-h-screen flex flex-col items-center justify-center text-center`). The `440px` central spine centers headings, inputs, task titles, day matrices, and the quiet footer/header route index.
3. **Dashboard Purity (Revised to Today-Only Surface):**
   - *Pass 1 flaw:* Filter chips (`All / Active / Done`), search inputs, and completion progress bars clutter the daily focus.
   - *Pass 2 fix:* `/` renders **only today's todo list**—today's weekday/date, a centered quick-entry field, and the stacked tasks for today (provisioned from today's recurring templates plus today's manual entries).

## Token System

| Token | Hex | Role |
| :--- | :--- | :--- |
| `white` | `#FFFFFF` | Primary canvas, default surface, inverted label text on black fills |
| `black` | `#000000` | Primary ink, active borders, inverted active fills, focus frames |
| `gray` | `#737373` | Completed task ink, secondary metadata, inactive borders, hairlines |

- **Typography:**
  - **Display & Task Copy (`Space Grotesk`):** Sharp geometric forms for centered task statements and page titles.
  - **Structural Index & Controls (`IBM Plex Mono`):** Technical monospace for dates, 7-day recurrence cells, role toggles, and navigation.
- **Geometry:** `border-radius: 0px` (`rounded-none`) on every element. No shadows, no gradients, no curves.

## ASCII Wireframes & Route Architecture

### 1. `/` — Today Dashboard (Centered Monolith)
```text
              [ TODAY ]   TEMPLATES   USERS   LOGOUT
           --------------------------------------------
                            WEDNESDAY
                      2026.11.11 // 1 OF 3

           +------------------------------------------+
           |       Write task for today + Enter       |
           +------------------------------------------+
           |  [ ]   Review daily server backups    ×  |
           +------------------------------------------+
           |  [ ]   Publish sprint release notes   ×  |
           +------------------------------------------+
           |  [■]   Morning standup sync           ×  |  <-- #737373 strike
           +------------------------------------------+
```

### 2. `/login` & `/register` — Audiobookshelf-Style First-Boot Auth
```text
                              TRAK
                 INITIAL ADMIN PROVISIONING (0 USERS)
           +------------------------------------------+
           |             USERNAME INPUT               |
           +------------------------------------------+
           |             PASSWORD INPUT               |
           +------------------------------------------+
           |        [ CLAIM ADMIN & INITIALIZE ]      |  <-- #000000 fill
           +------------------------------------------+
```
*(Once user #1 registers as Admin, `/register` locks and `/login` becomes the sole entry gate.)*

### 3. `/templates` — Day-Specific Recurring Task Provisioning
```text
           +-----+-----+-----+-----+-----+-----+-----+
           | MON | TUE |[WED]| THU | FRI | SAT | SUN |
           +-----+-----+-----+-----+-----+-----+-----+
                    RECURRING EVERY WEDNESDAY
           +------------------------------------------+
           |     Add Wednesday recurring task...      |
           +------------------------------------------+
           |       Review daily server backups     ×  |
           +------------------------------------------+
```

### 4. `/users` — Admin-Only User & Privilege Ledger
```text
                         USER DIRECTORY
           +------------------------------------------+
           |         NEW USERNAME  |  PASSWORD        |
           |          [■] GRANT ADMIN CLEARANCE       |
           |              [ PROVISION USER ]          |
           +------------------------------------------+
           |  kryisnn (ROOT)      [ ADMIN ]           |
           +------------------------------------------+
           |  operator_02         [ USER -> ADMIN ] × |
           +------------------------------------------+
```
