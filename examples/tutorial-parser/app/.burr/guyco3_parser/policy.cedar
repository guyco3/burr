// Default burr policy: Deny all actions
permit(
    principal == Burr::Module::"guest-module",
    action == Burr::Action::"env_read",
    resource
) when {
    context.key == "DEBUG_MODE"
};
