# Timeline Fixture

This fixture tests the timeline component.

```yaml
type: timeline
orientation: vertical
steps:
  - timestamp: "2026-05-18 10:00"
    title: "Initial Outage"
    type: "critical"
  - timestamp: "2026-05-18 10:15"
    title: "Rolled back to v1.2"
    type: "recovery"
  - timestamp: "2026-05-18 10:30"
    title: "Monitoring restored"
    type: "info"
```

Some trailing paragraph.
