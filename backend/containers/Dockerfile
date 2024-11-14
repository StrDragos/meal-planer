# Use an official Node.js runtime as a parent image
FROM node:18

# Set the working directory in the container
WORKDIR /usr/src/app

COPY firebase.json .

# Install the Firebase CLI globally
RUN npm install -g firebase-tools
RUN firebase init

# Expose the ports for the Firebase emulator
# Port 9099 for the Authentication emulator
EXPOSE 9099

# Copy a script that will initialize and start the Firebase emulator
COPY ./start-emulators.sh .

# Make the script executable
RUN chmod +x start-emulators.sh

# Command to run the Firebase emulator
CMD ["./start-emulators.sh"]