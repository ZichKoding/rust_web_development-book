#!/bin/bash

# Show the current status
echo "Checking the current status of the repository..."
git status

# Add all changes to staging
echo "Adding all changes to staging..."
git add .

# Get the commit message from the user
read -p "Enter the commit message: " commit_message

# Commit the changes with the provided message
echo "Committing the changes..."
git commit -m "$commit_message"

# Show the status again
echo "Checking the status after commit..."
git status

# Get the current branch
branch=$(git branch --show-current)

# Confirm that the user wants to push the changes to this branch
read -p "Do you want to push the changes to the current branch ($branch)? (y/n): " confirm

if [ "$confirm" != "y" ]; then
    echo "Exiting without pushing the changes..."
    exit 1
fi

# Push the changes to the current branch
echo "Pushing the changes to the current branch..."
git push

# Show the final status
echo "Final status of the repository..."
git status

echo "Done!"