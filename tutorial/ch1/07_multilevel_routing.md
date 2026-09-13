# 7. Multilevel Routing & Reverse Paths

In real-world distributed architectures, clients don't communicate directly with internal database or authentication nodes. Instead, traffic passes through an **API Gateway** or **Router** that dispatches requests to downstream microservices and routes responses back.

```
[ Client ]  <--->  [ API Gateway ]  <--->  [ Auth Service ]
                                    <--->  [ Data Service ]
```

---

## 1. Downstream Request Dispatching

When a client sends a request to the gateway, it specifies the destination `path` and a `reply_to` return address:

```rhai
send("gateway", #{
  type: "REQ",
  path: "/auth",
  req_id: globals.req_id,
  reply_to: "client",
  display: "/auth #" + globals.req_id
});
```

The gateway inspects `msg.path` and forwards the payload to the corresponding backend service:

```rhai
fn on_msg(msg) {
  if msg.type == "REQ" {
    if msg.path == "/auth" {
      send("auth_svc", msg);
    } else {
      send("data_svc", msg);
    }
  }
}
```

---

## 2. Reverse Response Routing with `msg.from` and `msg.reply_to`

Backend services don't need direct outgoing links to the original client. They simply reply to whoever forwarded the request (`msg.from`):

```rhai
// Inside auth_svc / data_svc:
fn on_msg(msg) {
  send(msg.from, #{
    type: "RESP",
    req_id: msg.req_id,
    reply_to: msg.reply_to,
    service: "auth",
    status: 200,
    display: "AUTH OK #" + msg.req_id
  });
}
```

When the gateway receives the response (`msg.type == "RESP"`), it forwards it back along the existing connection to `msg.reply_to`:

```rhai
// Inside gateway:
if msg.type == "RESP" {
  send(msg.reply_to, msg);
}
```

---

## Key Takeaways

1. **`links` defines downstream visibility**: A node's `links` array lists its downstream targets.
2. **Connectors are bidirectional**: Messages can travel in either direction along an established connector edge.
3. **`msg.from` identifies the immediate sender**: Useful for single-hop replies.
4. **`msg.reply_to` tracks the origin client**: Allows multi-hop gateways and proxies to route responses back across complex topologies.
