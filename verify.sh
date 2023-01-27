#!/bin/bash
# Arg 1 = path to .so file
# Arg 2 = program id of the contract to verify
# Arg 3 = RPC url (uses the default config if not supplied)

URL=$(solana config get | head -n 2 | tail -n 1 | awk '{print $3}')

if [[ "$3" != "" ]]; then
    URL=$3
fi

EXECUTABLE_SIZE=$(wc -c $1 | awk '{print $1}')
solana program dump -u $URL $2 tmp.so &> /dev/null
HASH=$(shasum -a 256 <(head -c $EXECUTABLE_SIZE tmp.so) | awk '{print $1}')
rm tmp.so
RESULT=$(shasum -a 256 -c <(echo "$HASH  $1") | head -n 1 | awk '{print $2}')
if [ "$RESULT" = "OK" ]; then
    echo "Program verified, hash=$HASH"
    exit 0
else
    echo "Program verification failed"
    exit 1
fi