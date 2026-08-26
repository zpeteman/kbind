#!/bin/bash
_nlsh_bind() {
    local cmd
    cmd=$(nlsh gen "$READLINE_LINE")
    if [ $? -eq 0 ]; then
        READLINE_LINE="$cmd"
        READLINE_POINT=${#READLINE_LINE}
    fi
}
bind -x '"\C-g": _nlsh_bind'
