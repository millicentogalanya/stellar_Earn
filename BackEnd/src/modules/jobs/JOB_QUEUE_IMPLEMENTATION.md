# Job Queue Processing System

Comprehensive background job processing system for the Stellar Earn platform. Handles asynchronous operations including payouts, emails, data exports, webhooks, analytics, and quest monitoring.

## Table of Contents

1. [Architecture](#architecture)
2. [Features](#features)
3. [Job Types](#job-types)
4. [API Reference](#api-reference)
5. [Configuration](#configuration)
6. [Usage Examples](#usage-examples)
7. [Monitoring & Observability](#monitoring--observability)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)

## Architecture

### Core Components

- **BullMQ** — Queue management with Redis
- **Job Processors** — Specialized handlers for each job type
- **Job Log Service** — Persistent audit trail and logging
- **Job Scheduler Service** — Cron-based scheduling with timezone support
- **Job Controller** — REST API for job management

### Queue Types

```
payouts          → High priority payout processing
email            → Email delivery and digests
exports          → Data export and report generation
cleanup          → Maintenance and cleanup tasks
maintenance      → Database optimization
webhooks         → Webhook delivery with retry logic
analytics        → Aggregation and metrics collection
quests           → Quest monitoring and verification
```

### Processing Flow

```
Create Job
    ↓
JobLog Entry Created
    ↓
Job Added to Queue
    ↓
Worker Picks Job
    ↓
Job Processing
    ├→ Progress Updates
    ├→ Retry on Failure
    └→ Complete or Fail
    ↓
Log Results
    ↓
Optionally Retry
```

## Features

### ✅ Implemented

- [x] Priority queues (CRITICAL, HIGH, MEDIUM, LOW)
- [x] Exponential backoff retry logic (5 attempts, configurable)
- [x] Progress tracking with real-time updates
- [x] Dead letter queue for failed jobs
- [x] Job dependencies and sequential execution
- [x] Cron-based scheduling with timezone support
- [x] Job cancellation and rescheduling
- [x] Bulk job creation
- [x] Comprehensive job logging and audit trail
- [x] Correlation IDs for tracing related jobs
- [x] Performance metrics and dashboard
- [x] Queue statistics and monitoring

### 🚀 Processing Capabilities

#### Payout Processing
- Stellar transaction creation and submission
- Multi-sig support with approval workflows
- Transaction verification and settlement
- Automatic retry on network failures

#### Email Operations
- Template-based email sending
- Bulk digest generation
- Support for SendGrid, AWS SES, others
- Delivery tracking and logging

#### Data Export
- Multi-format support (CSV, JSON, XLSX)
- Async export with download links
- Email notification on completion
- Large dataset handling

#### Cleanup & Maintenance
- Session expiration cleanup
- Log rotation and archival
- Database optimization (VACUUM, ANALYZE, REINDEX)
- Configurable retention policies

#### Webhook Delivery
- Event-driven webhook delivery
- HMAC signature generation
- Automatic retry with exponential backoff
- Failed delivery tracking

#### Analytics & Metrics
- Time-based aggregation (hourly, daily, weekly, monthly)
- Custom metrics collection
- Performance analysis
- Percentile calculations (p95, p99)

#### Quest Monitoring
- Deadline enforcement
- Submission validation
- Automatic reward distribution
- Completion verification

## Job Types

### Enum: JobType

```typescript
// Payout Processing
PAYOUT_PROCESS = 'payout:process'
PAYOUT_SETTLE = 'payout:settle'

// Email
EMAIL_SEND = 'email:send'
EMAIL_DIGEST = 'email:digest'

// Data Export
DATA_EXPORT = 'data:export'
REPORT_GENERATE = 'report:generate'

// Cleanup
CLEANUP_EXPIRED_SESSIONS = 'cleanup:expired-sessions'
CLEANUP_OLD_LOGS = 'cleanup:old-logs'
DATABASE_MAINTENANCE = 'maintenance:database'

// Webhooks
WEBHOOK_DELIVER = 'webhook:deliver'
WEBHOOK_RETRY = 'webhook:retry'

// Analytics
ANALYTICS_AGGREGATE = 'analytics:aggregate'
METRICS_COLLECT = 'metrics:collect'

// Quests
QUEST_DEADLINE_CHECK = 'quest:deadline-check'
QUEST_COMPLETION_VERIFY = 'quest:completion-verify'
```

### Enum: JobPriority

```typescript
CRITICAL = 0   // Urgent operations
HIGH = 1       // Important operations
MEDIUM = 5     // Normal operations (default)
LOW = 10       // Background operations
```

### Enum: JobStatus

```typescript
PENDING = 'pending'         // Waiting to process
PROCESSING = 'processing'   // Currently processing
COMPLETED = 'completed'     // Successfully completed
FAILED = 'failed'           // Maximum retries exceeded
CANCELLED = 'cancelled'     // User cancelled
DEFERRED = 'deferred'       // Scheduled for later
RETRY = 'retry'             // Scheduled for retry
```

## API Reference

### Create Job

```http
POST /api/v1/jobs
Content-Type: application/json

{
  "jobType": "email:send",
  "payload": {
    "messageId": "msg-123",
    "recipientEmail": "user@example.com",
    "templateId": "template-456",
    "variables": { "name": "John" }
  },
  "priority": 5,
  "maxAttempts": 5,
  "correlationId": "corr-123",
  "organizationId": "org-456",
  "userId": "user-789",
  "tags": ["marketing", "digest"]
}
```

### Query Jobs

```http
GET /api/v1/jobs?status=completed&organizationId=org-456&limit=50&offset=0&sortBy=createdAt&sortOrder=DESC

Response:
{
  "data": [
    {
      "id": "job-123",
      "jobType": "email:send",
      "status": "completed",
      "queueName": "email",
      "attempt": 1,
      "maxAttempts": 5,
      "progress": 100,
      "durationMs": 2500,
      "createdAt": "2026-03-27T10:00:00Z",
      "updatedAt": "2026-03-27T10:00:02Z",
      "completedAt": "2026-03-27T10:00:02Z",
      "correlationId": "corr-123",
      "organizationId": "org-456",
      "userId": "user-789"
    }
  ],
  "total": 156,
  "limit": 50,
  "offset": 0
}
```

### Get Job Details

```http
GET /api/v1/jobs/job-123

Response:
{
  "id": "job-123",
  "jobType": "email:send",
  "status": "completed",
  ...
}
```

### Retry Failed Job

```http
POST /api/v1/jobs/job-123/retry
Content-Type: application/json

{
  "delayMs": 5000,
  "updatedPayload": { ... } // optional
}
```

### Cancel Job

```http
DELETE /api/v1/jobs/job-123
Content-Type: application/json

{
  "reason": "User requested cancellation"
}
```

### Reschedule Job

```http
PATCH /api/v1/jobs/job-123/reschedule
Content-Type: application/json

{
  "delayMs": 60000,
  "updatedPayload": { ... } // optional
}
```

### Monitoring Dashboard

```http
GET /api/v1/jobs/monitoring/dashboard

Response:
{
  "totalJobs": 5230,
  "pendingJobs": 45,
  "processingJobs": 12,
  "completedJobs": 5100,
  "failedJobs": 73,
  "cancelledJobs": 0,
  "averageDurationMs": 2345,
  "successRate": 98.6,
  "failureRate": 1.4,
  "avgRetriesPerJob": 1.2,
  "deadLetterQueueSize": 23,
  "jobsByType": {
    "email:send": 2500,
    "payout:process": 1200,
    "analytics:aggregate": 800,
    ...
  },
  "jobsByStatus": {
    "pending": 45,
    "processing": 12,
    "completed": 5100,
    "failed": 73,
    "cancelled": 0
  },
  "recentFailures": [ ... ],
  "topFailedJobs": [
    { "jobType": "payout:process", "failureCount": 35 },
    { "jobType": "email:send", "failureCount": 23 },
    ...
  ],
  "queueStatus": { ... }
}
```

### Create Scheduled Job

```http
POST /api/v1/jobs/schedules
Content-Type: application/json

{
  "jobType": "cleanup:old-logs",
  "cronExpression": "0 2 * * *", // Daily at 2 AM
  "timezone": "America/New_York",
  "jobPayload": {
    "olderThanDays": 90,
    "logTypes": ["error", "warn"]
  },
  "organizationId": "org-456",
  "description": "Daily log cleanup"
}
```

### List Scheduled Jobs

```http
GET /api/v1/jobs/schedules/list

Response:
{
  "data": [
    {
      "id": "schedule-123",
      "jobType": "cleanup:old-logs",
      "cronExpression": "0 2 * * *",
      "timezone": "America/New_York",
      "isActive": true,
      "lastRunAt": "2026-03-27T06:00:00Z",
      "nextRunAt": "2026-03-28T06:00:00Z",
      "successCount": 87,
      "failureCount": 2,
      "description": "Daily log cleanup"
    }
  ]
}
```

### Trigger Schedule Now

```http
POST /api/v1/jobs/schedules/schedule-123/trigger

Response:
{
  "jobId": "job-123",
  "message": "Schedule triggered successfully, job ID: job-123"
}
```

## Configuration

### Environment Variables

```bash
# Redis
REDIS_URL=redis://localhost:6379

# Job Queue Settings
JOB_QUEUE_CONCURRENCY=10           # Concurrent jobs per queue
JOB_MAX_ATTEMPTS=5                 # Default retry attempts
JOB_BACKOFF_DELAY=5000             # Initial backoff delay (ms)
JOB_LOG_RETENTION_DAYS=90          # How long to keep logs
JOB_TIMEOUT_DEFAULT=60000          # Default job timeout

# Email Integration
EMAIL_PROVIDER=sendgrid            # sendgrid, ses, or custom
SENDGRID_API_KEY=xxx
AWS_SES_REGION=us-east-1

# Storage Integration
STORAGE_PROVIDER=s3                # s3, gcs, or local
AWS_S3_BUCKET=stellar-earn-exports
GCS_BUCKET=stellar-earn-exports
```

### Queue Concurrency Configuration

```typescript
// src/modules/jobs/jobs.constants.ts
export const JOB_QUEUE_CONFIG = {
  payouts: {
    concurrency: 10,      // Process 10 payouts in parallel
    priority: 'HIGH',
    timeout: 60000,
  },
  email: {
    concurrency: 20,      // Process 20 emails in parallel
    priority: 'MEDIUM',
    timeout: 30000,
  },
  exports: {
    concurrency: 5,       // Process 5 exports in parallel
    priority: 'MEDIUM',
    timeout: 300000,      // 5 minute timeout
  },
  // ... more queues
};
```

## Usage Examples

### Example 1: Send Email

```typescript
import { JobType, JobPriority } from '../job.types';

// Create email job
const emailJob = await createJob({
  jobType: JobType.EMAIL_SEND,
  payload: {
    messageId: `msg-${Date.now()}`,
    recipientEmail: 'user@example.com',
    templateId: 'welcome-template',
    variables: {
      firstName: 'John',
      activationLink: 'https://...',
    },
  },
  priority: JobPriority.HIGH,
  organizationId: 'org-123',
  userId: 'user-456',
  correlationId: 'signup-flow-123',
});
```

### Example 2: Process Payout

```typescript
import { JobType, PayoutProcessPayload } from '../job.types';

const payoutJob = await createJob({
  jobType: JobType.PAYOUT_PROCESS,
  payload: {
    payoutId: 'payout-789',
    organizationId: 'org-123',
    amount: 100.50,
    recipientAddress: 'GDZST3XVCDTUJ76ZAV2HA72KYXM4ZCT5JBHNYX7UHZASDEFDZDCXACHL',
  } as PayoutProcessPayload,
  priority: JobPriority.CRITICAL,
  maxAttempts: 3, // Payout retries limited to 3
  organizationId: 'org-123',
  correlationId: 'batch-payout-2026-03-27',
});
```

### Example 3: Schedule Recurring Task

```typescript
import { JobType } from '../job.types';

// Run daily at 2 AM
const schedule = await jobSchedulerService.createSchedule(
  JobType.CLEANUP_OLD_LOGS,
  '0 2 * * *',  // Cron expression
  {
    olderThanDays: 90,
    logTypes: ['error', 'warn'],
  },
  {
    timezone: 'America/New_York',
    organizationId: 'org-123',
    description: 'Daily log cleanup (90+ days old)',
  },
);
```

### Example 4: Bulk Export

```typescript
import { JobType, BusinessEventPayload } from '../job.types';

const exportJob = await createJob({
  jobType: JobType.DATA_EXPORT,
  payload: {
    organizationId: 'org-123',
    exportType: 'payouts',
    format: 'csv',
    userId: 'user-456',
  },
  priority: JobPriority.MEDIUM,
  organizationId: 'org-123',
  userId: 'user-456',
  correlationId: 'export-request-2026-03-27',
});

// Monitor progress
setInterval(async () => {
  const jobStatus = await getJob(exportJob.id);
  console.log(`Export progress: ${jobStatus.progress}%`);
}, 1000);
```

### Example 5: Webhook Delivery

```typescript
import { JobType } from '../job.types';

const webhookJob = await createJob({
  jobType: JobType.WEBHOOK_DELIVER,
  payload: {
    webhookId: 'webhook-123',
    event: 'payout.completed',
    payload: {
      payoutId: 'payout-789',
      status: 'success',
      amount: 100.50,
      transaction: {
        id: 'tx_123',
        hash: '0x...',
      },
    },
    url: 'https://client.example.com/webhooks/payout',
    secret: 'webhook_secret_key',
  },
  priority: JobPriority.HIGH,
  maxAttempts: 5,
});
```

## Monitoring & Observability

### Dashboard Metrics

```typescript
// Get comprehensive dashboard
GET /api/v1/jobs/monitoring/dashboard

Returns:
- Total job counts by status
- Success/failure rates
- Average processing time
- Top failing job types
- Dead letter queue size
- Recent failures with details
```

### Queue Statistics

```typescript
// Get queue-specific stats
GET /api/v1/jobs/stats/queues

Returns:
- Active jobs count
- Waiting jobs count
- Completed jobs count
- Failed jobs count
- Average processing time per queue
- Queue pause status
```

### Job Correlation Tracking

```typescript
// Track related jobs by correlation ID
GET /api/v1/jobs/related/corr-123

Returns all jobs with:
- Same correlation ID
- Created in order
- Shows job dependencies
- Execution flow
```

### Job Performance Analysis

```typescript
// Query slow jobs (>60 seconds)
const slowJobs = await jobLogService.getSlowJobs(60000, 10);

// Performance metrics
const metrics = await jobLogService.getPerformanceMetrics();
// Returns: avgDurationMs, minDurationMs, maxDurationMs, p95DurationMs
```

## Best Practices

### 1. Job Design

```typescript
// ✅ DO: Create focused, single-responsibility jobs
const job = await createJob({
  jobType: JobType.EMAIL_SEND,
  payload: { email, templateId },
  // Simple, contained task
});

// ❌ DON'T: Create jobs that do multiple things
const job = await createJob({
  jobType: 'complex:task',
  payload: {
    // Attempting to: send email, update database, notify webhooks
    // Instead: create 3 focused jobs
  },
});
```

### 2. Priority Assignment

```typescript
// Use appropriate priorities
CRITICAL  → Immediate issues, customer-blocking
HIGH      → Important business operations (payouts)
MEDIUM    → Regular operations (emails, webhooks)
LOW       → Background maintenance, analytics

// Payout: use HIGH to ensure timely execution
{
  jobType: JobType.PAYOUT_PROCESS,
  priority: JobPriority.HIGH,
}

// Log cleanup: use LOW, runs during off-peak
{
  jobType: JobType.CLEANUP_OLD_LOGS,
  priority: JobPriority.LOW,
}
```

### 3. Error Handling

```typescript
// Provide max attempts for idempotent operations
// Low attempts for non-retryable failures
{
  jobType: JobType.PAYOUT_PROCESS,
  maxAttempts: 3,  // Limited retries for payouts
}

// Higher attempts for transient failures
{
  jobType: JobType.WEBHOOK_DELIVER,
  maxAttempts: 10,  // Webhooks may have temporary downtime
}
```

### 4. Correlation Tracking

```typescript
// Use correlationId to track related operations
const signupFlowId = `signup-${Date.now()}`;

// Email job
await createJob({
  jobType: JobType.EMAIL_SEND,
  correlationId: signupFlowId,
  // ...
});

// Webhook job
await createJob({
  jobType: JobType.WEBHOOK_DELIVER,
  correlationId: signupFlowId,
  // ...
});

// Later, query all related jobs
const relatedJobs = await getRelatedJobs(signupFlowId);
```

### 5. Job Dependencies

```typescript
// Create dependent jobs (sequential execution)
const parentJob = await createJob({
  jobType: JobType.DATA_EXPORT,
  // ...
});

// Child job waits for parent
await createJobDependency(parentJob.id, childJob.id, {
  blockOnFailure: true,
});
```

### 6. Large Dataset Handling

```typescript
// For bulk operations, create multiple jobs instead of one large job
// Instead of:
await createJob({
  jobType: JobType.DATA_EXPORT,
  payload: {
    allRecordsFromYear2025,  // ❌ Too large
  },
});

// Do:
const batches = chunkData(allRecords, 1000);
for (const batch of batches) {
  await createJob({
    jobType: JobType.DATA_EXPORT,
    payload: { records: batch },
    correlationId: 'export-2025-batch-001',
  });
}
```

## Troubleshooting

### Issue: Jobs not processing

```bash
# 1. Check Redis connection
redis-cli ping  # Should return PONG

# 2. Check job status
GET /api/v1/jobs/monitoring/dashboard

# 3. Check queue status
GET /api/v1/jobs/stats/queues

# 4. Review job logs
SELECT * FROM job_logs WHERE status = 'failed' ORDER BY created_at DESC
```

### Issue: High failure rate

```bash
# 1. Check recent failures
GET /api/v1/jobs/monitoring/dashboard  # top_failed_jobs

# 2. Get failure details
SELECT * FROM job_logs WHERE status = 'failed' LIMIT 10

# 3. Review error messages
SELECT id, error_message, error_stack FROM job_logs WHERE status = 'failed'

# 4. Check job-specific issues
# For payouts: verify Stellar network connectivity
# For emails: verify SMTP/SendGrid credentials
# For webhooks: check endpoint availability
```

### Issue: Queue backup (too many waiting jobs)

```bash
# 1. Increase concurrency
// jobs.constants.ts - increase concurrency for relevant queue

# 2. Check processor performance
// Review job duration metrics
SELECT AVG(duration_ms) FROM job_logs WHERE queue_name = 'email'

# 3. Add more workers
// Deploy additional job worker instances

# 4. Reduce priority-queue contention
// Review job priority distribution
SELECT job_type, COUNT(*) FROM job_logs WHERE STATUS = 'pending' GROUP BY job_type
```

### Issue: Memory leak in worker

```bash
# 1. Monitor memory usage
docker stats

# 2. Check for event listener leaks
// Ensure all listeners are properly cleaned up

# 3. Review processor implementations
// Ensure streams and connections are closed

# 4. Restart worker container
docker-compose restart job-worker
```

### Issue: Retry loop (job keeps failing)

```bash
# 1. Analyze failure pattern
SELECT attempt, error_message, created_at FROM job_logs 
WHERE id = 'job-failing'
ORDER BY created_at DESC

# 2. Common causes:
# - Invalid payload data
# - External service unavailable
# - Permissions/authentication issues

# 3. Solutions:
# - Fix payload data
# - Wait for service recovery
# - Update credentials
# - Manually cancel and recreate job
```

## Related Documentation

- [Stellar Integration Guide](../stellar/STELLAR_INTEGRATION.md)
- [Multi-Sig Wallet System](../stellar/MULTISIG_IMPLEMENTATION.md)
- [API Versioning](../config/API_VERSIONING.md)
- [Error Handling](../common/ERROR_HANDLING.md)
- [Database Schema](../database/SCHEMA.md)
