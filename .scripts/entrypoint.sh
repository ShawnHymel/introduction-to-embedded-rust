#!/bin/bash

echo "Welcome to the Introduction to Embedded Rust development environment!"

# Start the Rust Book server
echo "Starting the Rust Book server..."
cd /home/$USER/rust-book
nohup mdbook serve --hostname 0.0.0.0 --port 3000 > /tmp/mdbook.log 2>&1 &
echo "Rust Book server started. You can access it at http://localhost:3000"
echo "To stop the server, run 'kill $(pgrep -f mdbook)'"

# Container is ready
echo "Development environment is ready!"

# Keep container running with interactive bash
cd /home/$USER
exec /bin/bash
