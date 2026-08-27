#!/bin/zsh
_kbind_bind() {
    local cmd
    cmd=$(kb gen "$BUFFER")
    if [ $? -eq 0 ]; then
        BUFFER="$cmd"
        CURSOR=${#BUFFER}
    fi
}
zle -N _kbind_bind
bindkey '^G' _kbind_bind
