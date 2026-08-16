// Allow TCP requests only to the official telemetry backend
permit(
    principal == User::"guest",
    action == Action::"socket_connect",
    resource
) when {
    context.ip == "10.0.0.5"
};

// Explicitly forbid exfiltration via TCP to unknown IPs
forbid(
    principal == User::"guest",
    action == Action::"socket_connect",
    resource
);
