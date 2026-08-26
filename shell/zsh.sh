#!/bin/zsh
_nlsh_bind() {
    local cmd
    cmd=$(nlsh gen "$BUFFER")
    if [ $? -eq 0 ]; then
        BUFFER="$cmd"
        CURSOR=${#BUFFER}
    fi
}
zle -N _nlsh_bind
bindkey '^G' _nlsh_bind
