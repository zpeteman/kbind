#!/bin/bash
_kbind_bind() {
    local cmd
    cmd=$(kb gen "$READLINE_LINE")
    if [ $? -eq 0 ]; then
        READLINE_LINE="$cmd"
        READLINE_POINT=${#READLINE_LINE}
    fi
}
bind -x '"\C-g": _kbind_bind'
