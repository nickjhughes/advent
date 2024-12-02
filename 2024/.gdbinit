cd /
set confirm off
set pagination off
set non-stop off
handle SIGPIPE nostop noprint pass
shell make
file app
shell tmux split-window
shell echo "tty $(tmux list-panes -F '#{pane_index} #{pane_tty}' | awk -v target=1 '$1 == target {print $2}')" > /tmp/gf-cmd-pane-tty
source /tmp/gf-cmd-pane-tty
r &
set scheduler-locking step
