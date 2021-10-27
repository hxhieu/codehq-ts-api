# `.env` file

```
CODEHQ_TS_API_AUTH_ISSUER=https://login.microsoftonline.com/TENANT_ID_HERE/v2.0
CODEHQ_TS_API_AUTH_CLIENT_ID=APP_REGISTRATION_ID
CODEHQ_TS_TIMESHEET_DSN='sqlserver://USER:PASS@HOST?database=DB&parseTime=true'
CODEHQ_TS_PIMP_DSN='sqlserver://USER:PASS@HOST?database=DB&parseTime=true'
```

Or it will fallback to the machine env vars if no `.env` file found.

Ultimately it will crash if nothing found :)
