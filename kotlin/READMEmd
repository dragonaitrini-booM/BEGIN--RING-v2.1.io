# 🚀 Ktor Backend Setup & Local Execution

This guide details how to prepare your environment and run the Ktor server locally using Gradle.

---

## 1. Prerequisites and Environment Variables

Your application uses Ktor's configuration system to securely load secrets from your shell environment. **You MUST set these variables before running the application.**

### Required Variables

| Variable | Description | Example Value |
|---|---|---|
| `SB_SERVICE_KEY` | Supabase Service Role Key (CRITICAL: Backend-only secret) | `export SB_SERVICE_KEY="secret-key-1234"` |
| `CF_TOKEN` | Cloudflare API Token | `export CF_TOKEN="cf-token-abc"` |
| `GH_PAT` | GitHub Personal Access Token | `export GH_PAT="ghp_pat_xyz"` |
| `GAS_KEY` | Google Apps Script Service Account Key | `export GAS_KEY="gas-key-5678"` |
| `SB_ANON_KEY` | Supabase Anon/Public Key | `export SB_ANON_KEY="anon-key-9012"` |
| `SB_URL` | Supabase Project URL | `export SB_URL="https://your-project.supabase.co"` |

### Command Example (Linux/macOS)

```bash
# Set all required environment variables in your current shell session
export SB_SERVICE_KEY="<YOUR_SUPABASE_SERVICE_ROLE_KEY>"
export CF_TOKEN="<YOUR_CLOUDFLARE_API_TOKEN>"
export GH_PAT="<YOUR_GITHUB_PAT>"
export GAS_KEY="<YOUR_GAS_KEY>"
export SB_ANON_KEY="<YOUR_SUPABASE_ANON_KEY>"
export SB_URL="<YOUR_SUPABASE_URL>"

# Verify the environment variable is set (optional)
echo "Supabase URL: $SB_URL"
