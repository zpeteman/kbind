function _kbind_bind
    set -l cmd (kb gen (commandline -b))
    if test $status -eq 0
        commandline -r $cmd
    end
end
bind \cg _kbind_bind
