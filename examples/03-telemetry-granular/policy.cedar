permit(
    principal,
    action,
    resource
) when {
    action == Action::"NetworkConnect" || action == Action::"WriteStdOut"
};

forbid(
    principal,
    action,
    resource
) when {
    action == Action::"FileSystemRead" || action == Action::"FileSystemWrite"
};
