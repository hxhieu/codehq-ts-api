# `.env` file

```
CODEHQ_TS_API_CORS_ALLOWED_ORIGINS="http://localhost:3000"
CODEHQ_TS_API_CORS_ALLOWED_METHODS="GET,POST"
CODEHQ_TS_API_AUTH_ISSUER=https://login.microsoftonline.com/TENANT_ID_HERE/v2.0
CODEHQ_TS_API_AUTH_CLIENT_ID=APP_REGISTRATION_ID
CODEHQ_TS_API_AUTH_ALLOWED_DOMAINS=ALLOWED_DOMAINS_CSV
CODEHQ_TS_TIMESHEET_DSN='sqlserver://USER:PASS@HOST?database=DB&parseTime=true'
CODEHQ_TS_PIMP_DSN='sqlserver://USER:PASS@HOST?database=DB&parseTime=true'
# Optional but give you the debug logs
RUST_LOG=info
RUST_BACKTRACE=1
```

Or it will fallback to the machine env vars if no `.env` file found.

Ultimately it will crash if nothing found :)

# Development

## SSL lib is required

- On Linux: follow your distro instructions
- On Windows: [OpenSSL binaries](https://slproweb.com/products/Win32OpenSSL.html)
