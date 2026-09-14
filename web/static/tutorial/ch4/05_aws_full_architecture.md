# Complete Cloud Architectures (`plibs:aws/all`)

By combining the composite library **`plibs:aws/all`** with your choice of visual theme, you can model realistic enterprise architectures in dozens of lines of clear, declarative YAML.

---

## 1. Single Import for All Services

Instead of importing individual modules, import the master package:

```yaml
imports:
  - from: "plibs:themes/cloud"
  - from: "plibs:aws/all"
```

This makes all 12 AWS components immediately available:
- **Compute**: `lambda_func`, `ec2_instance`, `ecs_service`
- **Networking**: `api_gateway`, `alb`, `cloudfront`
- **Database**: `dynamodb`, `s3_bucket`, `rds_aurora`
- **Messaging**: `sqs_queue`, `sns_topic`, `eventbridge`

---

## 2. Serverless E-Commerce Workflow

In this architecture:
1. Users send requests through **CloudFront CDN** and **API Gateway**.
2. **Lambda Order Function** validates the order and writes immediate state to **DynamoDB**.
3. Orders are published to an **SQS Queue** for asynchronous processing.
4. Background Lambda workers ingest the queue and persist invoices to **S3** and write financial ledgers to **Aurora RDS**.

---

## 3. Full Architecture in Action

Click **▶ Load this example** below to run the complete cloud architecture simulation!
