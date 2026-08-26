function _nlsh_bind
    set -l cmd (nlsh gen (commandline -b))
    if test $status -eq 0
        commandline -r $cmd
    end
end
bind \cg _nlsh_bind
