#!/bin/bash

# Login to Firebase CLI (optional, not required for local emulators)
# firebase login:ci --no-localhost

# Initialize Firebase project (if needed)
# firebase init

# Start the Firebase Authentication emulator
firebase init
firebase emulators:start --only auth --project meal-planer-dev --import=./emulator-data --export-on-exit=./emulator-data