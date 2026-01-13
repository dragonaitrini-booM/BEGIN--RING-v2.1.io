#!/bin/bash
# This script sets the 'tor-list' text record for the 'dragontools.eth' ENS name.
# It assumes that 'dragontools.eth' is registered and has a resolver configured.

# Replace with the actual private key for the owner of 'dragontools.eth'
ADMIN_KEY="0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"

# The ENS name we are configuring
ENS_NAME="dragontools.eth"

# The text record key and value
RECORD_KEY="tor-list"
RECORD_VALUE="sha256:cafed00d..."

# Public RPC endpoint (replace if you have a private one)
RPC_URL="https://eth.merkle.io"

# ENS Registry address
REGISTRY_ADDR="0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e"

echo "Fetching resolver for $ENS_NAME..."

# Get the namehash of the ENS name
NAMEHASH=$(cast namehash $ENS_NAME)

# Get the resolver address from the ENS registry
RESOLVER_ADDR=$(cast call $REGISTRY_ADDR "resolver(bytes32)" $NAMEHASH --rpc-url $RPC_URL)

if [ "$RESOLVER_ADDR" == "0x0000000000000000000000000000000000000000" ]; then
  echo "Error: No resolver found for $ENS_NAME. Please configure a resolver first."
  exit 1
fi

echo "Resolver found at: $RESOLVER_ADDR"
echo "Setting text record '$RECORD_KEY' to '$RECORD_VALUE'..."

# Send the transaction to set the text record
cast send $RESOLVER_ADDR "setText(bytes32,string,string)" $NAMEHASH "$RECORD_KEY" "$RECORD_VALUE" --private-key $ADMIN_KEY --rpc-url $RPC_URL

echo "Transaction sent. The ENS record for the Tor exit node list has been updated. ✅"
