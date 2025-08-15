#!/bin/bash

echo "Welcome to the Introduction to Embedded Rust development environment!"

# Start the Rust Book server
cd /home/$USER/rust-book
nohup mdbook serve --hostname 0.0.0.0 --port 3000 > /tmp/mdbook.log 2>&1 &
echo "Rust Book server started. You can access it at http://localhost:3000"
echo "To stop the server, run 'kill $(pgrep -f mdbook)'"
echo ""

# Container is ready
echo "To start rustlings exercises:"
echo "  cd /home/$USER/workspace"
echo "  rustlings init"
echo "  cd /home/$USER/workspace/rustlings"
echo "  rustlings"
echo ""

# Note for opening workspace in VS Code
echo "If you are in VS Code, please click File > Open Workspace from File..."
echo "and then select /home/student/workspace/default.code-workspace."
echo ""

# Keep container running with interactive bash
cd /home/$USER
exec /bin/bash
