# Create table in DUCKDB

```SQL
create table speed( lat float, long float, vehicle integer, speed float, time timestamp );
```

# Run connector

# Testing

```SQL
 select vehicle, avg(speed) from speed  group by vehicle;
```