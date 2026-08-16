// Only allow reading files inside the dedicated scratch directory
permit(
    principal == User::"guest",
    action == Action::"fs_read",
    resource
) when {
    context.path like "/app/uploads/scratch/*"
};

// Forbid all raw socket connections (force use of safe HTTP APIs if needed)
forbid(
    principal == User::"guest",
    action == Action::"socket_connect",
    resource
);
