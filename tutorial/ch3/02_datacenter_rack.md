# 3.2 Datacenter Rack Units

Beyond high-level software diagrams, `procsim`'s vector shapes allow you to model **physical hardware**, datacenter topologies, and industrial infrastructure with high precision.

In this lesson, we build a **19-inch Server Rack Chassis** template:
- Dark metallic chassis faceplate with rack ears and mounting screw points
- Multi-channel status indicator LEDs (Power, Network Link, Storage Activity)
- Hardware spec label (`1U • ETH0/48`, `2U • NVMe RAID`, etc.)
- Embedded hardware icon

```yaml
node_templates:
  - id: rack_chassis
    params: [node_name, icon, unit_id, led_pwr, led_net, led_act]
    shapes:
      # Main chassis faceplate
      - shape: roundedrect
        x: 0
        y: 0
        w: 136
        h: 44
        radius: 4
        fill: "#1e293b"
        stroke: "#475569"
        stroke_width: 1.5

      # Left & right rack-mount flanges (ears)
      - shape: rect
        x: -64
        y: 0
        w: 8
        h: 44
        fill: "#0f172a"
        stroke: "#334155"
        stroke_width: 1
      - shape: rect
        x: 64
        y: 0
        w: 8
        h: 44
        fill: "#0f172a"
        stroke: "#334155"
        stroke_width: 1

      # Mounting screw holes
      - shape: circle
        x: -64
        y: 13
        r: 2.2
        fill: "#94a3b8"
      - shape: circle
        x: -64
        y: -13
        r: 2.2
        fill: "#94a3b8"
      - shape: circle
        x: 64
        y: 13
        r: 2.2
        fill: "#94a3b8"
      - shape: circle
        x: 64
        y: -13
        r: 2.2
        fill: "#94a3b8"

      # Device Icon
      - shape: icon
        x: -40
        y: 0
        w: 22
        h: 22
        icon: "{{icon}}"

      # Unit text and hardware designation
      - shape: text
        x: 3
        y: 6
        text: "{{node_name}}"
        size: 9.5
        color: "#f8fafc"
      - shape: text
        x: 3
        y: -8
        text: "{{unit_id}}"
        size: 7.5
        color: "#94a3b8"

      # Hardware Status LEDs (Power, Net, Storage/Activity)
      - shape: circle
        x: 48
        y: 11
        r: 2.5
        fill: "{{led_pwr}}"
      - shape: circle
        x: 48
        y: 0
        r: 2.5
        fill: "{{led_net}}"
      - shape: circle
        x: 48
        y: -11
        r: 2.5
        fill: "{{led_act}}"
```

### Parameterizing LED indicators

Notice the three LED dots on the right edge of each chassis:
- `led_pwr`: Green (`#22c55e`) for active power
- `led_net`: Cyan / Blue (`#38bdf8`) for network link
- `led_act`: Amber (`#facc15`) or Red (`#ef4444`) for busy I/O or alert conditions

With template parameters, you can customize the operational state of every server unit individually without creating separate shapes for each one!

**Try it**: Load the example below to see a datacenter rack with an edge switch routing packets between compute hypervisors and a high-speed SAN storage array.
