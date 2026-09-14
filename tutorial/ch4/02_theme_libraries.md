# Theme Libraries (`plibs:themes/*`)

The `plibs/themes/` suite provides complete visual and interactive styling packages designed for different aesthetic and domain requirements.

---

## 1. Built-in Theme Roster

Each theme delivers a fully cohesive palette across the canvas, connection links, message envelopes, and interactive narration modals:

| Theme Preset | Visual Style | Accent & Highlights | Node Cards | Message Bubble Shape |
|:-------------|:-------------|:-------------------|:-----------|:---------------------|
| **`plibs:themes/cloud`** | Modern SaaS Light/Steel | Azure `#3B82F6` & Sky `#0284C7` | Soft Rounded Cards | Pill (`pill`) |
| **`plibs:themes/synthwave`** | Retro 80s Cyber Neon | Magenta `#FF2A85` & Cyan `#00F0FF` | Neon Glowing Bevels | Chamfered (`chamfered`) |
| **`plibs:themes/nordic`** | Polar Arctic Dark | Frost `#88C0D0` & Aurora `#A3BE8C` | Arctic Slate Panels | Rounded Box (`rounded`) |
| **`plibs:themes/dracula`** | Classic Dark Vampire | Purple `#BD93F9` & Pink `#FF79C6` | Gothic Dark Cards | Chamfered (`chamfered`) |
| **`plibs:themes/matrix`** | Phosphor Terminal | Emerald `#00FF66` & `#003B00` | Monospace Brackets | Terminal Box (`box`) |
| **`plibs:themes/solarized_light`** | Warm High-Readability | Solar Blue `#268BD2` & Cyan `#2AA198` | Clean Parchment Cards | Rounded (`rounded`) |
| **`plibs:themes/cyberpunk`** | High-Tech Blueprint | Electric Yellow `#FFE600` & Cyan | HUD Wireframes | Chamfered (`chamfered`) |
| **`plibs:themes/datacenter`** | Infrastructure Server | Amber `#F59E0B` & Blue `#3B82F6` | Rack Chassis Blades | Box (`box`) |
| **`plibs:themes/minimal`** | Monochromatic Wireframe | Slate Monochrome `#475569` | Capsule Pills | Pill (`pill`) |

---

## 2. Using Theme Packages

Switching themes takes a single line:

```yaml
imports:
  - from: "plibs:themes/dracula"
```

You can also import multiple libraries together:

```yaml
imports:
  - from: "plibs:themes/synthwave"
  - from: "stdlib:load_balancer"
  - from: "plibs:aws/messaging"
```

---

## 3. Interactive Theme Inspection

Click **▶ Load this example** below to see the **Dracula** theme in action with interactive step narration!
