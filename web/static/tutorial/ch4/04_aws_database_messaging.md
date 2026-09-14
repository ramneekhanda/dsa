# AWS Storage, Databases & Messaging

The `plibs:aws/database` and `plibs:aws/messaging` packages provide decoupled persistence and asynchronous message passing for cloud simulations.

---

## 1. Storage & Database Components (`plibs:aws/database`)

- **`dynamodb`**: Fast NoSQL Key-Value / Document table with partition key routing, item counts, and query metrics.
- **`s3_bucket`**: Scalable object storage bucket supporting `PUT` / `GET` operations and object count tracking.
- **`rds_aurora`**: High-availability relational database cluster with connection pool tracking and read-replica distribution.

---

## 2. Messaging & Event Components (`plibs:aws/messaging`)

- **`sqs_queue`**: Decoupled message queue with live queue depth visualization and consumer polling.
- **`sns_topic`**: High-throughput Pub/Sub notification topic with multi-subscriber fan-out.
- **`eventbridge`**: Serverless event bus with pattern matching and event routing across multiple cloud targets.

---

## 3. Asynchronous Pipeline Example

Click **▶ Load this example** below to see an event-driven architecture buffering incoming orders into an **SQS queue** and writing records to **DynamoDB** and **S3**!
