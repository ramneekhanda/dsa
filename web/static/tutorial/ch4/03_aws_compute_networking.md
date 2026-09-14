# AWS Compute & Networking

The `plibs:aws/compute` and `plibs:aws/networking` libraries deliver ready-to-use cloud infrastructure primitives with AWS branding, live metrics tracking, and realistic routing behaviors.

---

## 1. Available Components

### Compute (`plibs:aws/compute`)
- **`lambda_func`**: Serverless compute function with invocation metrics (`invocations`), concurrency, and automatic request forwarding.
- **`ec2_instance`**: Virtual machine blade tracking CPU utilization, memory usage, and instance health.
- **`ecs_service`**: Container orchestration task with container instance scaling and health checks.

### Networking (`plibs:aws/networking`)
- **`api_gateway`**: HTTP/REST gateway that accepts incoming requests and dispatches them across connected downstream services.
- **`alb`**: Application Load Balancer with round-robin traffic distribution.
- **`cloudfront`**: Global CDN Edge distribution with edge caching and origin routing.

---

## 2. Using Compute & Networking

```yaml
imports:
  - from: "plibs:aws/compute"
  - from: "plibs:aws/networking"

graph_defn:
  graph:
    - name: edge_cdn
      node_type: cloudfront
      links: [api_gw]
    - name: api_gw
      node_type: api_gateway
      links: [lambda_order, ecs_auth]
    - name: lambda_order
      node_type: lambda_func
      links: []
    - name: ecs_auth
      node_type: ecs_service
      links: []
```

---

## 3. Running Example

Click **▶ Load this example** below to run an edge-routed AWS serverless workflow!
