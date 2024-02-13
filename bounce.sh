#!/bin/bash

# the width of the terminal
TERMINAL_WIDTH=$(tput cols)

# init position of ball
position=0

# direction of ball movement {1: right, -1: left}
direction=1

# ascii char used to draw ball
ball="0"

# clear the screen
# clear

# animate the ball bouncing
while true; do
  # move cursor to beginning of line
  tput cr
  # clear line ... else 0 will trail when moving left
  tput el
  # calculate spaces to position ball @frame
  printf '%*s' "$position" ' ' 
  echo -n "$ball"

  # calculate ball position and direction @next_frame
  if [ $position -eq $((TERMINAL_WIDTH-1)) ]; then
    direction=-1
  elif [ $position -eq 0 ]; then
    direction=1
  fi
  position=$((position + direction))

  # time till @next_frame update - effectively ball speed
  sleep 0.0005
done
