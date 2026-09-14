# Groups & Architecture Tiers

Complex systems are typically structured into distinct tiers, such as **Edge & Ingress**, **Application Services**, and **Persistence**.

ProcSim allows you to group related nodes together and style visual boundary containers around them.

---

## Defining Groups

Define `groups:` at the root of your YAML:

```yaml
groups:
  frontend:
    title: "Edge & Ingress Tier"
    direction: tb        # Top-to-bottom column inside this group
    style:
      bg: "#1e293b"      # Container background color
      border: "#38bdf8"  # Border stroke color
      border_width: 2.0
      radius: 14.0       # Rounded corners

  backend:
    title: "Application Core"
    direction: tb
    style:
      bg: "#0f172a"
      border: "#a855f7"
      border_width: 2.0
      radius: 14.0

  storage:
    title: "Data Persistence"
    direction: tb
    style:
      bg: "#111827"
      border: "#10b981"
      border_width: 2.0
      radius: 14.0
```

---

## Assigning Nodes to Groups

Assign each node to its respective tier using the `group:` property in `graph:`:

```yaml
graph:
  - name: client
    node_type: client
    group: frontend
    links: [gateway]

  - name: auth_service
    node_type: service
    group: backend
    links: [user_db]
```

---

## Compound Multi-Direction Layouts

The layout engine automatically computes:
1. **Macro Layout**: Arranges the groups from left to right (`lr`).
2. **Micro Layout**: Stacks the inner nodes within each group from top to bottom (`tb`).
3. **Bounding Boxes**: Renders styled container boxes behind the grouped nodes with header labels.
